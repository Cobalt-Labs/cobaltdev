abstract class NotepadEvent {}

class LoadNotesEvent extends NotepadEvent {}

class SaveNoteEvent extends NotepadEvent {
  final String title;
  final String content;
  
  SaveNoteEvent({required this.title, required this.content});
}

class DeleteNoteEvent extends NotepadEvent {
  final String title;
  
  DeleteNoteEvent({required this.title});
}

class SearchNotesEvent extends NotepadEvent {
  final String query;
  
  SearchNotesEvent({required this.query});
}