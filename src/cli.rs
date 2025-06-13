use clap::{Parser, Subcommand};

#[derive(clap::Parser)]
#[command(name = "Rust Notes Engine")]
#[command(about = "A CLI note manager written in Rust", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands{
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

    Delete{
        title: String,
    },

    Search{
        query: String,
    },

    ExportMarkdown,
}