use rocket::Route;
pub mod files;
pub mod sys;
pub mod user;

pub fn serve() -> Vec<Route> {
    let mut apis = vec![];
    apis.append(&mut sys::route());
    apis.append(&mut user::route());
    apis.append(&mut files::route());

    apis
}
