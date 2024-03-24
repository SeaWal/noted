use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use std::cmp::min;

const HEIGHT_PADDING: usize = 3;

#[derive(Clone, Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    pub latch_col: usize,
}

#[derive(Clone, Debug)]
pub struct TextBox {
    pub text: Vec<String>,
    pub cursor: Cursor,
    pub visible_lines: (usize, usize),
    pub terminal_height: usize,
}

impl From<Vec<String>> for TextBox {
    fn from(v: Vec<String>) -> Self {
        let term_height = crossterm::terminal::size()
            .map(|(_, height)| height as usize)
            .unwrap_or_default(); // - HEIGHT_PADDING;
        Self {
            text: v,
            cursor: Cursor {
                row: 0,
                col: 0,
                latch_col: 0,
            },
            visible_lines: (0, term_height),
            terminal_height: term_height,
        }
    }
}

impl TextBox {
    pub fn new(terminal_height: usize) -> Self {
        TextBox {
            text: Vec::new(),
            cursor: Cursor {
                row: 0,
                col: 0,
                latch_col: 0,
            },
            visible_lines: (0, terminal_height),// -HEIGHT_PADDING),
            terminal_height: terminal_height,// -HEIGHT_PADDING,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) {
        match key {
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Enter => self.insert_newline(),
            KeyCode::Char(ch) => self.insert_char(ch),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Tab => self.move_cursor_next_word(),
            KeyCode::BackTab => self.move_cursor_prev_word(),
            _ => {}
        }
        self.update_visible_lines();
    }

    fn move_cursor_right(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        let row_len = if self.text[row].is_empty() {
            0
        } else {
            self.text[row].chars().count()
        };

        if col == row_len && row < self.text.len() - 1 {
            self.cursor.col = 0;
            self.cursor.row = row + 1;
        } else if col < row_len {
            self.cursor.col = col + 1;
        }

        self.cursor.latch_col = self.cursor.col
    }

    fn move_cursor_left(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);

        if col == 0 && row > 0 {
            let prev_row_len = if self.text[row - 1].is_empty() {
                0
            } else {
                self.text[row - 1].chars().count()
            };
            self.cursor.row = row - 1;
            self.cursor.col = prev_row_len;
        } else if col > 0 {
            self.cursor.col = col - 1;
        }

        self.cursor.latch_col = self.cursor.col
    }

    fn move_cursor_down(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        if row < self.text.len() - 1 {
            self.cursor.row = row + 1;
            let next_line_len = self.text[row + 1].chars().count();
            if col > next_line_len {
                self.cursor.col = next_line_len
            }
            self.cursor.col = min(next_line_len, self.cursor.latch_col)
        }
    }

    fn move_cursor_up(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);

        if row > 0 {
            self.cursor.row = row - 1;
            let prev_line_len = self.text[row - 1].chars().count();
            if col > prev_line_len {
                self.cursor.col = prev_line_len
            }
            self.cursor.col = min(prev_line_len, self.cursor.latch_col)
        }
    }

    fn insert_char(&mut self, ch: char) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        if self.text.is_empty() {
            self.text.push(String::new());
        }

        let curr_line = &mut self.text[row];
        curr_line.insert(col, ch);

        if col == curr_line.char_indices().count() {
            curr_line.insert(col + 1, ' ');
        }
        self.move_cursor_right()
    }

    fn insert_newline(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        let line = &mut self.text[row];

        // we could be in the middle of a line of text
        let pos_in_line = line
            .char_indices()
            .nth(col)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let newline: String = line[pos_in_line..].to_string();
        line.truncate(pos_in_line);
        self.text.insert(row + 1, newline);
        self.cursor.row = row.saturating_add(1);
        self.cursor.col = 0;
    }

    fn delete_char(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        if col == 0 {
            self.delete_line();
        } else {
            let line = &mut self.text[row];
            line.remove(col - 1);
            self.cursor.col = col.saturating_sub(1);
        }
    }

    fn delete_line(&mut self) {
        let (row, _) = (self.cursor.row, self.cursor.col);
        if row == 0 {
            return;
        }

        let curr_line = self.text.remove(row);
        let prev_line = &mut self.text[row - 1];

        self.cursor.row = row.saturating_sub(1);
        self.cursor.col = prev_line.chars().count();

        prev_line.push_str(&curr_line);
    }

    fn move_cursor_next_word(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        let line = &self.text[row];

        fn next_word_start(line: &String, init_pos: usize) -> Option<usize> {
            for (i, ch) in line.chars().enumerate().skip(init_pos) {
                if ch.is_whitespace() {
                    return Some(i);
                }
            }
            None
        }

        match next_word_start(line, col + 1) {
            Some(col) => self.cursor.col = col,
            None if row < self.text.len() - 1 => {
                self.cursor.row = row + 1;
                self.cursor.col = 0;
            }
            None => self.cursor.col = line.len(),
        }
    }

    fn move_cursor_prev_word(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        let line = &self.text[row];

        fn prev_word_end(line: &String, init_pos: usize) -> Option<usize> {
            for (i, ch) in line.chars().rev().enumerate().skip(line.len() - init_pos) {
                if ch.is_whitespace() {
                    return Some(line.len() - i - 1);
                }
            }
            None
        }

        match prev_word_end(line, col) {
            Some(col) => self.cursor.col = col,
            None if row > 0 => {
                self.cursor.row = row - 1;
                self.cursor.col = self.text[row - 1].len();
            }
            None => self.cursor.col = 0,
        }
    }

    fn update_visible_lines(&mut self) {
        let row = self.cursor.row;
        let available_height = self.terminal_height - HEIGHT_PADDING - 1;
        let start = if row >= available_height {
            row.saturating_sub(available_height) + 1
        } else {
            0
        };
        let end = min(self.text.len(), start + available_height);

        self.visible_lines = (start, end);
    }

    pub fn reset(&mut self) {
        self.text.clear();
        self.cursor.row = 0;
        self.cursor.col = 0;
    }
}

