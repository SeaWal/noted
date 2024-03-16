use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{AppState, CurrentView},
    note::Note,
};

pub fn update(app: &mut AppState, key_event: KeyEvent) {
    match app.current_view {
        CurrentView::Main => {
            if !app.editing_title {
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
                        app.current_note = app.notes.length() - 1;
                        app.editing_title = true
                    }

                    KeyCode::Char('d') => {
                        app.notes.remove(app.current_note);
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
            // else, editing title -> switch focus to nav
            else {
                match key_event.code {
                    KeyCode::Char(ch) => {
                        app.title_buf.push(ch);
                    }
                    KeyCode::Backspace => {
                        app.title_buf.pop();
                    }
                    KeyCode::Esc => {
                        app.editing_title = false;
                        app.title_buf.clear();
                    }
                    KeyCode::Enter => {
                        let note = app.notes.get(app.current_note).expect("Couldn't open note.");
                        note.set_title(&app.title_buf);
                        app.editing_title = false;
                        app.current_view = CurrentView::Editing;
                        app.textbox.text = match app.notes.get(app.current_note) {
                            Some(note) => note.clone().content,
                            None => Vec::new(),
                        };
                    }
                    _ => {}
                }
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
                    app.textbox
                        .handle_input(key_event.code, key_event.modifiers)
                }
            }
            _ => {
                app.textbox
                    .handle_input(key_event.code, key_event.modifiers);
            }
        },
    }
}
