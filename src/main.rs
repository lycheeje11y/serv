mod cli;
mod config;
mod fs_utils;

use rocket as rkt;
use std::borrow::Cow;
use std::ffi::OsStr;

use askama_rocket::Template;
use clap::Parser;
use rocket::{get, Config, Data, Request, Route};
use std::path::PathBuf;
use std::process::exit;

use crate::config::ServConfig;
use crate::fs_utils::get_parent_directory;
use rocket::fs::NamedFile;
use rocket::futures::lock::Mutex;
use rocket::http::ContentType;
use rocket::http::Method::Get;
use rocket::response::Redirect;
use rocket::route::{Handler, Outcome};
use rust_embed::RustEmbed;
use std::sync::Arc;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

#[derive(Template)]
#[template(path = "files.html")]
struct IndexTemplate {
    files: Vec<PathBuf>,
    directory: PathBuf,
    parent_directory: Option<PathBuf>,
    base_directory: PathBuf,
}

#[derive(Clone)]
struct CustomHandler();

#[rocket::async_trait]
impl Handler for CustomHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
        let config_binding = req.rocket().state::<Arc<Mutex<ServConfig>>>().unwrap();
        let mut config = config_binding.lock().await;
        let mut path = PathBuf::from(req.uri().to_string());

        let mut requested_path: PathBuf = config.base_directory.clone();
        let mut parent_directory: Option<PathBuf> = None;
        let mut files: Vec<PathBuf>;

        if path.starts_with("/") {
            path = path.strip_prefix("/").unwrap().to_path_buf();
        }

        if fs_utils::is_valid_subpath(&path, &config.base_directory) {
            requested_path = config.base_directory.join(path);

            if requested_path.is_dir() {
                parent_directory = get_parent_directory(&requested_path, &config.base_directory);
            } else if requested_path.is_file() {
                let file = NamedFile::open(requested_path).await;
                return Outcome::from(req, (ContentType::Binary, file));
            }
        }

        return if requested_path.exists() {
            files = fs_utils::walk_dir(&requested_path).unwrap();
            let template = IndexTemplate {
                files,
                directory: requested_path,
                parent_directory,
                base_directory: config.base_directory.clone(),
            };
            Outcome::from(req, template)
        } else {
            Outcome::from(req, Redirect::permanent("/"))
        };
    }
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
async fn launch() -> _ {
    let matches = cli::Cli::parse();

    let path = PathBuf::from(matches.path.unwrap_or(fs_utils::get_cwd()));
    let port = matches.port;

    if !fs_utils::is_file(&path) {
        eprintln!("Sorry, the given path is either not available or it is not a directory.");
        exit(1);
    }

    let config = ServConfig {
        base_directory: path.clone(),
        port,
    };

    rkt::build()
        .configure(Config::figment().merge(("port", config.port)))
        .configure(Config::figment().merge((Config::LOG_LEVEL, rkt::config::LogLevel::Off)))
        .manage(Arc::new(Mutex::new(config.clone())))
        //        .attach(fairings::download())
        .register("/", rkt::catchers![not_found, dotfile_attempt])
        .mount(
            "/",
            vec![Route::ranked(10, Get, "/<path..>", CustomHandler())],
        )
        .mount("/", rkt::routes![assets])
}
