pub mod file;
pub mod login;
pub mod setup;
pub mod sys;
pub mod upload;
use rocket::Route;

pub fn serve_api() -> Vec<Route> {
    let mut apis = vec![];
    apis.append(&mut file::route());
    apis.append(&mut login::route());
    apis.append(&mut sys::route());
    apis.append(&mut setup::route());
    apis.append(&mut upload::route());

    apis
}
