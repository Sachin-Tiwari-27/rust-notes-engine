use crate::{errors::NoteError, models::*, storage::*};

pub fn update_note(title: &str, new_body: &str, new_tag: &Tag) -> Result<(), NoteError> {
    let mut notes = load_notes()?;
    let mut found = false;
    for note in notes.iter_mut() {
        if note.title == title {
            note.body = new_body.to_string();
            note.tag = new_tag.clone();
            found = true;
            break;
        }
    }

    if !found {
        return Err(NoteError::NotFound(title.into()));
    }
    save_notes(&notes)?;
    Ok(())
}
