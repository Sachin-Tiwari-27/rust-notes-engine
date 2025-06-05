mod notes;
use notes::{Note, Tag, print_note};

fn main() {
    let note = Note{
        title: "Rust Study".into(),
        body: "Focus on error handling today".into(),
        tag: Tag::Work,
    };
    print_note(&note);

}
