mod cli;
mod config;
mod fs_utils;
mod fairings;

use rocket as rkt;

use askama_rocket::Template;
use clap::Parser;
use rocket::{State, uri};
use std::path::PathBuf;
use std::process::exit;

use rocket::fs::{FileServer, Options};
use std::sync::{Arc, Mutex};
use rocket::response::Redirect;
use rocket_include_static_resources::manifest_dir_macros::relative_path;

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

fn redirect_to_download(path: PathBuf) -> Redirect {
    println!("Path: {}", path.display());
    if path.display().to_string().trim() == "favicon.io".to_string() {
        return Redirect::to(uri!("/favicon.ico"));
    }
    Redirect::to(format!("/files/{}", path.display()))
}

#[rkt::get("/<path..>", rank = 13)]
fn file_redirect(path: PathBuf) -> Redirect {
    redirect_to_download(path)
}

#[rkt::catch(404)]
fn not_found() -> &'static str {
    "404 Not Found"
}

#[rkt::catch(422)]
fn dotfile_attempt() -> &'static str {
    "Sorry, dotfiles are not supported yet. Either rename the file on the host system or go directly to /files/<dotfile-name>"
}

#[rkt::launch]
fn launch() -> _ {
    let matches = cli::Cli::parse();

    let path = PathBuf::from(matches.path.unwrap_or(fs_utils::get_cwd()));

    if !fs_utils::is_file(&path) {
        eprintln!("Sorry, the given path is either not available or it is not a directory.");
        exit(1);
    }

    let config = config::ServConfig { path, port: matches.port };

    rkt::build()
        .configure(rocket::Config::figment().merge(("port", config.port)))
        .attach(fairings::insert_config(config.clone()))
        .attach(fairings::download())
        .register("/", rkt::catchers![not_found, dotfile_attempt])
        .mount("/files", FileServer::new(config.path, Options::DotFiles))
        .mount("/favicon.ico", FileServer::new("favicon.ico", Options::IndexFile).rank(12))
        .mount("/assets", FileServer::from(relative_path!("assets")).rank(11))
        .mount("/", rkt::routes![index, file_redirect])
}
