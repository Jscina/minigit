# MiniGit 🧪

A minimalist version control system written in Rust, inspired by the core concepts of Git.

MiniGit supports a basic local workflow:

- `init` a repository
- `add` files to a staging area
- `commit` changes with a message
- view the commit `log`

Ideal as a learning tool or tiny custom VCS for personal projects.

---

## 🚀 Features

✅ `minigit init`  
Creates a `.minigit/` directory with the required structure:

- `objects/`: stores file and commit blobs
- `index`: tracks staged files
- `HEAD`: tracks the latest commit

✅ `minigit add <files or directories>`  
Stages files for commit by hashing their content and saving them to `objects/`.

- Supports globbing via shell (`minigit add src/*.rs`)
- Recursively adds directories
- Ignores files listed in `.minigitingore`

✅ `minigit commit -m "message"`  
Creates a commit:

- Stores a JSON blob with timestamp, message, and staged file hashes
- Clears the index
- Updates `HEAD` to point to the new commit

✅ `minigit log`  
Displays info about the latest commit:

- Commit hash
- Timestamp
- Commit message
- List of files and hashes

---

## 🗃️ Commit Format

Commits are stored as JSON inside Git-style blob objects:

```json
{
  "message": "Initial commit",
  "timestamp": "2025-04-30T15:00:00Z",
  "files": [
    { "path": "src/main.rs", "hash": "5dd01c..." },
    { "path": "README.md", "hash": "9a2fe1..." }
  ]
}
```

---

## 📂 `.minigitingore`

Use this file in your root directory to ignore files or folders from being staged.

Example:

```
target/
*.log
*.rs.bk
```

---

## 🛠 Build

```bash
cargo build --release && cp target/release/minigit /usr/local/bin
```

# Usage:

```bash
minigit init
minigit add src/
minigit commit -m "Add core logic"
minigit log
minigit --help
```

---

## 📦 Dependencies

- [`clap`](https://crates.io/crates/clap) – CLI argument parsing
- [`sha1`](https://crates.io/crates/sha1) – Git-style hashing
- [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) – commit serialization
- [`chrono`](https://crates.io/crates/chrono) – timestamps
- [`anyhow`](https://crates.io/crates/anyhow) – error handling

---

## 📎 License

MIT. Build cool stuff.
