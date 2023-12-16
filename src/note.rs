use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::Read,
};

// TODO: make this configurable from app.rs
pub const DB_PATH: &str = "../notes/notes.json";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Note {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.id, self.title)
    }
}

impl Note {
    pub fn new(id: usize, title: &str, content: &str) -> Self {
        Note {
            id,
            title: String::from(title),
            content: String::from(content),
            created_at: Utc::now(),
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title)
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content)
    }

    pub fn update(&mut self, title: &str, content: &str) {
        self.set_title(title);
        self.set_content(content);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NoteList {
    notes: Vec<Note>,
}

impl NoteList {
    pub fn new() -> Self {
        NoteList { notes: Vec::new() }
    }

    pub fn insert(&mut self, note: &Note) {
        let note_id = match self.max_note_id() {
            Some(note_id) => note_id + 1,
            None => 1,
        };

        let mut note = note.clone();
        note.set_id(note_id);

        self.notes.push(note);
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

    pub fn save(&self, file_path: &str) -> Result<()> {
        let file = File::create(file_path)?;
        let _ = serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn load(file_path: &str) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut json_string = String::new();
        let _ = file.read_to_string(&mut json_string)?;
        let note_list: NoteList = serde_json::from_str(&json_string)?;
        Ok(note_list)
    }

    pub fn print_notes(&self) {
        for note in &self.notes {
            println!("{}", note);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Note> {
        self.notes.iter()
    }

    pub fn get(&self, id: usize) -> Option<&Note> {
        self.notes.iter().find(|&note| note.id == id)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_note_inserted() {
        let mut note_list = NoteList::new();
        note_list.insert(&Note::new(0, "", ""));

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
        let note1 = Note::new(0, "", "");
        let note2 = Note::new(1, "", "");

        let mut note_list = NoteList::new();
        note_list.insert(&note1);
        note_list.insert(&note2);

        assert_eq!(note_list.length(), 2);
    }

    #[test]
    fn test_max_note_id() {
        let mut note_list = NoteList::new();

        let note1 = Note::new(0, "", "");
        let note2 = Note::new(0, "", "");
        note_list.insert(&note1);
        note_list.insert(&note2);

        let max_id = note_list.max_note_id().unwrap();

        assert_eq!(max_id, 2);
    }

    #[test]
    fn test_load_notelist_from_json() {
        let test_file = "test.json";
        let mut nl = NoteList::new();
        nl.insert(&Note::new(0, "title1", "content1"));
        nl.insert(&Note::new(1, "title2", "content2"));
        let file = File::create(test_file).unwrap();
        let _ = serde_json::to_writer_pretty(file, &nl);

        let result = NoteList::load(test_file);
        assert!(result.is_ok());

        let note_list = result.unwrap();
        assert_eq!(note_list.length(), 2);
    }

    #[test]
    fn test_write_notelist_to_json() {
        let mut note_list = NoteList::new();
        note_list.insert(&Note::new(0, "title1", "content1"));
        note_list.insert(&Note::new(1, "title2", "content2"));

        let file_path = "test.json";
        let result = note_list.save(file_path);
        assert!(result.is_ok());

        let parsed_result = NoteList::load(file_path);
        assert!(parsed_result.is_ok());
        let parsed_nl = parsed_result.unwrap();
        assert_eq!(note_list, parsed_nl);
    }
}
