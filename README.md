# 🦀 Rust Notes Engine

A command-line note-taking app written in Rust — simple, clean, and powerful.

---

## 🛠 Getting Started

```bash
git clone https://github.com/Sachin-Tiwari-27/rust-notes-engine.git
cd rust-notes-engine
cargo run


## 🚀 Features

- Add, List, Filter notes
- Search in title, body, or tag
- Delete notes by title
- Export notes to `.md` files in Markdown format

## 🛠 Commands

```bash
# Add a note
cargo run -- add "Title" "Body content" "Tag"

# List all notes
cargo run -- list

# Filter by tag
cargo run -- filter "Work"

# Search notes
cargo run -- search "meeting"

# Delete by title
cargo run -- delete "Title"

# Export to markdown
cargo run -- export-markdown
```

## 📁 Folder Structure

- `src/cli.rs`: CLI parsing using clap
- `src/main.rs`: Logic + file I/O
- `notes.json`: Stored notes

---