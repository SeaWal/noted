
use note::Note;

#[derive(Debug, Default)]
pub struct AppState {
    pub should_quit: bool,
    pub notes: Vec<Note>,
    pub current_note: usize,
    pub current_view: usize,
}

impl AppState {
    
    // construct new AppState
    pub fn new() -> Self {
        Self::default()
    }

    // handle event tick
    pub fn tick(&mut self) {

    }

    // switch between editor/homescreen
    pub fn toggle_view(&mut self) {
        
    }

    // add new note to list
    pub fn insert_note(&mut self, note: Note) {

    }

    // delete a note
    pub fn delete_note(&mut self, note: Note) {

    }
    
    // quit out app
    pub fn quit(&mut self) {
        self.should_quit = true
    }
}