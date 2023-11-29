use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Block, Borders, BorderType};
use ratatui::prelude::{Alignment, Frame};

use crate::app::AppState;

pub fn render(_app: &mut AppState, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(
            "Render notes here..."
        )
        .block(
            Block::default()
            .title("Noted")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        frame.size(),
    );
}