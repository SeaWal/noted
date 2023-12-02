use crate::note::{Note, NoteList};

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
}

impl AppState {
    // construct new AppState
    pub fn new() -> Self {
        AppState {
            should_quit: false,
            notes: NoteList::new(),
            current_note: 0,
            current_view: CurrentView::Main,
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
}
