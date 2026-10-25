use crate::models::JournalEntry;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub logged_in: bool,
    pub username: String,
    pub token: String,
    pub entries: Vec<JournalEntry>, 
}

impl AppState {
    pub fn add_entry(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            logged_in: false,
            username: String::new(),
            token: String::new(),
            entries: Vec::new(), // <-- Initialize entries
        }
    }
}