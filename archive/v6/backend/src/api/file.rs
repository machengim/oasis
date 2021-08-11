use crate::entity::range::RangedFile;
use rocket::Route;
use std::path::PathBuf;

pub fn route() -> Vec<Route> {
    routes![get_file]
}

#[get("/file")]
async fn get_file() -> RangedFile {
    let path = PathBuf::from("/home/ma/Videos/lust.mp4");
    RangedFile { path }
}
