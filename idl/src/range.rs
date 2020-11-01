use std::fmt;

#[derive(Debug, Copy, Clone, Default)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn get_byte_range(self, text: &str) -> Option<std::ops::Range<usize>> {
        let mut count = 0;
        let mut start = None;
        let mut end = None;

        for (index, line) in text.lines().enumerate() {
            if index == self.start.line {
                start = Some(count + self.start.column);
                if end.is_some() {
                    break;
                }
            }

            if index == self.end.line {
                end = Some(count + self.end.column);
                if start.is_some() {
                    break;
                }
            }

            count += line.len() + 1; //?? considers only \n?
        }

        Some(std::ops::Range {
            start: start?,
            end: end?,
        })
    }

    pub fn merge(self, range: Range) -> Self {
        Self {
            start: self.start,
            end: range.end,
        }
    }

    pub fn from_position(position: Position) -> Self {
        Self {
            start: position,
            end: position,
        }
    }

    pub fn as_position(self) -> Position {
        Position {
            line: self.start.line,
            column: self.start.column,
        }
    }

    pub fn end_as_range(self) -> Self {
        self.end.as_range()
    }

    pub fn in_range(self, other: Self) -> bool {
        self.start >= other.start && self.end <= other.end
            || self.start <= other.end && self.end >= other.start
    }
}

#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn as_range(self) -> Range {
        Range::from_position(self)
    }
}
