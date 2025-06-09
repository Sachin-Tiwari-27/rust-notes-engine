mod notes;
use clap::{Parser, Subcommand};
use notes::{Note, Tag, print_note};

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

    let mut notes: Vec<Note> = Vec::new();

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
