use crate::{
    note::{Note, NoteList},
    textbox::TextBox,
};
use anyhow::Result;
// use crossterm::event::KeyCode;

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
    pub save_file: String,
    pub textbox: TextBox,
    pub editing_title: bool,
    pub title_buf: String,
    pub terminal_height: usize,
}

impl AppState {
    // construct new AppState
    pub fn new(terminal_height: usize) -> Self {
        AppState {
            should_quit: false,
            notes: NoteList::new(),
            current_note: 0,
            current_view: CurrentView::Main,
            save_file: String::from("./notes/test.json"),
            textbox: TextBox::new(terminal_height),
            editing_title: false,
            title_buf: String::new(),
            terminal_height: terminal_height,
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
            Some(note) => note.set_content(self.textbox.text.clone()),
            None => {}
        }
    }
}
