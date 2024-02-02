use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Alignment, Frame};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState};

use crate::app::{AppState, CurrentView};
use crate::note::NoteList;

pub fn render(app: &mut AppState, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(frame.size());

    match app.current_view {
        CurrentView::Main => {
            let mut idx = TableState::default();
            idx.select(Some(app.current_note));
            let list = render_notes(&mut app.notes);
            frame.render_stateful_widget(list, layout[0], &mut idx);
        }
        CurrentView::Editing => {
            frame.render_widget(app.textbox.clone(), layout[0]);
        }
    }

    let nav_hints = render_nav(app);
    frame.render_widget(nav_hints, layout[1]);

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title(app.textbox.cursor_pos.to_string())
                    .title_alignment(Alignment::Center), // .borders(Borders::ALL)
                                                         // .border_type(BorderType::Rounded),
            )
            .style(Style::default())
            .alignment(Alignment::Center),
        frame.size(),
    );
}

fn render_notes(note_list: &mut NoteList) -> Table<'_> {
    let rows: Vec<Row> = note_list
        .iter()
        .enumerate()
        .map(|(id, note)| {
            Row::new(vec![
                Cell::from(Span::from(id.to_string())),
                Cell::from(Span::from(note.title.clone())),
                Cell::from(Span::from(note.created_at.to_string())),
            ])
        })
        .collect();

    let col_names = Row::new(vec![
        Cell::from("ID").add_modifier(Modifier::BOLD),
        Cell::from("Title").add_modifier(Modifier::BOLD),
        Cell::from("Created At").add_modifier(Modifier::BOLD),
    ]);

    let table = Table::new(rows.into_iter())
        .header(col_names)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(BorderType::Plain),
        )
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ]);

    table
}

fn render_nav(app: &mut AppState) -> Paragraph<'_> {
    let nav_hints = {
        match app.current_view {
            CurrentView::Main => Span::styled("(q/Esc) to quit", Style::default()),

            CurrentView::Editing => Span::styled("(Esc) to quit", Style::default()),
        }
    };

    Paragraph::new(Line::from(nav_hints))
}

fn build2(input_text: &str, cursor_pos: usize) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut offset = 0;
    for line in input_text.lines() {
        let mut spans = Vec::new();
        for (index, ch) in line.char_indices() {
            let style = if offset + index == cursor_pos {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            spans.push(Span::styled(ch.to_string(), style));
        }
        spans.push(Span::styled('\n'.to_string(), Style::default()));
        offset += line.len();
        lines.push(Line::from(spans))
    }

    lines
}
