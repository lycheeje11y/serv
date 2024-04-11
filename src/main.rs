mod cli;
mod config;
mod fs_utils;
mod fairings;

use rocket as rkt;

use askama_rocket::Template;
use clap::Parser;
use rocket::State;
use rocket_include_static_resources::manifest_dir_macros::relative_path;
use rocket_include_static_resources::static_resources_initializer;
use std::path::PathBuf;
use std::process::exit;

use rocket::fs::{FileServer, Options};
use std::sync::{Arc, Mutex};

#[derive(Template)]
#[template(path = "files.html")]
struct IndexTemplate {
    files: Vec<PathBuf>,
}

#[rkt::get("/")]
fn index(config_guard: &State<Arc<Mutex<config::ServConfig>>>) -> IndexTemplate {
    let config = config_guard.lock().unwrap();
    let mut files = fs_utils::walk_dir(&config.path).unwrap();


    files.sort();
    IndexTemplate { files: files.into_iter().map(PathBuf::from).collect() }
}

#[rkt::catch(404)]
fn not_found() -> &'static str {
    "404 Not Found"
}

#[rkt::launch]
fn launch() -> _ {
    let matches = cli::Cli::parse();

    let path = PathBuf::from(matches.path.unwrap_or(fs_utils::get_cwd()));

    if !fs_utils::is_file(&path) {
        eprintln!("Sorry, the given path is either not available or it is not a directory.");
        exit(1);
    }

    let config = config::ServConfig { path };

    rkt::build()
        .attach(fairings::insert_config(config.clone()))
        .attach(static_resources_initializer!("/favicon.ico" => "favicon.ico"))
        .register("/", rkt::catchers![not_found])
        .mount("/", rkt::routes![index])
        .mount("/", FileServer::new(config.path, Options::NormalizeDirs | Options::DotFiles))

        .mount(
            "/assets",
            FileServer::from(relative_path!("assets")).rank(11),
        )
}
