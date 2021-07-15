use crate::entity::AuthIndex;
use crate::util;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::Route;
use std::path::PathBuf;

pub fn serve_route() -> Vec<Route> {
    routes![index, login, setup]
}

#[get("/")]
fn index(_auth: AuthIndex) -> Redirect {
    Redirect::to(uri!("/setup"))
}

#[get("/login")]
async fn login() -> Option<NamedFile> {
    get_react_index().await
}

#[get("/setup")]
async fn setup(_auth: AuthIndex) -> Option<NamedFile> {
    get_react_index().await
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

/*
struct ServerRoute;

#[rocket::async_trait]
impl Fairing for ServerRoute {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if request.method() == Method::Get {
            let request_url = request.uri().to_string();
            let target_uri = match &request_url[..] {
                "/" | "/index.html" => "/?page=home",
                "/setup" => "/?page=setup",
                "/login" => "/?page=login",
                _ => "",
            };

            if target_uri.len() > 0 {
                request.set_uri(Origin::parse(target_uri).unwrap());
                println!("New uri set: {}", request.uri());
            }
        }
    }
}
 */
