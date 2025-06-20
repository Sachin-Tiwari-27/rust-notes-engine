use crate::errors::NoteError;
use crate::models::Note;
use std::fs::{read_to_string, write};

const FILE: &str = "notes.json";

pub fn load_notes() -> Result<Vec<Note>, NoteError> {
    match read_to_string(FILE) {
        Ok(data) => serde_json::from_str(&data).map_err(|e| NoteError::ParseError(e.to_string())),
        Err(_) => Ok(Vec::new()),
    }
}

pub fn save_notes(notes: &Vec<Note>) -> Result<(), NoteError> {
    let data =
        serde_json::to_string_pretty(notes).map_err(|e| NoteError::ParseError(e.to_string()))?;
    write(FILE, data).map_err(|e| NoteError::IOError(e.to_string()))
}
