use crate::db;
use crate::entity::auth::{Db, FirstRun};
use crate::entity::request::SetupRequest;
use crate::entity::site::{self, AppState};
use crate::entity::user;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn route() -> Vec<Route> {
    routes![post_setup_first_run, post_setup_default]
}

// TODO: create a dir at the setting location
#[post("/setup", rank = 1, data = "<setup_req>")]
async fn post_setup_first_run(
    setup_req: Json<SetupRequest>,
    _auth: FirstRun,
    mut db: Db,
    state: &State<AppState>,
) -> Result<(), Status> {
    let error409 = Err(Status::Conflict);
    let error500 = Err(Status::InternalServerError);

    match user::find_exist_username(&setup_req.username, &mut db.conn).await {
        Err(_) => return error500,
        Ok(true) => return error409,
        Ok(false) => (),
    };

    let insert_user = match user::insert_user_sql(&setup_req.username, &setup_req.password, 9) {
        Ok(query) => query,
        Err(_) => return error500,
    };
    let update_site = site::setup_site_sql(&setup_req.storage);

    if let Err(e) = db::tx_execute(vec![insert_user, update_site], &mut db.conn).await {
        eprintln!("{}", e);
        return error500;
    }

    let mut first_run = state.first_run.lock().unwrap();
    *first_run = false;

    Ok(())
}

#[post("/setup", rank = 2)]
fn post_setup_default() -> Status {
    Status::BadRequest
}
