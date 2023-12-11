use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Alignment, Frame};
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Text, Line};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::app::{AppState, CurrentView};

pub fn render(app: &mut AppState, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Noted", Style::default())).block(title_block);
    frame.render_widget(title, layout[1]);

    let nav_hints = {
        match app.current_view {
            CurrentView::Main => Span::styled("(q/Esc) to quit", Style::default().fg(Color::Red)),

            CurrentView::Editing => {
                Span::styled("(q/Esc) to quit", Style::default().fg(Color::Blue))
            }
        }
    };

    frame.render_widget(Paragraph::new(Line::from(nav_hints)), layout[2]);

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
