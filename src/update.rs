use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{AppState, CurrentView};

pub fn update(app: &mut AppState, key_event: KeyEvent) {
    match app.current_view {
        CurrentView::Main => {
            match key_event.code {
                // close the program
                KeyCode::Esc | KeyCode::Char('q') => app.quit(),

                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit()
                    }
                }
                // on home screen, create/open a note
                KeyCode::Char('n') | KeyCode::Enter => app.current_view = CurrentView::Editing,

                // navigate up/down list of notes
                KeyCode::Up => {
                    if app.current_note == 0 {
                        app.current_note = 0
                    } else {
                        app.current_note -= 1
                    }
                }
                KeyCode::Down => {
                    if app.current_note == app.notes.max_note_id().unwrap() {
                        app.current_note = app.current_note
                    } else {
                        app.current_note += 1
                    }
                }

                // default case
                _ => {}
            }
        }
        CurrentView::Editing => match key_event.code {
            _ => {}
        },
    }
}
