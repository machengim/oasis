#[macro_use] extern crate rocket;
use std::{io::Read, path::Path};
use futures::Stream;
use rocket::{http::Status, response::stream::TextStream};
use rocket::{Request, Response, fairing::{AdHoc, Fairing, Info, Kind}, form::name::Name, http::{ContentType, Header}, response::NamedFile};
mod partial;
use partial::Media;
use rocket::response::stream::ByteStream;

#[derive(Responder)]
#[response(status=206, content_type="video/mp4")]
struct Video {
    file: Option<NamedFile>
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello,  {}!", name)
}

/*
#[get("/movie")]
async fn movie() -> Option<NamedFile> {
    let filename = "/home/ma/Downloads/lust.mp4";
    NamedFile::open(Path::new(filename)).await.ok()
} */

#[get("/movie")]
async fn movie() -> (Status, Media) {
    let filename = "/home/ma/Downloads/ep04.mp4";
    let media = Media{path: filename.to_string()};
    (Status::PartialContent, media)
}

#[get("/media")]
async fn media() -> TextStream![&'static str] {
    ByteStream!([0; 16])
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/hello", routes![hello])
    .mount("/", routes![movie])
    .attach(AdHoc::config::<AppConfig>())
    .attach(CORS)
}

pub struct CORS;

#[derive(Debug, serde::Deserialize)]
struct AppConfig{
    address: String
}

#[async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS and ACCEPT RANGE headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}