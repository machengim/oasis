pub mod file;
pub mod setup;
pub mod sys;
use rocket::Route;

pub fn serve_api() -> Vec<Route> {
    let mut apis = vec![];
    apis.append(&mut file::route());
    apis.append(&mut sys::route());
    apis.append(&mut setup::route());

    apis
}
