abstract class NotepadState {}

class NotepadInitial extends NotepadState {}

class NotepadLoading extends NotepadState {}

class NotepadLoaded extends NotepadState {
  final List<String> notes;
  
  NotepadLoaded({required this.notes});
}

class NotepadError extends NotepadState {
  final String message;
  
  NotepadError({required this.message});
}