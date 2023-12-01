use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: make this configurable from app.rs
pub const DB_PATH: &str = "./notes/notes.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    id: usize,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
}

impl Note {
    pub fn new() -> Self {
        Note {
            id: 0,
            title: "".into(),
            content: "".into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteList {
    notes: Vec<Note>,
}

impl NoteList {
    pub fn new() -> Self {
        NoteList { notes: Vec::new() }
    }

    pub fn insert(&mut self, note: &Note) {
        self.notes.push(note.clone());
    }

    pub fn remove(&mut self, id: usize) -> Option<Note> {
        if let Some(index) = self.notes.iter().position(|note| note.id == id) {
            Some(self.notes.remove(index))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn length(&self) -> usize {
        self.notes.len()
    }

    pub fn max_note_id(&self) -> Option<usize> {
        if let Some(max_id) = self.notes.iter().map(|note| note.id).max() {
            Some(max_id)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_note_inserted() {
        let mut note_list = NoteList::new();
        note_list.insert(&Note::new());

        assert_eq!(note_list.length(), 1);
    }

    #[test]
    fn test_note_deleted() {
        let note = Note {
            id: 1,
            title: "title".into(),
            content: "content".into(),
            created_at: Utc::now(),
        };

        let mut note_list = NoteList::new();
        note_list.insert(&note);
        note_list.remove(note.id);

        assert_eq!(note_list.length(), 0)
    }

    #[test]
    fn test_notelist_length() {
        let note1 = Note::new();
        let note2 = Note::new();

        let mut note_list = NoteList::new();
        note_list.insert(&note1);
        note_list.insert(&note2);

        assert_eq!(note_list.length(), 2);
    }
}
