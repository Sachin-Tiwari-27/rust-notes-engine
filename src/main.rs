mod notes;
use clap::{Parser, Subcommand};
use notes::{Note, Tag, print_note};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use serde_json;

#[derive(clap::Parser)]
#[command(name = "Rust Notes Engine")]
#[command(about = "A CLI note manager written in Rust", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands{
    ///Add new Note
    Add{
        title: String,
        body: String,
        tag: String,
    },

    ///List all notes
    List,

    ///Filters all notes by tag
    Filter{
        tag: String,
    },

}
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


