## Secure File Shredder (Rust)

A command-line tool written in Rust for securely deleting files by overwriting, renaming, and clearing metadata. Designed for macOS.

#### Features
- Overwrites file contents with cryptographically secure random bytes (_OsRng_)
- Supports multiple overwrite passes
- Renames file with random characters before deletion
- Clears file metadata to reduce recovery risk

#### Usage
Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

##### Basic usage
```
cargo run -- path/to/file.txt
```
This will securely shred the file with 1 overwrite pass by default.

##### Custom number of overwrite passes
```
cargo run -- path/to/file.txt -p 3
```
This overwrites the file 3 times before renaming and deletion.

#### Notes
- This tool is intended fo macOS.
- Use with caution, as files deleted using this tool are not recoverable.
