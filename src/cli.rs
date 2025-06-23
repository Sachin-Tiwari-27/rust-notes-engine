use clap::{Parser};

#[derive(Parser)]
#[command(name = "Rust Notes Engine")]
#[command(about = "A CLI note manager written in Rust", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    ///Add new Note
    Add {
        title: String,
        body: String,
        tag: String,
    },

    ///Update notes
    Update {
        title: String,
        new_body: String,
        new_tag: String,
    },

    ///List all notes
    List,

    ///Filters all notes by tag
    Filter { tag: String },

    ///Delete note by title
    Delete { title: String },

    ///Search note matching the text
    Search { query: String },

    ///Show Tag counts
    Stats,

    ///Export note to a .md file
    ExportMarkdown { output_path: String },
}
