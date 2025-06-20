use crate::{errors::NoteError, storage::*};

pub fn delete_note(title: &str) -> Result<(), NoteError> {
    let mut notes = load_notes()?;
    let len_before = notes.len();
    let title_lowercase = title.to_lowercase();
    notes.retain(|n| n.title.to_lowercase() != title_lowercase);
    if notes.len() == len_before {
        return Err(NoteError::NotFound(title.into()));
    }
    save_notes(&notes)?;
    Ok(())
}
