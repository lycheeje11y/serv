use human_bytes::human_bytes;
use std::path::PathBuf;
use std::{env, fs, io};

pub fn walk_dir(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?;

    let paths: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();

    Ok(paths)
}

pub fn get_cwd() -> String {
    let binding = env::current_dir().unwrap();
    binding.to_str().unwrap().to_string()
}

// this is an unsafe function; only use it in templates.
pub fn get_size(file: &PathBuf) -> u64 {
    match fs::metadata(file) {
        Ok(t) => t.len(),
        Err(_) => 0,
    }
}

pub fn human_readable_size(bytes: &u64) -> String {
    human_bytes(bytes.clone() as f64)
}

pub fn is_file(path: &PathBuf) -> bool {
    if !path.is_dir() {
        return false;
    }
    if !path.exists() {
        return false;
    }

    true
}

pub fn get_filename(path: &PathBuf) -> String {
    return path.file_name().unwrap().to_str().unwrap().to_string();
}
