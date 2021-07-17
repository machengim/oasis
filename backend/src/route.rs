use crate::entity::auth::FirstRun;
use crate::util;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::Route;
use std::path::PathBuf;

pub fn serve_static_route() -> Vec<Route> {
    routes![
        get_index_first_run,
        get_index_default,
        get_login,
        get_setup_first_run,
        get_setup_default
    ]
}

#[get("/", rank = 1)]
fn get_index_first_run(_auth: FirstRun) -> Redirect {
    Redirect::to(uri!("/setup"))
}

// TODO: check user login info
#[get("/", rank = 2)]
async fn get_index_default() -> Option<NamedFile> {
    get_react_index().await
}

#[get("/login")]
async fn get_login() -> Option<NamedFile> {
    get_react_index().await
}

#[get("/setup", rank = 1)]
async fn get_setup_first_run(_auth: FirstRun) -> Option<NamedFile> {
    get_react_index().await
}

#[get("/setup", rank = 2)]
async fn get_setup_default() -> Status {
    Status::BadRequest
}

async fn get_react_index() -> Option<NamedFile> {
    let react_dir = util::get_react_dir();
    let mut path = PathBuf::from(react_dir);
    path.push("index.html");
    NamedFile::open(path).await.ok()
}

/* Use in production mode.
struct CacheContent;

#[rocket::async_trait]
impl Fairing for CacheContent {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Get {
            response.set_raw_header("Cache-control", "private");
            response.set_raw_header("Cache-control", "max-age=3600");
            println!("------ Header set --------");
        }
    }
}
*/
