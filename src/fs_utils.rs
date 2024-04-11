use std::{env, fs, io};
use std::path::PathBuf;
use human_bytes::human_bytes;


pub fn walk_dir(dir: &PathBuf) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(dir)?;

    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();

            path.file_name()?.to_str().map(|s| s.to_owned())
        })
        .collect();

    Ok(file_names)
}

pub fn get_cwd() -> String {
    let binding = env::current_dir().unwrap();
    binding.to_str().unwrap().to_string()
}

// this is an unsafe function; only use it in templates.
pub fn get_size(file: &PathBuf) -> u64 {
    return fs::metadata(file).unwrap().len()
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
