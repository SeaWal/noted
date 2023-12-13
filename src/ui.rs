use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Alignment, Frame};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table};

use crate::app::{AppState, CurrentView};
use crate::note::{self, NoteList};

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
    frame.render_widget(title, layout[0]);

    let mut list_items = Vec::<ListItem>::new();
    for note in app.notes.iter() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <10} Hello", note.id.to_string()),
            Style::default().fg(Color::Green),
        ))))
    }

    let list = List::new(list_items);
    frame.render_widget(list, layout[1]);

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

fn render_notes<'a>(note_list: &mut NoteList) -> Table<'a> {
    let notes = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    let details: Vec<_> = note_list
        .iter()
        .map(|note| {
            Row::new(vec![
                Cell::from(Span::raw(note.id.into())),
                Cell::from(Span::raw(note.title)),
                Cell::from(Span::raw(note.created_at.into())),
            ])
        })
        .collect();

    let table = Table::new(details);

    table
}
