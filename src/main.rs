mod cli;
mod config;
mod fairings;
mod fs_utils;
mod send_file;

use rocket as rkt;
use std::borrow::Cow;
use std::ffi::OsStr;

use askama_rocket::Template;
use clap::Parser;
use rocket::{get, route, Data, Request, Route, State};
use std::path::PathBuf;
use std::process::exit;

use crate::config::ServConfig;
use rocket::http::uri::fmt::Path;
use rocket::http::uri::Segments;
use rocket::http::ContentType;
use rocket::http::Method::Get;
use rust_embed::RustEmbed;
use std::sync::{Arc, Mutex};

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

#[derive(Template)]
#[template(path = "files.html")]
struct IndexTemplate {
    files: Vec<PathBuf>,
    start_directory: PathBuf,
    message: String,
}
/*
#[get("/")]
fn index(config_guard: &State<Arc<Mutex<ServConfig>>>) -> IndexTemplate {
    let config = config_guard.lock().unwrap();
    let mut files = fs_utils::walk_dir(&config.base_directory).unwrap();

    files.sort();
    IndexTemplate {
        files: files.into_iter().map(PathBuf::from).collect(),
        start_directory: config.start_directory.clone(),
        message: "hello".to_string()
    }
}
*/

fn handler<'r>(req: &'r Request, _: Data<'r>) -> route::BoxFuture<'r> {
    let config_binding = req.rocket().state::<Arc<Mutex<ServConfig>>>().unwrap();
    let mut config = config_binding.lock().unwrap();
    let mut files = fs_utils::walk_dir(&config.base_directory).unwrap();
    let uri = req.uri().path();
    let relative_requested_path = PathBuf::from(req.uri().to_string()).strip_prefix("/").unwrap().to_path_buf();
    let requested_path = &config.base_directory.clone().join(&relative_requested_path);
    let mut message: String = "no".to_string();

    if uri.segments().len() != 0 {
        if files.contains(requested_path) {
            if requested_path.is_dir() {
                config.base_directory.push(requested_path);
            } else {

            }
        }
    }

    files.sort();
    let template = IndexTemplate {
        files: files.into_iter().map(PathBuf::from).collect(),
        start_directory: config.start_directory.clone(),
        message,
    };
    route::Outcome::from(req, template).pin()
}

#[get("/assets/<file..>")]
fn assets(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Plain);

    Some((content_type, asset.data))
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
    let port = matches.port;

    if !fs_utils::is_file(&path) {
        eprintln!("Sorry, the given path is either not available or it is not a directory.");
        exit(1);
    }

    let config = ServConfig {
        base_directory: path.clone(),
        start_directory: path,
        port,
    };

    rkt::build()
        .configure(rocket::Config::figment().merge(("port", config.port)))
        .manage(Arc::new(Mutex::new(config.clone())))
        .attach(fairings::download())
        .register("/", rkt::catchers![not_found, dotfile_attempt])
        .mount("/", vec![Route::ranked(10, Get, "/<path..>", handler)])
        .mount("/", rkt::routes![assets])
}
