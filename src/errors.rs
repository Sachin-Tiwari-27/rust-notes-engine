use std::fmt;

#[derive(Debug)]

pub enum NoteError {
    NotFound(String),
    IOError(String),
    ParseError(String),
}

impl std::error::Error for NoteError {}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoteError::NotFound(msg) => write!(f, "Note not found: {}", msg),
            NoteError::IOError(msg) => write!(f, "I/O Error: {}", msg),
            NoteError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}
