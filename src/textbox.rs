use std::cmp::{max, min};

pub enum CursorDirection {
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
}

pub struct TextBox {
    pub text: String,
    pub line_indices: Vec<usize>,
    pub cursor_pos: usize,
}

impl From<String> for TextBox {
    fn from(s: String)-> Self {
        let mut indices = get_newline_index(&s.clone());
        indices.insert(0, 0);
        Self {
            text: s,
            line_indices: indices,
            cursor_pos: 0
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

    fn update_line_indices(&mut self) {
        self.line_indices.clear();
        self.line_indices = get_newline_index(self.text.as_str());
        self.line_indices.insert(0, 0);
    }

    pub fn insert_char(&mut self, pos: usize, ch: char) {
        self.text.insert(pos, ch);
        self.update_line_indices();
        self.move_cursor(CursorDirection::CursorRight);
    }

    pub fn delete_char(&mut self, pos: usize) {
        self.text.remove(pos);
        self.update_line_indices();
        self.move_cursor(CursorDirection::CursorLeft);
    }

    pub fn insert_newline(&mut self) {
        self.text.insert(self.cursor_pos, '\n');
        self.update_line_indices();
        self.move_cursor(CursorDirection::CursorDown);
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        match direction {
            CursorDirection::CursorLeft => {}
            CursorDirection::CursorRight => {}
            CursorDirection::CursorDown => {}
            CursorDirection::CursorUp => {}
        }
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_pos = min(self.cursor_pos + 1, self.text.len())
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_pos = max(self.cursor_pos - 1, 0)
    }

    pub fn move_cursor_down(&mut self) {
        if let Some(index) = self.get_current_line_index() {
            if index > 0 {
                self.cursor_pos = self.line_indices[index - 1]
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if let Some(index) = self.get_current_line_index() {
            if index < self.line_indices.len() - 1 {
                self.cursor_pos = self.line_indices[index + 1]
            }
        }
    }

    fn get_current_line_index(&mut self) -> Option<usize> {
        self.line_indices
            .iter()
            .position(|&start| start <= self.cursor_pos)
    }
}

fn get_newline_index(text: &str) -> Vec<usize> {
    text.char_indices()
        .filter(|(_, ch)| *ch == '\n')
        .map(|(i, _)| i + 1)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_newline_index_finds_all_newlines() {
        let s = "Test\nfinding\nnewline\nchars".into();
        let idx = get_newline_index(s);

        assert_eq!(idx, vec![5, 13, 21])
    }

    #[test]
    fn test_get_newline_index_with_no_newline_chars() {
        let s = "A String With No Newlines".into();
        let idx = get_newline_index(s);
        assert!(idx.is_empty())
    }
}
