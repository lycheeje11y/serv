use std::path::PathBuf;

#[derive(Clone)]
pub struct ServConfig {
    pub base_directory: PathBuf,
    pub port: u32,
}
