use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Status};
use rocket::{Request, Response};

use crate::util::constants;

pub struct StaticFileCache;

#[rocket::async_trait]
impl Fairing for StaticFileCache {
    fn info(&self) -> Info {
        Info {
            name: "Static file cache",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.status() != Status::Ok {
            return;
        }

        if request.method() == Method::Get && request.uri().path().starts_with("/api/file/") {
            let content = format!("private, max-age={}", constants::CACHE_MAX_AGE);
            response.set_raw_header("Cache-Control", content);
        }
    }
}
