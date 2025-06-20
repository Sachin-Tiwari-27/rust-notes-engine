use crate::{errors::NoteError, storage::*};
use std::fs::File;
use std::io::Write;

pub fn export_to_markdown(output_path: &str) -> Result<(), NoteError> {
    let notes = load_notes()?;
    let mut md_content = String::new();

    for note in notes {
        md_content.push_str(&format!(
            "# {}\n\n{}\n\n**Tag**: {:?}\n\n---\n\n",
            note.title, note.body, note.tag
        ));
    }

    let mut file = File::create(output_path).map_err(|e| NoteError::IOError(e.to_string()))?;
    file.write_all(md_content.as_bytes())
        .map_err(|e| NoteError::IOError(e.to_string()))?;

    Ok(())
}
