mod cli;
mod commands;
mod errors;
mod models;
mod storage;

use crate::commands::{
    add::add_note, delete::delete_note, export_to_markdown::export_to_markdown,
    search::search_note, update::update_note, stats::show_tag_stats
};
use crate::{
    models::{Tag, print_note},
    storage::load_notes,
};
use clap::Parser;
use cli::{Cli, Commands};

fn parsed_tag(input: &str) -> Tag {
    match input.to_lowercase().as_str() {
        "work" => Tag::Work,
        "personal" => Tag::Personal,
        "urgent" => Tag::Urgent,
        _ => Tag::Other,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { title, body, tag } => {
            let tag_enum = parsed_tag(tag);

            match add_note(title, body, &tag_enum) {
                Ok(_) => println!("✅ Note added successfully!"),
                Err(e) => eprintln!("❌ Error: {}", e),
            }
        }
        Commands::Update {
            title,
            new_body,
            new_tag,
        } => {
            let tag_enum = parsed_tag(new_tag);
            match update_note(title, new_body, &tag_enum) {
                Ok(_) => println!("✅ Note updated!"),
                Err(e) => eprintln!("❌ Error: {}", e),
            }
        }
        Commands::List => {
            let notes = load_notes().unwrap_or_default();
            for note in notes {
                print_note(&note);
            }
        }
        Commands::Filter { tag } => {
            let tag_enum = parsed_tag(tag);
            let notes = load_notes().unwrap_or_default();
            for note in notes.iter().filter(|n| n.tag == tag_enum) {
                print_note(note);
            }
        }
        Commands::Delete { title } => match delete_note(title) {
            Ok(_) => println!("🗑️ Note deleted!"),
            Err(e) => eprintln!("❌ Error: {}", e),
        },
        Commands::Search { query } => match search_note(query) {
            Ok(note) => print_note(&note),
            Err(e) => eprintln!("❌ Error: {}", e),
        },
        Commands::Stats {} => {
            if let Err(e) = show_tag_stats() {
        eprintln!("Error: {}", e);
    }
        },
        Commands::ExportMarkdown { output_path } => match export_to_markdown(output_path) {
            Ok(_) => println!("✅ Note Exorted!"),
            Err(e) => eprintln!("❌ Error: {}", e),
        },
    }
}
