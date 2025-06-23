use crate::{models::Tag, storage::load_notes, errors::NoteError};
use std::collections::HashMap;

pub fn show_tag_stats() -> Result<(), NoteError> {
    let notes = load_notes()?;
    let mut counts: HashMap<Tag, usize> = HashMap::new();

    for note in notes{
        *counts.entry(note.tag.clone()).or_insert(0) +=1;
    }

    println!("🧮 Note count per tag:\n");

    for (tag,count) in &counts {
        let emoji  = match tag {
            Tag::Work => "📘",
            Tag::Personal => "🏠",
            Tag::Urgent => "🚨",
            Tag::Other => "🗂️",
        };
        println!("{} {:<10} : {}", emoji, format!("{:?}", tag), count);
    }

    Ok(())
}