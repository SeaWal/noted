pub mod app;
pub mod event;
pub mod note;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;

use crate::note::Note;
// use app::AppState;
// use event::{EventHandler, EventType};
// use ratatui::{backend::CrosstermBackend, Terminal};
// use tui::Tui;
// use update::update;

fn main() -> Result<()> {
    let json_content = r#"{ "id": 0, "title" : "sample", "content" : "sample", "created_at" : "2023-12-01T15:30:45" }"#;
    println!("{}", json_content);
    
    let note = match serde_json::from_str::<Note>(json_content) {
        Ok(note) => println!("success"),
        Err(err) => eprintln!("[ERROR]: {}",err),
    };

    Ok(note)
    // let mut app = AppState::new();

    // let backend = CrosstermBackend::new(std::io::stderr());
    // let terminal = Terminal::new(backend)?;
    // let event_handler = EventHandler::new(250);
    // let mut tui = Tui::new(terminal, event_handler);
    // tui.enter()?;

    // while !app.should_quit {
    //     tui.draw(&mut app)?;

    //     match tui.events.next()? {
    //         EventType::Tick => {}
    //         EventType::Key(key_event) => update(&mut app, key_event),
    //         EventType::Mouse(_) => {}
    //         EventType::Resize(_, _) => {}
    //     };
    // }

    // tui.exit()?;
    // Ok(())
}
