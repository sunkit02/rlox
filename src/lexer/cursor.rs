#[allow(dead_code)]
pub trait Peekable {
    type Item;

    /// Peek at the index of `current_index + n` without consuming it
    fn peek_nth(&self, n: usize) -> Option<Self::Item>;

    /// Peek at the index of `current_index - n` without consuming it
    fn peek_prev_nth(&self, n: usize) -> Option<Self::Item>;

    /// Peek at the next value without consuming it
    fn peek(&self) -> Option<Self::Item> {
        self.peek_nth(0)
    }

    /// Peek at the previous value without comsuming it
    fn peek_prev(&self) -> Option<Self::Item> {
        self.peek_prev_nth(1)
    }
}

/// A cursor over a [Vec<char>](std::alloc::Vec)
#[derive(Debug)]
pub struct Cursor {
    stack: Vec<char>,
    needle: usize,
}

impl Cursor {
    pub fn new(source: &str) -> Self {
        Self {
            stack: source.chars().collect(),
            needle: 0,
        }
    }

    /// Gets a substring that starts and ends at the specified indicies, exclusive.
    /// Returns `None` if the one or both of the indices given are invalid.
    pub fn substring(&self, start: usize, end: usize) -> Option<String> {
        let stack_len = self.stack.len();

        let substring_option = if start > stack_len || end > stack_len || start > end {
            return None;
        } else {
            let slice = &self.stack[start..end];
            let slice = slice.iter().collect();

            Some(slice)
        };

        substring_option
    }

    // For testing only
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_at_end(&self) -> bool {
        self.needle >= self.stack.len()
    }
}

impl Iterator for Cursor {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.stack.get(self.needle)?;

        self.needle += 1;

        Some(*c)
    }
}

impl Peekable for Cursor {
    type Item = char;

    fn peek_nth(&self, n: usize) -> Option<Self::Item> {
        match self.needle.checked_add(n) {
            Some(target) if target < self.stack.len() => Some(self.stack[target]),
            _ => None,
        }
    }

    fn peek_prev_nth(&self, n: usize) -> Option<Self::Item> {
        self.needle.checked_sub(n).map(|target| self.stack[target])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring_return_none_when_start_greater_than_len() {
        let cursor = Cursor::new("hello world");
        let len = cursor.len();

        let start = len + 1;
        let end = start;

        assert_eq!(cursor.substring(start, end), None);
    }

    #[test]
    fn substring_return_none_when_end_greater_than_len() {
        let cursor = Cursor::new("hello world");
        let len = cursor.len();

        let end = len + 1;
        let start = end;

        assert_eq!(cursor.substring(start, end), None);
    }

    #[test]
    fn substring_return_none_when_start_greater_than_end() {
        let cursor = Cursor::new("hello world");

        let end = 0;
        let start = end + 1;

        assert_eq!(cursor.substring(start, end), None);
    }
}
