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
            cursor_pos: 0,
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

    pub fn inc_cursor(&mut self) {
        if self.cursor_pos == self.input_text.len() - 1 {
            return;
        }
        self.cursor_pos += 1;
    }

    pub fn dec_cursor(&mut self) {
        if self.cursor_pos != 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn inc_line(&mut self) {
        self.cursor_pos = match find_next_newline(self.input_text.clone(), self.cursor_pos) {
            Some(index) => index + 1,
            None => self.cursor_pos,
        }
    }

    pub fn dec_line(&mut self) {
        self.cursor_pos = match find_prev_newline(self.input_text.clone(), self.cursor_pos) {
            Some(index) => index,
            None => self.cursor_pos,
        }
    }
}

fn find_next_newline(input: String, start_index: usize) -> Option<usize> {
    for (index, ch) in input.char_indices().skip(start_index) {
        if ch == '\n' {
            return Some(index);
        }
    }
    None
}

fn find_prev_newline(input: String, start_index: usize) -> Option<usize> {
    for (index, ch) in input
        .char_indices()
        .rev()
        .skip_while(|&(i, _)| i > start_index-1)
    {
        if ch == '\n' {
            return Some(index-1);
        }
    }
    None
}
