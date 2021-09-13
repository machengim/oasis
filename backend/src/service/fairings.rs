use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};

pub struct CacheFairing;

#[rocket::async_trait]
impl Fairing for CacheFairing {
    fn info(&self) -> Info {
        Info {
            name: "GET Cache",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.status() != Status::Ok {
            return;
        }

        if request.method() == Method::Get && request.uri().path().starts_with("/api/file/") {
            let content = format!("private, max-age={}", 3600 * 24 * 3);
            response.set_raw_header("Cache-Control", content);
        }
    }
}