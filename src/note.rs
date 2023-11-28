use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// TODO: make this configurable from app.rs
pub const DB_PATH: &str = "./notes/notes.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    id: usize,
    title: String,
    content: String,
    created_at: DateTime<Utc>
}

impl Note {

    pub fn new() -> Self {
        Note {
            id: 0,
            title: "".into(),
            content: "".into(),
            created_at: Utc::now()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteList {
    notes: Vec<Note>,
}

impl NoteList {

    pub fn new() -> Self {
        NoteList {
            notes: Vec::new()
        }
    }

    pub fn insert(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn remove(&mut self, note: Note) -> Option<Note> {
        self.notes.pop()
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
        let note1 = Note::new();
        let note2 = Note::new();

        let mut note_list = NoteList::new();
        note_list.insert(note1);
        note_list.insert(note2);

        assert_eq!(note_list.length(), 2);
    }

    #[test]
    fn test_notelist_is_empty() {

    }
}