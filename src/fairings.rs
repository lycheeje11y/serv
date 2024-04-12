use std::sync::{Arc, Mutex};
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use crate::config;

pub fn insert_config(config: config::ServConfig) -> AdHoc {
	AdHoc::on_ignite("Insert Config", |rocket| async {
		rocket.manage(Arc::new(Mutex::new(config)))
	})
}


pub fn download() -> AdHoc {
	AdHoc::on_response("Download Files Correctly", |req, resp| Box::pin(async {
		let uri = req.uri().to_string();

		if uri.contains("/files/") {
			resp.set_header(ContentType::Binary);
		}
	}))
}
