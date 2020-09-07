use super::scanner::{WRange, WordRange};

#[derive(Debug, Copy, Clone, Default)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub(super) fn from_word_range<T>(value: &WordRange<T>) -> Self {
        let line = value.line;
        let index = value.index;
        let length = value.length;

        Self {
            start: Position { line, index },
            end: Position {
                line,
                index: index + length,
            },
        }
    }

    pub(super) fn from_wrange(value: WRange) -> Self {
        Self {
            start: Position {
                index: value.index,
                line: value.line,
            },
            end: Position {
                line: value.line,
                index: value.index + value.length,
            },
        }
    }

    pub(super) fn merge(self, range: Range) -> Self {
        Self {
            start: self.start,
            end: range.end,
        }
    }

    pub(super) fn from_position(position: Position) -> Self {
        Self {
            start: position,
            end: position,
        }
    }

    pub(super) fn as_position(self) -> Position {
        Position {
            line: self.start.line,
            index: self.start.index,
        }
    }

    pub(super) fn end_as_range(self) -> Self {
        self.end.as_range()
    }

    pub(super) fn in_range(self, other: Self) -> bool {
        self.start >= other.start && self.end <= other.end ||
        self.start <= other.end && self.end >= other.start
    }
}

#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Position {
    pub line: usize,
    pub index: usize,
}

impl Position {
    pub(super) fn as_range(self) -> Range {
        Range::from_position(self)
    }
}