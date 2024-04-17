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
    human_bytes(*bytes as f64)
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

pub fn is_valid_subpath(relative_directory: &PathBuf, base_directory: &PathBuf) -> bool {
    let in_question = base_directory.join(relative_directory);
    in_question.starts_with(base_directory)
}

pub fn get_parent_directory(path: &PathBuf, base_directory: &PathBuf) -> Option<PathBuf> {
    let difference = get_relative_path(&path, &base_directory);
    let mut difference_fields: Vec<PathBuf> = vec![];
    for v in difference.iter() {
        difference_fields.push(PathBuf::from(v));
    }

    return if difference_fields.len() == 0 {
        None
    } else {
        if let Some(result) = path.parent() {
            let result = result.strip_prefix(base_directory).unwrap().to_path_buf();
            Some(result)
        } else {
            None
        }
    };
}

pub fn get_relative_path(file_path: &PathBuf, base_directory: &PathBuf) -> PathBuf {
    return if let Ok(path) = file_path.strip_prefix(base_directory) {
        path.to_path_buf()
    } else {
        file_path.to_path_buf()
    };
}

