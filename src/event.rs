use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug, Clone, Copy)]
pub enum EventType {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}
