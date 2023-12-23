use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{AppState, CurrentView},
    note::Note,
};

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
                KeyCode::Char('n') => {
                    let note = Note::new("", "");
                    app.notes.insert(&note);

                    app.current_view = CurrentView::Editing
                }

                KeyCode::Enter => app.current_view = CurrentView::Editing,

                // navigate up/down list of notes
                KeyCode::Up => {
                    if app.current_note == 0 {
                        app.current_note = 0
                    } else {
                        app.current_note -= 1
                    }
                }
                KeyCode::Down => {
                    if app.current_note >= app.notes.length() {}
                    else { app.current_note += 1 }
                }

                // default case
                _ => {}
            }
        }
        CurrentView::Editing => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.save();
                app.quit()
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.save();
                    app.quit()
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.set_current_note()
                }
            }

            KeyCode::Char(value) => {
                app.input_text.push(value);
            }

            KeyCode::Backspace => {
                let _ = app.input_text.pop();
            }

            KeyCode::Enter => app.input_text.push('\n'),

            _ => {}
        },
    }
}
