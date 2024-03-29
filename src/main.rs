pub mod app;
pub mod event;
pub mod note;
pub mod textbox;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;
use app::AppState;
use crossterm::terminal;
use event::{EventHandler, EventType};

use note::NoteList;
use ratatui::{backend::CrosstermBackend, Terminal};

use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let event_handler = EventHandler::new(250);
    let mut tui = Tui::new(terminal, event_handler);

    let height = terminal::size()
        .map(|(_, height)| height)
        .expect("Couldn't open terminal") as usize;
    
    let mut app = AppState::new(height);
    app.notes = NoteList::load("./notes/test.json").unwrap_or(NoteList::new());
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            EventType::Tick => {}
            EventType::Key(key_event) => update(&mut app, key_event),
            EventType::Mouse(_) => {}
            EventType::Resize(_, _) => {}
        };
    }

    tui.exit()?;

    Ok(())
}
