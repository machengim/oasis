use crate::entity::error::Error;
use anyhow::Result as AnyResult;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::Response;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use std::io::SeekFrom;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, Take};

pub struct Range {
    pub range: Option<(u64, u64)>,
}

pub struct RangedFile {
    path: PathBuf,
    size: u64,
    start: u64,
    end: u64,
    take: Take<File>,
}

impl RangedFile {
    pub async fn new(range: (u64, u64), path: PathBuf) -> AnyResult<Self> {
        let (start, mut end) = range;
        let mut file = File::open(&path).await?;
        let size = file.metadata().await?.len();
        if end == 0 {
            end = size - 1;
        }

        file.seek(SeekFrom::Start(start)).await?;
        let take = file.take(end - start + 1);

        Ok(RangedFile {
            path,
            size,
            start,
            end,
            take,
        })
    }

    fn get_content_type(&self) -> ContentType {
        if let Some(ext) = self.path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if let Some(content_type) = ContentType::from_extension(ext_str) {
                    return content_type;
                }
            }
        }

        ContentType::Binary
    }

    fn get_content_range(&self) -> String {
        format!("bytes {}-{}/{}", self.start, self.end, self.size)
    }

    fn get_content_len(&self) -> String {
        (self.end - self.start + 1).to_string()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Range {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let bad_request = (Status::BadRequest, Error::BadRequest);
        let range_header = req.headers().get_one("Range");

        match range_header {
            Some(range_str) => {
                let parts: Vec<&str> = range_str.split(|c| c == '=' || c == '-').collect();

                // start value must be valid.
                let start: u64 = match parts.get(1) {
                    Some(v) => v.parse().unwrap(),
                    None => return Outcome::Failure(bad_request),
                };

                // end value could be empty, but must not be smaller than start value.
                let end: u64 = parts.get(2).unwrap_or(&"0").parse().unwrap_or(0);
                if end < start && end != 0 {
                    return Outcome::Failure(bad_request);
                }

                Outcome::Success(Range {
                    range: Some((start, end)),
                })
            }
            None => Outcome::Success(Range { range: None }),
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for RangedFile {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        Response::build()
            .status(Status::PartialContent)
            .header(self.get_content_type())
            .raw_header("Content-Range", self.get_content_range())
            .raw_header("Content-Length", self.get_content_len())
            .streamed_body(self.take)
            .ok()
    }
}