fn line_into_spans(line: &str) -> Vec<Span> {
    let mut spans = Vec::new();

    for ch in line.chars() {
        let span = Span::styled(ch.to_string(), Style::default());
        spans.push(span);
    }

    spans
}

fn cursor_line_into_spans(line: &str, cursor_pos: usize) -> Vec<Span> {
    let mut spans = Vec::new();

    for (i, ch) in line.chars().enumerate() {
        let style = if i == cursor_pos {
            Style::default().bg(Color::Gray).fg(Color::Black)
        } else {
            Style::default()
        };
        let span = Span::styled(ch.to_string(), style);
        spans.push(span);
    }

    if cursor_pos >= line.len() {
        spans.push(Span::styled(
            "N".to_string(),
            Style::default().bg(Color::Gray).fg(Color::Gray),
        ));
    }

    spans
}

impl Widget for TextBox {
    fn render(mut self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        self.update_visible_lines();
        let (start, end) = self.visible_lines;
        let visible_text = &self.text[start..end];
        let mut lines: Vec<Line> = Vec::new();

        for (i, line) in visible_text.iter().enumerate() {
            let spans: Vec<Span>;
            if i+start == self.cursor.row {
                spans = cursor_line_into_spans(line, self.cursor.col);
            } else {
                spans = line_into_spans(line);
            }
            lines.push(Line::from(spans));
        }
        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_cursor_right() {
        let text = vec![
            "Lorem ipsum".into(),
            "dolor sit amet".into(),
            "consectetur".into(),
        ];
        let mut textbox = TextBox::from(text);
        textbox.move_cursor_right();
        assert_eq!(textbox.cursor.col, 1);
    }

    #[test]
    fn test_move_cursor_left() {
        let text = vec![
            "Lorem ipsum".into(),
            "dolor sit amet".into(),
            "consectetur".into(),
        ];
        let mut textbox = TextBox::from(text);
        textbox.move_cursor_right();
        textbox.move_cursor_left();
        assert_eq!(textbox.cursor.col, 0);
    }

    #[test]
    fn test_move_cursor_down() {
        let text = vec![
            "Lorem ipsum".into(),
            "dolor sit amet".into(),
            "consectetur".into(),
        ];
        let mut textbox = TextBox::from(text);
        textbox.move_cursor_down();
        assert_eq!(textbox.cursor.row, 1);
    }

    #[test]
    fn test_move_cursor_up() {
        let text = vec![
            "Lorem ipsum".into(),
            "dolor sit amet".into(),
            "consectetur".into(),
        ];
        let mut textbox = TextBox::from(text);
        textbox.move_cursor_down();
        textbox.move_cursor_up();
        assert_eq!(textbox.cursor.row, 0);
    }
}
