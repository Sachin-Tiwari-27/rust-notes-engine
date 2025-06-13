mod notes;
mod cli;

use cli::{Cli, Commands};
use clap::Parser;
use notes::{Note, Tag, print_note};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use serde_json;


fn main() {

    let cli = Cli::parse();

    let mut notes = load_notes();

    match &cli.command {
        Commands::Add { title, body, tag } => {
            let parsed_tag = match tag.to_lowercase().as_str() {
                "work" => Tag::Work,
                "personal" => Tag::Personal,
                "urgent" => Tag::Urgent,
                _=> {
                    println!("Invalid Tag, use Work, Personal, or Urgent.");
                    return;
                }
            };

            let note = Note {
                title: title.clone(),
                body: body.clone(),
                tag: parsed_tag,
            };

            notes.push(note);
            save_notes(&notes);
            println!("✅ Note added.")
        }
        Commands::List => {
            if notes.is_empty(){
                println!("No notes yet.");
            } else {
                for note in &notes {
                    print_note(note);
                }
            }

        }

        Commands::Filter {tag} => {
            let parsed_tag = match tag.to_lowercase().as_str() {
                "work" => Tag::Work,
                "personal" => Tag::Personal,
                "urgent" => Tag::Urgent,
                _=> {
                    println!{"Invalid tag. Use Work, Personal or Urgent."};
                    return;
                }
            };

            let filtered : Vec<&Note> = notes
            .iter()
            .filter(|n| n.tag == parsed_tag)
            .collect();
            
            if filtered.is_empty(){
                println!("No Notes found for tag {:?}", parsed_tag);
            } else {
                for note in filtered {
                    print_note(note);
                }
            }
        }

        Commands::Delete { title } => {
            let initial_len = notes.len();
            let title_lower = title.to_lowercase();
            notes.retain(|note| note.title.to_lowercase() != title_lower);

            if notes.len() < initial_len {
                save_notes(&notes);
                println!("🗑️ Deleted note with title: '{}'", title);
            }
            else {
                println!("⚠️ No note found with title: '{}'", title);
            }
        }

        Commands::Search { query } => {

            let query_lower = query.to_lowercase();

            let matched: Vec<&Note> = notes.iter().filter(|note| {
                note.title.to_lowercase().contains(&query_lower) || 
                note.body.to_lowercase().contains(&query_lower) ||
                format!("{:?}", note.tag).to_lowercase().contains(&query_lower)
            }).collect();

            if matched.is_empty() {
                println!("🔍 No notes found for query: '{}'", query);
            }
            else {
                println!("🔍 Found {} notes matching '{}':", matched.len(), query);
            for note in matched {
                 print_note(note);
                 }
            }
        
        }

        Commands::ExportMarkdown => {

            if notes.is_empty() {
                 println!("📝 No notes to export.");
             } else {
                 export_to_markdown(&notes);
            }

        }

       

    }
    

}

fn load_notes() -> Vec<Note> {
    let mut file = match fs::File::open("notes.json") {
        Ok(f) => f,
        Err(_) => return Vec::new() // file not found
    };

    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");

    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_notes(notes: &Vec<Note>) {
    let data = serde_json::to_string_pretty(notes).expect("Failed to serialize notes");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("notes.json")
        .expect("Failed to open file");
    file.write_all(data.as_bytes()).expect("Failed to write file");

}

fn export_to_markdown(notes: &Vec<Note>) {
    let dir = "markdown_exports";

    fs::create_dir_all(dir).expect("❌ Failed to create export folder");

    for note in notes {
        let filename = format!("{}/{}.md",dir,sanitize_filename(&note.title));
        let mut file = File::create(&filename).expect("❌ Failed to create file");

        let content = format!(
            "# {}\n\n{}\n\n**Tag** {:?}\n",
            note.title, note.body, note.tag
        );

        file.write_all(content.as_bytes()).expect("❌ Failed to write to file");
        println!("✅ Exported: {}", filename);
    }
}

fn sanitize_filename(title: &str) -> String {
    title.replace("/", "-").replace("\\", "-").replace(" ", "_")
}
