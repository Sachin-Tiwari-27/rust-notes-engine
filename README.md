# 🦀 Rust Notes Engine

A modular, testable CLI-based note-taking application written in Rust. Supports tagging, filtering, Markdown export, and more.

---

## 🛠 Getting Started

```bash
git clone https://github.com/Sachin-Tiwari-27/rust-notes-engine.git
cd rust-notes-engine
cargo run


## 🚀 Features

- Add, update, delete notes
- Tag notes by category (Work, Personal, Other)
- Search notes by title
- Filter notes by tag
- Export all notes to Markdown
- Persistent JSON storage
- Modular command structure
- Unit + Integration tests

---

## 📁 Project Structure

```
// Updated Folder Structure with Modular Commands

// ├── src/
// │   ├── main.rs
// │   ├── cli.rs
// │   ├── models.rs
// │   ├── errors.rs
// │   ├── storage.rs
// │   └── commands/
// │       ├── mod.rs
// │       ├── add/
// │       │   ├── mod.rs
// │       │   └── tests.rs
// │       ├── update/
// │       │   ├── mod.rs
// │       │   └── tests.rs
// │       ├── delete/
// │       │   ├── mod.rs
// │       │   └── tests.rs
// │       ├── search/
// │       │   └── mod.rs
// │       ├── export_to_markdown/
// │       │   └── mod.rs
// │       └── tests.rs   // optional shared test utilities
// └── tests/
//     └── integration.rs
---

## 🧑‍💻 Usage

### Add Note:
```bash
cargo run -- add "Meeting" "Project kickoff" "Work"
```

### Update Note:
```bash
cargo run -- update "Meeting" "Updated body" "Personal"
```

### Delete Note:
```bash
cargo run -- delete "Meeting"
```

### Search Note:
```bash
cargo run -- search "Meeting"
```

### Export to Markdown:
```bash
cargo run -- exportmarkdown "notes_export.md"
```

---

## 🧪 Run Tests
```bash
cargo test -- test_add_notes
```

### Integration Test:
```bash
cargo test --test integration
```

---

## 🛠️ Built With
- Rust
- Clap
- Serde
- std::fs
- std::io

---

## 📄 License
MIT

---

Happy learning and hacking 🦀!