use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

#[derive(Clone, Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct TextBox {
    pub text: Vec<String>,
    pub cursor: Cursor,
}

impl From<Vec<String>> for TextBox {
    fn from(v: Vec<String>) -> Self {
        Self {
            text: v,
            cursor: Cursor { row: 0, col: 0 },
        }
    }
}

impl TextBox {
    pub fn new() -> Self {
        TextBox {
            text: Vec::new(),
            cursor: Cursor { row: 0, col: 0 },
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Enter => self.insert_newline(),
            KeyCode::Char(ch) => self.insert_char(ch),
            KeyCode::Backspace => self.delete_char(),
            _ => {}
        }
    }

    fn move_cursor_right(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        let curr_line = &self.text[row];
        if col < curr_line.len() {
            self.cursor.col = col.saturating_add(1);
        } else {
            if row + 1 < self.text.len() {
                self.cursor.row = row.saturating_add(1);
                self.cursor.col = 0;
            }
        }
    }

    fn move_cursor_left(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        let curr_line = &self.text[row];

        if col > 0 {
            self.cursor.col = col.saturating_sub(1);
        }

        if (col == 0) & (row - 1 > 0) {
            self.cursor.row = row.saturating_sub(1);
            self.cursor.col = curr_line.len() - 1;
        }
    }

    fn move_cursor_down(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);

        if row + 1 < self.text.len() {
            self.cursor.row = row.saturating_add(1);

            let next_line_len = self.text[row + 1].len();
            if col > next_line_len {
                self.cursor.col = next_line_len;
            }
        }
    }

    fn move_cursor_up(&mut self) {
        let (row, col) = (self.cursor.row, self.cursor.col);

        if row - 1 >= 0 {
            self.cursor.row = row.saturating_sub(1);

            let next_line_len = self.text[row + 1].len();
            if col > next_line_len {
                self.cursor.col = next_line_len;
            }
        }
    }

    fn insert_char(&mut self, ch: char) {
        let (row, col) = (self.cursor.row, self.cursor.col);
        let curr_line = &mut self.text[row];
        curr_line.insert(col, ch);
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
}

impl Widget for TextBox {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut lines: Vec<Span> = Vec::new();
        for line in self.text.iter() {
            lines.push(Span::from(line));
        }
        Paragraph::new(Line::from(lines))
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
