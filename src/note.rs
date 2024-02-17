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
    pub title: String,
    pub content: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.title, self.created_at)
    }
}

impl Note {
    pub fn new(title: &str, content: Vec<String>) -> Self {
        Note {
            title: String::from(title),
            content: content,
            created_at: Utc::now(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title)
    }

    pub fn set_content(&mut self, content: Vec<String>) {
        self.content = content
    }

    pub fn update(&mut self, title: &str, content: Vec<String>) {
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
        self.notes.push(note.clone());
    }

    pub fn remove(&mut self, id: usize) -> Option<Note> {
        if id < self.notes.len() {
            Some(self.notes.remove(id))
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

    pub fn save(&self, file_path: &str) -> Result<()> {
        let file = File::create(file_path)?;
        let _ = serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn load(file_path: &str) -> Result<Self> {
        let mut file = File::open(file_path).expect("File does not exist");
        let mut json_string = String::new();
        let _ = file.read_to_string(&mut json_string).expect("Couldn't read from JSON");
        let note_list: NoteList = serde_json::from_str(&json_string).expect("Couldn't load JSON into NoteList");
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

    pub fn get(&mut self, id: usize) -> Option<&mut Note> {
        self.notes.get_mut(id)
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_note_inserted() {
        let mut note_list = NoteList::new();
        note_list.insert(&Note::new("", Vec::new()));

        assert_eq!(note_list.length(), 1);
    }

    #[test]
    fn test_note_deleted() {
        let note = Note {
            title: "title".into(),
            content: vec!["This".into(), "is".into(), "the".into(), "content".into()],
            created_at: Utc::now(),
        };

        let mut note_list = NoteList::new();
        note_list.insert(&note);
        note_list.remove(0);
        assert_eq!(note_list.length(), 0)
    }

    #[test]
    fn test_notelist_length() {
        let note1 = Note::new("", Vec::new());
        let note2 = Note::new("", Vec::new());

        let mut note_list = NoteList::new();
        note_list.insert(&note1);
        note_list.insert(&note2);

        assert_eq!(note_list.length(), 2);
    }


    #[test]
    fn test_write_notelist_to_json() {
        let mut note_list = NoteList::new();
        note_list.insert(&Note::new("title1", vec!["content1".to_string()]));
        note_list.insert(&Note::new("title2", vec!["content2".to_string()]));

        let file_path = "test.json";
        let result = note_list.save(file_path);
        assert!(result.is_ok());
        assert!(fs::metadata(file_path).is_ok());
    }

    #[test]
    fn test_load_notelist_from_json() {
        let test_file = "test.json";
        let mut nl = NoteList::new();
        nl.insert(&Note::new("title1", vec!["content1".into()]));
        nl.insert(&Note::new("title2", vec!["content2".into()]));

        let _ = nl.save(test_file);

        let result = NoteList::load(test_file);
        // assert!(result.is_ok());

        assert_eq!(nl, result.unwrap());
        fs::remove_file(test_file).unwrap();
        // let note_list = result.unwrap();
        // assert_eq!(note_list.length(), 2);
    }
}
