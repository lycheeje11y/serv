use std::path::PathBuf;

#[derive(Clone)]
pub struct ServConfig {
    pub start_directory: PathBuf,
    pub base_directory: PathBuf,
    pub port: u32,
}
