use std::{fs, io};

pub fn walk_dir(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;

    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            path.file_name()?.to_str().map(|s| s.to_owned())
        })
        .collect();

    Ok(file_names)
}
