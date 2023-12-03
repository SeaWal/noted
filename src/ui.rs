use ratatui::prelude::{Alignment, Frame};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::app::{AppState, CurrentView};

pub fn render(app: &mut AppState, frame: &mut Frame) {
    match app.current_view {
        CurrentView::Main => frame.render_widget(
            Paragraph::new("Render notes here...")
                .block(
                    Block::default()
                        .title("Noted")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            frame.size(),
        ),
        CurrentView::Editing => {}
    }
}
