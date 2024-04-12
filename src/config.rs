use std::path::PathBuf;

#[derive(Clone)]
pub struct ServConfig {
    pub path: PathBuf,
    pub port: u32,
}
