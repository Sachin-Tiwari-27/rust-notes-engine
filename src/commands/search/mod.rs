use crate::{errors::NoteError, models::*, storage::*};

pub fn search_note(query: &String) -> Result<Note, NoteError> {
    let notes = load_notes()?;

    //using .into_iter() so that we can return the Note back.
    notes
        .into_iter()
        .find(|note| {
            note.title.eq_ignore_ascii_case(query)
                || note.body.eq_ignore_ascii_case(query)
                || format!("{:?}", note.tag).eq_ignore_ascii_case(query)
        })
        .ok_or(NoteError::NotFound(query.to_string()))
}
