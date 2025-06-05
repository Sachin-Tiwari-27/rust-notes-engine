#[derive(Debug)]
pub enum Tag{
    Work,
    Personal,
    Urgent
}

#[derive(Debug)]
pub struct Note{
    pub title: String,
    pub body: String,
    pub tag: Tag,
}

pub fn print_note(note: &Note) {
    println!("[{:?}] {}", note.tag, note.title);
}