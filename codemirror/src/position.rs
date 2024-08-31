use derive_more::From;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub line: Line,
    pub column: Column,
}

impl Position {
    pub const fn new(line: Line, column: Column) -> Self {
        Position { line, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub struct Line(pub(crate) u32);

impl Line {
    pub const fn new(line: u32) -> Self {
        Self(line)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub struct Column(u32);

impl Column {
    pub const fn new(column: u32) -> Self {
        Self(column)
    }
}
