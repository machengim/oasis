use crate::entity::auth::AuthUser;
use rocket::data::{Data, ToByteUnit};
use rocket::fs::TempFile;
use rocket::Route;

pub fn route() -> Vec<Route> {
    routes![upload]
}

// TODO: auth user
#[post("/upload", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<(), std::io::Error> {
    // let filename = paste.name().unwrap_or("a.txt").to_owned();
    // println!("filename is {}", filename);

    // paste.persist_to(filename).await?;
    paste.open(2.gibibytes()).into_file("a.txt").await?;
    Ok(())
}
