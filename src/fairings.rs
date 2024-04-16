use crate::config::ServConfig;
use rocket::fairing::AdHoc;
use rocket::form::validate::Contains;
use rocket::http::ContentType;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn download() -> AdHoc {
    AdHoc::on_response("Download Files Correctly", |req, resp| {
        Box::pin(async move {
            let uri = req.uri().to_string();
            let queries = req.uri().query();
            if queries.is_some() {
                let query_segs: Vec<(&str, &str)> = queries.unwrap().segments().collect();
                if query_segs.contains(("download", "true")) {
                    let config_binding = req.rocket().state::<Arc<Mutex<ServConfig>>>().unwrap();
                    let mut config = config_binding.lock().unwrap();

                    let filename = PathBuf::from(uri.replace("/files/", ""));
                    let location = &config.base_directory;
                    let full_path = location.join(&filename);

                    if full_path.is_dir() {
                        config.base_directory = config.base_directory.join(filename);
                    } else {
                        resp.set_header(ContentType::Binary);
                    }
                }
            }
        })
    })
}
