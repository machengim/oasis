#[macro_use] extern crate rocket;
use std::path::Path;
use rocket::{Request, Response, fairing::{Fairing, Info, Kind}, http::Header, response::NamedFile};

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello,  {}!", name)
}

#[get("/movie")]
async fn movie() -> Option<NamedFile> {
    let filename = "/home/ma/Dropbox/channel/ep04/ep04.mp4";
    NamedFile::open(Path::new(filename)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/hello", routes![hello])
    .mount("/", routes![movie])
    .attach(CORS)
}

pub struct CORS;

#[async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Accept-Ranges", "bytes"));
    }
}