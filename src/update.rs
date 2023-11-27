use crossterm::event::{KeyCode, KeyEvent};

use crate::app::AppState;

// TODO: remove comments after implementing
// TODO: move this to app.rs 
// TODO: make part of AppState impementation?
pub fn update(app: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        // close the progrem
        KeyCode::Esc | KeyCode::Char('q')=> {},
        // on home screen, create/open a new note
        KeyCode::Char('n') => {},
        // if highlighting a note, open the editor
        KeyCode::Enter => {},
        // if editing a note, save it
        KeyCode::Char('s') => {}
        // navigate up/down list of notes
        KeyCode::Up => {}
        KeyCode::Down => {}

        // default case
        _ => {}
    }
}