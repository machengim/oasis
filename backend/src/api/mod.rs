use rocket::Route;
pub mod sys;

pub fn serve() -> Vec<Route> {
    let mut apis = vec![];
    apis.append(&mut sys::route());

    apis
}
