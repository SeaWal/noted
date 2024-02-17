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
                    let note = Note::new("", Vec::new());
                    app.notes.insert(&note);

                    app.current_view = CurrentView::Editing
                }

                KeyCode::Enter => {
                    app.textbox.text = match app.notes.get(app.current_note) {
                        Some(note) => note.clone().content,
                        None => Vec::new(),
                    };
                    app.current_view = CurrentView::Editing
                }

                // navigate up/down list of notes
                KeyCode::Up => {
                    if app.current_note == 0 {
                        app.current_note = 0
                    } else {
                        app.current_note -= 1
                    }
                }
                KeyCode::Down => {
                    if app.current_note == app.notes.length() - 1 {
                    } else {
                        app.current_note += 1
                    }
                }

                // default case
                _ => {}
            }
        }
        CurrentView::Editing => match key_event.code {
            KeyCode::Esc => {
                let _ = app.save();
                app.current_view = CurrentView::Main
            }
            KeyCode::Char(value) => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    match value {
                        's' | 'S' => app.set_current_note(),
                        'c' | 'C' => {
                            let _ = app.save();
                            app.quit()
                        }
                        _ => {}
                    }
                } else {
                    app.textbox.handle_input(key_event.code)
                }
            }
            _ => {
                app.textbox.handle_input(key_event.code);
            }
        },
    }
}
