use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// TODO: make this configurable from app.rs
pub const DB_PATH: &str = "./notes/notes.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    id: usize,
    title: String,
    content: String,
    created_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NoteList {
    notes: Vec<Note>,
    size: usize,
}

impl NoteList {

    pub fn new() -> Self {
        NoteList {
            notes: Vec::new(),
            size: 0,
        }
    }

    pub fn insert(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn remove(&mut self, note: Note) -> Option<Note> {
        self.notes.pop(note)
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn length(&self) -> usize {
        self.notes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_inserted() {

    }

    #[test]
    fn test_note_deleted() {

    }

    #[test]
    fn test_notelist_length() {

    }

    #[test]
    fn test_notelist_is_empty() {
        
    }
}