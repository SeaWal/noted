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

    let list = render_notes2(&mut app.notes);
    frame.render_widget(list, layout[1]);

    let nav_hints = {
        match app.current_view {
            CurrentView::Main => Span::styled("((q/Esc) to quit", Style::default().fg(Color::Red)),

            CurrentView::Editing => {
                Span::styled("((q/Esc) to quit", Style::default().fg(Color::Blue))
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

fn render_notes<'a>(note_list: &mut NoteList) -> List<'a> {
    let items: Vec<ListItem> = note_list
        .iter()
        .map(|note| ListItem::new(format!("{} {} {}", note.id, note.title, note.created_at)))
        .collect();

    let list = List::new(items).block(Block::default().title("Notes").borders(Borders::ALL));

    list
}

fn render_notes2(note_list: &mut NoteList) -> Table<'_> {
    let rows: Vec<Row> = note_list
        .iter()
        .map(|note| {
            Row::new(vec![
                Cell::from(Span::raw(note.id.to_string())),
                Cell::from(Span::raw(note.title.clone())),
                Cell::from("hello")
            ])
        })
        .collect();

    let col_names = Row::new(vec![Cell::from("ID"), Cell::from("Title")]);

    let table = Table::new(rows.into_iter()).header(col_names).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    );

    table
}
