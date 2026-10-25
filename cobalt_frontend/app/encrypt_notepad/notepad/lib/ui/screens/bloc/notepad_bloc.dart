import 'dart:io';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as path;
import 'package:flutter_ui/bridge_generated.dart/frb_generated.dart';
import 'notepad_event.dart';
import 'notepad_state.dart';

class NotepadBloc extends Bloc<NotepadEvent, NotepadState> {
  NotepadBloc() : super(NotepadInitial()) {
    on<LoadNotesEvent>(_onLoadNotes);
    on<SaveNoteEvent>(_onSaveNote);
    on<DeleteNoteEvent>(_onDeleteNote);
    on<SearchNotesEvent>(_onSearchNotes);
  }

  List<String> _allNotes = [];
  String _currentSearchQuery = '';

  Future<void> _onLoadNotes(
    LoadNotesEvent event,
    Emitter<NotepadState> emit,
  ) async {
    emit(NotepadLoading());

    try {
      List<String> allTitles = [];

      try {
        final titlesString =
            await RustLib.instance.api.crateApiListNoteTitles();
        if (titlesString.isNotEmpty) {
          allTitles.addAll(titlesString.split(';'));
        }
      } catch (e) {
        print("Error fetching Rust notes: $e");
      }

      if (Platform.isAndroid && allTitles.isEmpty) {
        try {
          final directory = await getApplicationDocumentsDirectory();
          final files =
              directory
                  .listSync()
                  .where((file) => file.path.endsWith('.txt'))
                  .map((file) => path.basenameWithoutExtension(file.path))
                  .toList();
          allTitles.addAll(files);
        } catch (e) {
          print("Error fetching Flutter notes: $e");
        }
      }

      _allNotes = allTitles;

      final filteredNotes =
          _currentSearchQuery.isEmpty
              ? _allNotes
              : _allNotes
                  .where(
                    (note) => note.toLowerCase().contains(
                      _currentSearchQuery.toLowerCase(),
                    ),
                  )
                  .toList();

      emit(NotepadLoaded(notes: filteredNotes));
    } catch (e) {
      emit(NotepadError(message: "Failed to load notes: ${e.toString()}"));
    }
  }

  Future<void> _onSaveNote(
    SaveNoteEvent event,
    Emitter<NotepadState> emit,
  ) async {
    final currentState = state;
    emit(NotepadLoading());

    try {
      final title = event.title.trim();
      final content = event.content.trim();

      if (title.isEmpty) {
        emit(NotepadError(message: "Title cannot be empty"));
        return;
      }

      if (content.isEmpty) {
        emit(NotepadError(message: "Content cannot be empty"));
        return;
      }

      bool saved = false;

      if (Platform.isAndroid) {
        try {
          final directory = await getApplicationDocumentsDirectory();
          final fileName = '${title.replaceAll(RegExp(r'[^\w\s]+'), '')}.txt';
          final filePath = '${directory.path}/$fileName';
          final file = File(filePath);
          await file.writeAsString(content);
          saved = true;
        } catch (e) {
          print("Flutter save failed: $e");
        }
      }

      if (!saved) {
        try {
          saved = await RustLib.instance.api.crateApiSaveNoteToDisk(
            title: title,
            content: content,
          );
        } catch (e) {
          print("Rust save failed: $e");
        }
      }

      if (saved) {
        add(LoadNotesEvent());
      } else {
        emit(NotepadError(message: "Failed to save note"));
        if (currentState is NotepadLoaded) {
          emit(currentState);
        }
      }
    } catch (e) {
      emit(NotepadError(message: "Error saving note: ${e.toString()}"));
      if (currentState is NotepadLoaded) {
        emit(currentState);
      }
    }
  }

  Future<void> _onDeleteNote(
    DeleteNoteEvent event,
    Emitter<NotepadState> emit,
  ) async {
    final currentState = state;
    emit(NotepadLoading());

    try {
      bool deleted = false;

      try {
        await RustLib.instance.api.crateApiDeleteNoteFromDisk(
          title: event.title,
        );
        deleted = true;
      } catch (e) {
        print("Rust delete failed: $e");
      }

      if (!deleted && Platform.isAndroid) {
        try {
          final directory = await getApplicationDocumentsDirectory();
          final fileName =
              '${event.title.replaceAll(RegExp(r'[^\w\s]+'), '')}.txt';
          final filePath = '${directory.path}/$fileName';
          final file = File(filePath);
          if (await file.exists()) {
            await file.delete();
            deleted = true;
          }
        } catch (e) {
          print("Flutter delete failed: $e");
        }
      }

      if (deleted) {
        _allNotes.remove(event.title);
        add(LoadNotesEvent());
      } else {
        emit(NotepadError(message: "Failed to delete note: ${event.title}"));
        if (currentState is NotepadLoaded) {
          emit(currentState);
        }
      }
    } catch (e) {
      emit(NotepadError(message: "Error deleting note: ${e.toString()}"));
      if (currentState is NotepadLoaded) {
        emit(currentState);
      }
    }
  }

  Future<void> _onSearchNotes(
    SearchNotesEvent event,
    Emitter<NotepadState> emit,
  ) async {
    _currentSearchQuery = event.query;

    if (_allNotes.isEmpty) {
      add(LoadNotesEvent());
      return;
    }

    final filteredNotes =
        event.query.isEmpty
            ? _allNotes
            : _allNotes
                .where(
                  (note) =>
                      note.toLowerCase().contains(event.query.toLowerCase()),
                )
                .toList();

    emit(NotepadLoaded(notes: filteredNotes));
  }

  Future<String> loadNoteContent(String title) async {
    try {
      try {
        return await RustLib.instance.api.crateApiLoadNoteFromDisk(
          title: title,
        );
      } catch (e) {
        print("Rust load failed: $e");
      }

      if (Platform.isAndroid) {
        final directory = await getApplicationDocumentsDirectory();
        final fileName = '${title.replaceAll(RegExp(r'[^\w\s]+'), '')}.txt';
        final filePath = '${directory.path}/$fileName';
        final file = File(filePath);
        if (await file.exists()) {
          return await file.readAsString();
        }
      }

      throw Exception("Note not found: $title");
    } catch (e) {
      throw Exception("Error loading note: ${e.toString()}");
    }
  }

  Future<String> encryptText(String text) async {
    try {
      return await RustLib.instance.api.crateApiEncryptText(text: text);
    } catch (e) {
      throw Exception("Encryption failed: ${e.toString()}");
    }
  }

  Future<String> decryptText(String encryptedText) async {
    try {
      return await RustLib.instance.api.crateApiDecryptText(
        encryptedText: encryptedText,
      );
    } catch (e) {
      throw Exception("Decryption failed: ${e.toString()}");
    }
  }
}
