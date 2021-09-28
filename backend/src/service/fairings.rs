use crate::util::constants;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Status};
use rocket::{Request, Response};

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

        if request.method() == Method::Get && allow_cache(request) {
            let content = format!("private, max-age={}", constants::CACHE_MAX_AGE);
            response.set_raw_header("Cache-Control", content);
        }
    }
}

// Do not cache development related files in debug mode.
#[cfg(debug_assertions)]
fn allow_cache<'r>(req: &'r Request<'_>) -> bool {
    let req_path = req.uri().path();

    if req_path.starts_with("/api/file/") {
        return true;
    }

    false
}

#[cfg(not(debug_assertions))]
fn allow_cache<'r>(req: &'r Request<'_>) -> bool {
    let req_path = req.uri().path();

    if req_path.starts_with("/api/file/") {
        return true;
    }

    for ext in constants::CACHE_FILE_EXTS.iter() {
        if req_path.ends_with(ext) {
            return true;
        }
    }

    false
}
