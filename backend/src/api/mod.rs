use rocket::Route;
mod files;
mod sys;
mod upload;
mod user;

pub fn serve() -> Vec<Route> {
    let mut apis = vec![];
    apis.append(&mut sys::route());
    apis.append(&mut user::route());
    apis.append(&mut files::route());
    apis.append(&mut upload::route());

    apis
}
