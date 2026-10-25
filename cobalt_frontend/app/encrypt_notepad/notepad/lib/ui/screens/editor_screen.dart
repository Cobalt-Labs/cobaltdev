import 'dart:async';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_ui/bridge_generated.dart/frb_generated.dart' as bridge;
import 'package:flutter_ui/ui/screens/bloc/notepad_bloc.dart';
import 'package:flutter_ui/ui/screens/bloc/notepad_event.dart';
import 'package:flutter_ui/ui/screens/bloc/notepad_state.dart';
import 'package:permission_handler/permission_handler.dart';

class EditorScreen extends StatefulWidget {
  final String initialTitle;
  final String initialContent;

  const EditorScreen({
    super.key,
    this.initialTitle = '',
    this.initialContent = '',
  });

  @override
  State<EditorScreen> createState() => _EditorScreenState();
}

class _EditorScreenState extends State<EditorScreen> {
  late TextEditingController _titleController;
  late TextEditingController _contentController;
  bool _isLoading = false;
  bool _isEncrypted = false;
  bool _hasChanges = false;
  bool _isSaving = false;
  final bool _autoSaveEnabled = true;
  late FocusNode _titleFocus;
  late FocusNode _contentFocus;
  Timer? _autoSaveTimer;
  DateTime? _lastSaved;

  int get wordCount =>
      _contentController.text
          .trim()
          .split(RegExp(r'\s+'))
          .where((word) => word.isNotEmpty)
          .length;

  int get charCount => _contentController.text.length;

  int get charCountNoSpaces =>
      _contentController.text.replaceAll(' ', '').length;

  @override
  void initState() {
    super.initState();
    _titleController = TextEditingController(text: widget.initialTitle);
    _contentController = TextEditingController(text: widget.initialContent);
    _titleFocus = FocusNode();
    _contentFocus = FocusNode();

    _titleController.addListener(_onTextChanged);
    _contentController.addListener(_onTextChanged);

    if (widget.initialTitle.isEmpty) {
      Future.microtask(() => _titleFocus.requestFocus());
    }

    _startAutoSave();
  }

  void _onTextChanged() {
    final titleChanged = _titleController.text != widget.initialTitle;
    final contentChanged = _contentController.text != widget.initialContent;
    if ((titleChanged || contentChanged) != _hasChanges) {
      setState(() {
        _hasChanges = titleChanged || contentChanged;
      });
    }
  }

  void _startAutoSave() {
    if (!_autoSaveEnabled) return;

    _autoSaveTimer?.cancel();
    _autoSaveTimer = Timer.periodic(const Duration(seconds: 30), (timer) {
      if (_hasChanges && !_isSaving && _canAutoSave()) {
        _performAutoSave();
      }
    });
  }

  bool _canAutoSave() {
    return _titleController.text.trim().isNotEmpty &&
        _contentController.text.trim().isNotEmpty;
  }

  Future<void> _performAutoSave() async {
    if (_isSaving) return;

    setState(() => _isSaving = true);

    try {
      final bloc = context.read<NotepadBloc>();
      bloc.add(
        SaveNoteEvent(
          title: _titleController.text.trim(),
          content: _contentController.text.trim(),
        ),
      );

      setState(() {
        _lastSaved = DateTime.now();
      });
      _showToast('Auto-saved');
    } catch (e) {
      debugPrint('Auto-save failed: $e');
    } finally {
      setState(() => _isSaving = false);
    }
  }

