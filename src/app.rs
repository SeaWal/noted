use crate::note::{Note, NoteList};
use anyhow::Result;

#[derive(Debug)]
pub enum CurrentView {
    Main,
    Editing,
}
#[derive(Debug)]
pub struct AppState {
    pub should_quit: bool,
    pub notes: NoteList,
    pub current_note: usize,
    pub current_view: CurrentView,
    pub input_text: String,
    pub save_file: String,
    pub cursor_pos: usize,
}

impl AppState {
    // construct new AppState
    pub fn new() -> Self {
        AppState {
            should_quit: false,
            notes: NoteList::new(),
            current_note: 0,
            current_view: CurrentView::Main,
            input_text: String::new(),
            save_file: String::from("./notes/test.json"),
            cursor_pos: 0
        }
    }

    // handle event tick
    pub fn tick(&mut self) {}

    // switch between editor/homescreen
    pub fn toggle_view(&mut self) {}

    // add new note to list
    pub fn insert_note(&mut self, _note: Note) {}

    // delete a note
    pub fn delete_note(&mut self, _note: Note) {}

    // quit out app
    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn save(&self) -> Result<()> {
        self.notes.save(self.save_file.as_str())
    }

    pub fn set_current_note(&mut self) {
        match self.notes.get(self.current_note) {
            Some(note) => note.set_content(&self.input_text),
            None => {}
        }
    }
}
