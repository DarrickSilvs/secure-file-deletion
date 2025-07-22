use clap::Parser;
use anyhow::{Context, Result};
use rand_core::{TryRngCore, OsRng};
use rand::distr::{Distribution, Alphanumeric};
use std::{
    fs::{self, OpenOptions}, io::Write, path::PathBuf
};

#[derive(Parser, Debug)]
#[command(name = "shredder", version, about = "Secure file deletion")]
pub struct Arguments {
    // path to the file to shred
    pub file_path: PathBuf,

    #[arg(short, long, default_value_t = 1)]
    pub passes: u8,
}

pub fn file_shred(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if path.is_dir() {
        println!("Directory is not supported for removal.")
    }

    let size = fs::metadata(&path)
                .with_context(|| format!("Could not access file metadata: {}", path.display()))?
                .len();

    let mut file = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(&path)
                            .with_context(|| format!("Could not open/write file: {}", path.display()))?;
    
    let mut remaining = size;
    const CHUNK_SIZE: usize = 8192; // 8192 is 8kb, commonly used by OS for buffer size
    let mut buffer = [0u8; CHUNK_SIZE];

    while remaining > 0 {
        let to_write = std::cmp::min(remaining, CHUNK_SIZE as u64) as usize;
        
        OsRng.try_fill_bytes(&mut buffer[..to_write]).unwrap();
        file.write_all(&buffer[..to_write])?;

        remaining -= to_write as u64;
    }
    file.sync_all().with_context(|| format!("Failed syncing: {}", path.display()))?;
    drop(file);

    Ok(())
}

pub fn file_remove(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = path.file_name()
                                .with_context(|| format!("File name not found in: {}", path.display()))?
                                .to_string_lossy();

    let name_len = file_name.chars().count();
    let mut rng = rand::rng();
    let random_name: String = Alphanumeric
        .sample_iter(&mut rng)
        .take(name_len)
        .map(char::from)
        .collect();
    
    let new_path = match path.parent() {
        Some(parent) => parent.join(&random_name),
        None => PathBuf::from(&random_name),
    };

    fs::rename(path, &new_path)
        .with_context(|| format!("Failed to rename {} to {}", path.display(), new_path.display()))?;

    fs::remove_file(&new_path)
        .with_context(|| format!("Failed to remove file: {}", &new_path.display()))?;

    Ok(())
}

fn main() {
    let file_path = Arguments::parse().file_path;
    let passes = Arguments::parse().passes;

    for _ in 0..passes {
        let _ = file_shred(&file_path);
        let _= file_remove(&file_path);
    }


}
