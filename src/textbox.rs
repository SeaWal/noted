pub struct TextBox {
    pub lines: String,
    pub line_indices: Vec<usize>,
    pub cursor_pos: usize,
}

impl TextBox {
    pub fn new() -> Self {
        TextBox {
            lines: String::new(),
            line_indices: Vec::new(),
            cursor_pos: 0,
        }
    }

    fn update_line_indices(&mut self) {
        self.line_indices = get_newline_index(self.lines.as_str());
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
