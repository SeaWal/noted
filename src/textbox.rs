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

impl TextBox {
    pub fn new() -> Self {
        TextBox {
            text: String::new(),
            line_indices: Vec::new(),
            cursor_pos: 0,
        }
    }

    #[allow(dead_code)]
    fn update_line_indices(&mut self) {
        self.line_indices = get_newline_index(self.text.as_str());
    }

    pub fn insert_char(&mut self, pos: usize, ch: char) {
        self.text.insert(pos, ch)
    }

    pub fn insert_newline(&mut self) {
        self.insert_char(self.cursor_pos, '\n')
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        match direction {
            CursorDirection::CursorLeft => {}
            CursorDirection::CursorRight => {}
            CursorDirection::CursorDown => {}
            CursorDirection::CursorUp => {}
        }
    }
}

fn get_newline_index(text: &str) -> Vec<usize> {
    text.char_indices()
        .filter(|(_, ch)| *ch == '\n')
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_newline_index_finds_all_newlines() {
        let s = "Test\nfinding\nnewline\nchars".into();
        let idx = get_newline_index(s);

        assert_eq!(idx, vec![4, 12, 20])
    }

    #[test]
    fn test_get_newline_index_with_no_newline_chars() {
        let s = "A String With No Newlines".into();
        let idx = get_newline_index(s);
        assert!(idx.is_empty())
    }
}
