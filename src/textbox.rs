use std::cmp::min;

use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

#[derive(Clone, Debug)]
pub struct TextBox {
    pub text: String,
    pub line_indices: Vec<usize>,
    pub cursor_pos: usize,
}

impl From<String> for TextBox {
    fn from(s: String) -> Self {
        let indices = get_newline_index(&s.clone());
        Self {
            text: s,
            line_indices: indices,
            cursor_pos: 0,
        }
    }
}

impl Into<String> for TextBox {
    fn into(self) -> String {
        self.text
    }
}

impl TextBox {
    pub fn new() -> Self {
        TextBox {
            text: String::new(),
            line_indices: Vec::new(),
            cursor_pos: 0,
        }
    }

    pub fn update_line_indices(&mut self) {
        self.line_indices.clear();
        self.line_indices = get_newline_index(self.text.as_str());
    }

    pub fn insert_char(&mut self, pos: usize, ch: char) {
        self.text.insert(pos, ch);
        self.update_line_indices();
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self, pos: usize) {
        if pos >= 0 {
            self.text.remove(pos);
            self.update_line_indices();
            self.move_cursor_left();
        }
    }

    pub fn insert_newline(&mut self) {
        self.text.insert(self.cursor_pos, '\n');
        self.update_line_indices();
        self.move_cursor_down();
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_pos = min(self.cursor_pos + 1, self.text.len());
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1)
    }

    pub fn move_cursor_up(&mut self) {
        if let Some(index) = self.get_current_line_index() {
            if index > 0 {
                self.cursor_pos = self.line_indices[index - 1];
                print!("up")
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if let Some(index) = self.get_current_line_index() {
            if index < self.line_indices.len() - 1 {
                self.cursor_pos = self.line_indices[index + 1];
                print!("down")
            }
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Backspace => self.delete_char(self.cursor_pos),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Enter => self.insert_newline(),
            KeyCode::Char(ch) => self.insert_char(self.cursor_pos, ch),
            _ => {}
        }

        // print!("{}", self.cursor_pos);
        // self.line_indices.iter().for_each(|&el| print!("{} ", el))
    }

    fn get_current_line_index(&mut self) -> Option<usize> {
        self.line_indices
            .iter()
            .position(|&start| start <= self.cursor_pos)
    }
}

impl Widget for TextBox {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let text = Span::from(self.text.clone());
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

fn get_newline_index(text: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let mut current_line_start = 0;

    for (index, ch) in text.char_indices() {
        if ch == '\n' || index == text.len() - 1 {
            indices.push(current_line_start);
            current_line_start = index + 1;
        }
    }

    // if !text.ends_with('\n') {
    //     indices.push(current_line_start)
    // }

    indices
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_newline_index_finds_all_line_starts() {
        let s = "Test\nfinding\nnewline\nchars".into();
        let idx = get_newline_index(s);

        assert_eq!(idx, vec![0, 5, 13, 21])
    }

    #[test]
    fn test_get_newline_index_with_no_newline_chars() {
        let s = "A String With No Newlines".into();
        let idx = get_newline_index(s);
        assert_eq!(idx, vec![0])
    }

    #[test]
    fn test_textbox_from_string_sets_line_indices() {
        let textbox = TextBox::from("This\nis\nthe\nstring".to_string());
        assert_eq!(textbox.line_indices, [0, 5, 8, 12])
    }

    #[test]
    fn test_textbox_into_string() {
        let textbox = TextBox::from("This\nis\nthe\nstring".to_string());
        let s: String = textbox.into();
        assert_eq!(s, "This\nis\nthe\nstring".to_string())
    }
}
