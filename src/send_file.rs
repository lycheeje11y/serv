use std::io;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use rocket::tokio::fs::File;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder};

// This is my reimplementation of the Rocket NamedFile struct so that it responds with the ContentType::Binary
#[derive(Debug)]
pub struct SendFile(PathBuf, File);

impl SendFile {
    pub async fn send_file<P: AsRef<Path>>(path: P) -> io::Result<SendFile> {
        let file = File::open(path.as_ref()).await?;
        Ok(SendFile(path.as_ref().to_path_buf(), file))
    }
}

impl<'r> Responder<'r, 'static> for SendFile {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let mut response = self.1.respond_to(req)?;
        response.set_header(ContentType::Binary);

        Ok(response)
    }
}

impl Deref for SendFile {
    type Target = File;

    fn deref(&self) -> &File {
        &self.1
    }
}

impl DerefMut for SendFile {
    fn deref_mut(&mut self) -> &mut File {
        &mut self.1
    }
}
