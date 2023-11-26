
use note::Note;

#[derive(Debug, Default)]
pub struct AppState {
    pub should_quit: bool,
    pub notes: Vec<Note>,

}