  void _showToast(String message) {
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(message),
          duration: const Duration(seconds: 1),
          behavior: SnackBarBehavior.floating,
          backgroundColor: Colors.green.shade700,
        ),
      );
    }
  }

  Future<void> _toggleEncryption() async {
    if (_isEncrypted) {
      setState(() => _isLoading = true);
      try {
        final decryptedContent = await bridge.RustLib.instance.api
            .crateApiDecryptText(encryptedText: _contentController.text);
        setState(() {
          _contentController.text = decryptedContent;
          _isEncrypted = false;
        });
      } catch (e) {
        _showError('Decryption failed: ${e.toString()}');
      } finally {
        setState(() => _isLoading = false);
      }
    } else {
      setState(() => _isLoading = true);
      try {
        final encryptedContent = await bridge.RustLib.instance.api
            .crateApiEncryptText(text: _contentController.text);
        setState(() {
          _contentController.text = encryptedContent;
          _isEncrypted = true;
        });
      } catch (e) {
        _showError('Encryption failed: ${e.toString()}');
      } finally {
        setState(() => _isLoading = false);
      }
    }
  }

  Future<void> requestStoragePermission() async {
    if (Platform.isAndroid) {
      var status = await Permission.storage.status;
      if (!status.isGranted) {
        status = await Permission.storage.request();
        if (!status.isGranted) {
          _showError('Storage permission is required to save notes.');
        }
      }
    }
  }

  void _showError(String message) {
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(message),
          backgroundColor: Colors.red,
          behavior: SnackBarBehavior.floating,
        ),
      );
    }
  }

  Future<void> _saveNote() async {
    final title = _titleController.text.trim();
    final content = _contentController.text.trim();

    if (title.isEmpty) {
      _showError('Title cannot be empty');
      _titleFocus.requestFocus();
      return;
    }
    if (content.isEmpty) {
      _showError('Content cannot be empty');
      _contentFocus.requestFocus();
      return;
    }

    if (Platform.isAndroid) {
      await requestStoragePermission();
    }

    final bloc = context.read<NotepadBloc>();
    bloc.add(SaveNoteEvent(title: title, content: content));
  }

  Future<bool> _onWillPop() async {
    if (!_hasChanges) return true;
    final result = await showDialog<bool>(
      context: context,
      builder:
          (context) => AlertDialog(
            title: const Text('Discard changes?'),
            content: const Text(
              'You have unsaved changes. Are you sure you want to discard them?',
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(false),
                child: const Text('Cancel'),
              ),
              ElevatedButton(
                onPressed: () => Navigator.of(context).pop(true),
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.redAccent,
                ),
                child: const Text('Discard'),
              ),
            ],
          ),
    );
    return result ?? false;
  }

  void _showStatsDialog() {
    showDialog(
      context: context,
      builder:
          (context) => AlertDialog(
            title: const Text('Document Statistics'),
            content: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Words: $wordCount'),
                Text('Characters: $charCount'),
                Text('Characters (no spaces): $charCountNoSpaces'),
                if (_lastSaved != null)
                  Text('Last saved: ${_formatTime(_lastSaved!)}'),
              ],
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(),
                child: const Text('Close'),
              ),
            ],
          ),
    );
  }

  String _formatTime(DateTime time) {
    final now = DateTime.now();
    final difference = now.difference(time);

    if (difference.inMinutes < 1) {
      return 'Just now';
    } else if (difference.inHours < 1) {
      return '${difference.inMinutes} minutes ago';
    } else if (difference.inDays < 1) {
      return '${difference.inHours} hours ago';
    } else {
      return '${difference.inDays} days ago';
    }
  }

  @override
  void dispose() {
    _autoSaveTimer?.cancel();
    _titleController.removeListener(_onTextChanged);
    _contentController.removeListener(_onTextChanged);
    _titleController.dispose();
    _contentController.dispose();
    _titleFocus.dispose();
    _contentFocus.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocListener<NotepadBloc, NotepadState>(
      listener: (context, state) {
        if (state is NotepadLoaded) {
          _showToast('Note saved successfully');
          setState(() {
            _hasChanges = false;
            _isSaving = false;
            _lastSaved = DateTime.now();
          });
          if (widget.initialTitle.isEmpty) {
            Navigator.of(context).pop(true);
          }
        } else if (state is NotepadError) {
          _showError('Failed to save note: ${state.message}');
          setState(() {
            _isSaving = false;
          });
        } else if (state is NotepadLoading) {
          setState(() {
            _isSaving = true;
          });
        }
      },
      child: PopScope(
        canPop: false,
        onPopInvoked: (didPop) async {
          if (didPop) return;
          final result = await _onWillPop();
          if (result && context.mounted) {
            Navigator.of(context).pop();
          }
        },
        child: Scaffold(
          appBar: AppBar(
            title: Text(widget.initialTitle.isEmpty ? 'New Note' : 'Edit Note'),
            actions: [
              IconButton(
                icon: const Icon(Icons.info_outline),
                onPressed: _showStatsDialog,
                tooltip: 'Statistics',
              ),
              IconButton(
                icon: Icon(_isEncrypted ? Icons.lock : Icons.lock_open),
                onPressed: _isLoading ? null : _toggleEncryption,
                tooltip: _isEncrypted ? 'Decrypt' : 'Encrypt',
              ),
              AnimatedSwitcher(
                duration: const Duration(milliseconds: 200),
                child:
                    _hasChanges
                        ? IconButton(
                          key: const ValueKey('save'),
                          icon:
                              _isSaving
                                  ? const SizedBox(
                                    width: 24,
                                    height: 24,
                                    child: CircularProgressIndicator(
                                      strokeWidth: 2,
                                    ),
                                  )
                                  : const Icon(Icons.save),
                          onPressed:
                              (_isLoading || _isSaving) ? null : _saveNote,
                          tooltip: 'Save',
                        )
                        : const SizedBox(width: 48),
              ),
            ],
          ),
          body:
              _isLoading
                  ? const Center(child: CircularProgressIndicator())
                  : GestureDetector(
                    onTap: () {
                      FocusScope.of(context).unfocus();
                    },
                    child: Column(
                      children: [
                        Container(
                          width: double.infinity,
                          padding: const EdgeInsets.symmetric(
                            horizontal: 16,
                            vertical: 8,
                          ),
                          color: Theme.of(
                            context,
                          ).colorScheme.surface.withOpacity(0.3),
                          child: Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Text(
                                'Words: $wordCount • Characters: $charCount',
                                style: Theme.of(context).textTheme.bodySmall,
                              ),
                              if (_lastSaved != null && _autoSaveEnabled)
                                Text(
                                  'Auto-saved ${_formatTime(_lastSaved!)}',
                                  style: Theme.of(context).textTheme.bodySmall
                                      ?.copyWith(color: Colors.green),
                                ),
                            ],
                          ),
                        ),
                        Expanded(
                          child: Padding(
                            padding: const EdgeInsets.all(16.0),
                            child: Column(
                              children: [
                                Container(
                                  decoration: BoxDecoration(
                                    borderRadius: BorderRadius.circular(12),
                                    color:
                                        Theme.of(context).colorScheme.surface,
                                  ),
                                  padding: const EdgeInsets.symmetric(
                                    horizontal: 16,
                                    vertical: 8,
                                  ),
                                  child: TextField(
                                    controller: _titleController,
                                    focusNode: _titleFocus,
                                    decoration: const InputDecoration(
                                      hintText: 'Note Title',
                                      border: InputBorder.none,
                                      counter: SizedBox.shrink(),
                                    ),
                                    style: const TextStyle(
                                      fontSize: 20,
                                      fontWeight: FontWeight.bold,
                                    ),
                                    maxLength: 50,
                                  ),
                                ),
                                const SizedBox(height: 16),
                                Expanded(
                                  child: Container(
                                    decoration: BoxDecoration(
                                      borderRadius: BorderRadius.circular(12),
                                      color:
                                          Theme.of(context).colorScheme.surface,
                                    ),
                                    padding: const EdgeInsets.all(16),
                                    child: TextField(
                                      controller: _contentController,
                                      focusNode: _contentFocus,
                                      maxLines: null,
                                      expands: true,
                                      decoration: const InputDecoration(
                                        hintText: 'Write your note here...',
                                        border: InputBorder.none,
                                      ),
                                      style: const TextStyle(
                                        fontSize: 16,
                                        height: 1.5,
                                      ),
                                    ),
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
          floatingActionButton: AnimatedOpacity(
            opacity: _hasChanges ? 1.0 : 0.0,
            duration: const Duration(milliseconds: 200),
            child:
                _hasChanges
                    ? FloatingActionButton(
                      onPressed: (_isLoading || _isSaving) ? null : _saveNote,
                      tooltip: 'Save Note',
                      child:
                          _isSaving
                              ? const SizedBox(
                                width: 24,
                                height: 24,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                  color: Colors.white,
                                ),
                              )
                              : const Icon(Icons.save),
                    )
                    : const SizedBox.shrink(),
          ),
        ),
      ),
    );
  }
}
