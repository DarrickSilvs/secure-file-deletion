use clap::Parser;
use anyhow::{Context, Result};
use rand_core::{TryRngCore, OsRng};
use std::{
    fs::{self, OpenOptions}, io::Write, path::PathBuf
};

#[derive(Parser, Debug)]
pub struct Arguments {
    // path to the file to shred
    file_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    let file_path = args.file_path;

    if file_path.is_dir() {
        println!("Directory is not supported for removal.")
    }

    let size = fs::metadata(&file_path)
                .with_context(|| format!("Could not access file metadata: {}", file_path.display()))?
                .len();

    let mut file = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(&file_path)
                            .with_context(|| format!("Could not open/write file: {}", file_path.display()))?;
    
    let mut remaining = size;
    const CHUNK_SIZE: usize = 8192; // 8192 is 8kb, commonly used by OS for buffer size
    let mut buffer = [0u8; CHUNK_SIZE];

    while remaining > 0 {
        let to_write = std::cmp::min(remaining, CHUNK_SIZE as u64) as usize;
        
        OsRng.try_fill_bytes(&mut buffer[..to_write]).unwrap();
        file.write_all(&buffer[..to_write])?;

        remaining -= to_write as u64;
    }
    file.sync_all().with_context(|| format!("Failed syncing: {}", file_path.display()))?;
    drop(file);

    Ok(())
}
