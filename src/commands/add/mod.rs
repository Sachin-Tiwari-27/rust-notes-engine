use crate::{errors::NoteError, models::*, storage::*};

pub fn add_note(title: &str, body: &str, tag: &Tag) -> Result<(), NoteError> {
    let mut notes = load_notes()?;
    if notes.iter().any(|n| n.title == title) {
        return Err(NoteError::ParseError("Duplicate title".into()));
    }
    notes.push(Note {
        title: title.to_string(),
        body: body.to_string(),
        tag: tag.clone(),
    });
    save_notes(&notes)?;
    Ok(())
}
