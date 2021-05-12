use std::path::Path;
use rocket::{data::ByteUnit, http::{Header, Status}, response::{self, NamedFile, Responder, Response}};
use rocket::Request;
use rocket::http::ContentType;
use rocket::http::hyper::header::{ACCEPT_RANGES, RANGE};
use futures::executor::block_on;
use rocket::response::stream::ByteStream;

pub struct Media{
    pub path: String,
}



impl<'a, 'b: 'a> Responder<'a, 'b> for Media {
    fn respond_to(self, request: &'a Request<'_>) -> response::Result<'b> {
        let size = std::fs::metadata(Path::new(&self.path)).unwrap().len();
        let range = request.headers().get_one("Range");
        if let Some(v) = range {
            println!("get range: {}", v);
        }

        let content = block_on(NamedFile::open(Path::new(&self.path))).unwrap().take_file();
                        
        Response::build()
            .raw_header("Content-Type", "video/mp4")
            .raw_header("Accept-Ranges", "bytes")
            .raw_header("Connection", "keep-alive")
            .raw_header("keep-alive", "timeout=5")
            .status(Status::PartialContent)
            .sized_body(size as usize, content)
            .ok()
    }
}