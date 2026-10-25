import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_ui/ui/screens/bloc/notepad_bloc.dart';
import 'package:flutter_ui/ui/screens/bloc/notepad_event.dart';
import 'package:flutter_ui/ui/screens/bloc/notepad_state.dart';

import 'editor_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;
  final TextEditingController _searchController = TextEditingController();
  bool _isSearching = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      duration: const Duration(milliseconds: 300),
      vsync: this,
    );
    _animation = CurvedAnimation(parent: _controller, curve: Curves.easeInOut);
    _controller.forward();

    context.read<NotepadBloc>().add(LoadNotesEvent());

    _searchController.addListener(_onSearchChanged);
  }

  @override
  void dispose() {
    _controller.dispose();
    _searchController.dispose();
    super.dispose();
  }

  void _onSearchChanged() {
    context.read<NotepadBloc>().add(
      SearchNotesEvent(query: _searchController.text),
    );
  }

  void _toggleSearch() {
    setState(() {
      _isSearching = !_isSearching;
      if (!_isSearching) {
        _searchController.clear();
        context.read<NotepadBloc>().add(SearchNotesEvent(query: ''));
      }
    });
  }

  void _refreshNotes() {
    context.read<NotepadBloc>().add(LoadNotesEvent());
  }

  void _navigateToEditor() async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder:
            (context) =>
                const EditorScreen(initialTitle: '', initialContent: ''),
      ),
    );
    if (result == true) {
      _refreshNotes();
    }
  }

  Future<void> _openNote(String noteTitle) async {
    try {
      final notepadBloc = context.read<NotepadBloc>();
      final content = await notepadBloc.loadNoteContent(noteTitle);

      if (mounted) {
        final result = await Navigator.push(
          context,
          MaterialPageRoute(
            builder:
                (context) => EditorScreen(
                  initialTitle: noteTitle,
                  initialContent: content,
                ),
          ),
        );
        if (result == true) {
          _refreshNotes();
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Error loading note: $e'),
            backgroundColor: Colors.redAccent,
            behavior: SnackBarBehavior.floating,
          ),
        );
      }
    }
  }

  void _deleteNote(String noteTitle) {
    context.read<NotepadBloc>().add(DeleteNoteEvent(title: noteTitle));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title:
            _isSearching
                ? TextField(
                  controller: _searchController,
                  autofocus: true,
                  decoration: const InputDecoration(
                    hintText: 'Search notes...',
                    border: InputBorder.none,
                    hintStyle: TextStyle(color: Colors.white70),
                  ),
                  style: const TextStyle(color: Colors.white),
                )
                : const Text(
                  'Encrypted Notes',
                  style: TextStyle(letterSpacing: 0.5),
                ),
        elevation: 0,
        actions: [
          IconButton(
            icon: Icon(_isSearching ? Icons.close : Icons.search),
            onPressed: _toggleSearch,
            tooltip: _isSearching ? 'Close Search' : 'Search Notes',
          ),
          if (!_isSearching)
            IconButton(
              icon: const Icon(Icons.refresh),
              onPressed: _refreshNotes,
              tooltip: 'Refresh Notes',
            ),
        ],
      ),
      body: AnimatedBuilder(
        animation: _animation,
        builder: (context, child) {
          return FadeTransition(opacity: _animation, child: child);
        },
        child: Column(
          children: [
            if (!_isSearching)
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Hero(
                  tag: 'create_button',
                  child: ElevatedButton.icon(
                    onPressed: _navigateToEditor,
                    style: ElevatedButton.styleFrom(
                      minimumSize: const Size.fromHeight(50),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                    ),
                    icon: const Icon(Icons.add),
                    label: const Text(
                      'Create New Note..',
                      style: TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                        letterSpacing: 1.2,
                      ),
                    ),
                  ),
                ),
              ),
            Expanded(
              child: BlocConsumer<NotepadBloc, NotepadState>(
                listener: (context, state) {
                  if (state is NotepadError) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(
                        content: Text(state.message),
                        backgroundColor: Colors.redAccent,
                        behavior: SnackBarBehavior.floating,
                      ),
                    );
                  }
                },
                builder: (context, state) {
                  if (state is NotepadLoading) {
                    return const Center(child: CircularProgressIndicator());
                  } else if (state is NotepadError) {
                    return Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          const Icon(
                            Icons.error_outline,
                            size: 48,
                            color: Colors.redAccent,
                          ),
                          const SizedBox(height: 16),
                          Text(
                            'Error loading notes',
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                          const SizedBox(height: 8),
                          Text(state.message),
                          const SizedBox(height: 16),
                          ElevatedButton(
                            onPressed: _refreshNotes,
                            child: const Text('Retry'),
                          ),
                        ],
                      ),
                    );
                  } else if (state is NotepadLoaded) {
                    final notes = state.notes;

                    if (notes.isEmpty) {
                      return Center(
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            Icon(
                              _isSearching
                                  ? Icons.search_off
                                  : Icons.note_alt_outlined,
                              size: 80,
                              color: Theme.of(
                                context,
                              ).colorScheme.secondary.withOpacity(0.5),
                            ),
                            const SizedBox(height: 16),
                            Text(
                              _isSearching ? 'No notes found' : 'No notes yet',
                              style: Theme.of(context).textTheme.titleLarge,
                            ),
                            const SizedBox(height: 8),
                            Text(
                              _isSearching
                                  ? 'Try a different search term'
                                  : 'Create your first encrypted note!',
                              style: TextStyle(
                                color: Theme.of(
                                  context,
                                ).colorScheme.onSurface.withOpacity(0.7),
                              ),
                            ),
                          ],
                        ),
                      );
                    }

                    return ListView.builder(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 16,
                        vertical: 8,
                      ),
                      itemCount: notes.length,
                      itemBuilder: (context, index) {
                        final noteTitle = notes[index];
                        return Dismissible(
                          key: Key(noteTitle),
                          direction: DismissDirection.endToStart,
                          background: Container(
                            alignment: Alignment.centerRight,
                            padding: const EdgeInsets.only(right: 20.0),
                            color: Colors.redAccent,
                            child: const Column(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Icon(
                                  Icons.delete_forever,
                                  color: Colors.white,
                                  size: 28,
                                ),
                                SizedBox(height: 4),
                                Text(
                                  "Delete",
                                  style: TextStyle(
                                    color: Colors.white,
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                              ],
                            ),
                          ),
                          dismissThresholds: const {
                            DismissDirection.endToStart: 0.6,
                          },
                          confirmDismiss: (direction) async {
                            return await showDialog<bool>(
                              context: context,
                              builder: (BuildContext context) {
                                return AlertDialog(
                                  title: Text('Delete "$noteTitle"?'),
                                  content: const Text(
                                    'This action cannot be undone. The note will be deleted permanently.',
                                  ),
                                  actions: [
                                    TextButton(
                                      onPressed:
                                          () =>
                                              Navigator.of(context).pop(false),
                                      child: const Text('Cancel'),
                                    ),
                                    ElevatedButton(
                                      onPressed:
                                          () => Navigator.of(context).pop(true),
                                      style: ElevatedButton.styleFrom(
                                        backgroundColor: Colors.redAccent,
                                      ),
                                      child: const Text('Delete'),
                                    ),
                                  ],
                                );
                              },
                            );
                          },
                          onDismissed: (direction) {
                            _deleteNote(noteTitle);
                            ScaffoldMessenger.of(context).showSnackBar(
                              SnackBar(
                                content: Text('$noteTitle deleted'),
                                behavior: SnackBarBehavior.floating,
                                action: SnackBarAction(
                                  label: 'Undo',
                                  onPressed: () {
                                    _refreshNotes();
                                  },
                                ),
                              ),
                            );
                          },
                          child: Card(
                            margin: const EdgeInsets.only(bottom: 12),
                            clipBehavior: Clip.antiAlias,
                            elevation: 2,
                            child: InkWell(
                              onTap: () => _openNote(noteTitle),
                              child: Padding(
                                padding: const EdgeInsets.all(16.0),
                                child: Row(
                                  children: [
                                    const Icon(Icons.note_rounded),
                                    const SizedBox(width: 12),
                                    Expanded(
                                      child: Text(
                                        noteTitle,
                                        style: const TextStyle(
                                          fontSize: 18,
                                          fontWeight: FontWeight.w500,
                                        ),
                                      ),
                                    ),
                                    const Icon(Icons.lock, size: 20),
                                  ],
                                ),
                              ),
                            ),
                          ),
                        );
                      },
                    );
                  }

                  return const Center(child: CircularProgressIndicator());
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}
