use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, Hash)]
pub enum Tag{
    Work,
    Personal,
    Urgent,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note{
    pub title: String,
    pub body: String,
    pub tag: Tag,
}

pub fn print_note(note: &Note) {
    println!("📌[{:?}] , Title: {}, Body: {}", note.tag, note.title, note.body);
}
