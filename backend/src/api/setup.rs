use crate::db;
use crate::entity::auth::{Db, FirstRun};
use crate::entity::request::SetupRequest;
use crate::entity::site::{AppState, Query};
use bcrypt::{hash, DEFAULT_COST};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn route() -> Vec<Route> {
    routes![post_setup_first_run, post_setup_default]
}

// // TODO: unifinished.
#[post("/setup", rank = 1, data = "<setup_req>")]
async fn post_setup_first_run(
    setup_req: Json<SetupRequest>,
    _auth: FirstRun,
    mut db: Db,
    state: &State<AppState>,
) -> Result<(), Status> {
    let encrypt_password = match hash(&setup_req.password, DEFAULT_COST) {
        Ok(v) => v,
        Err(_) => return Err(Status::InternalServerError),
    };

    let query = Query::new(
        "insert into USER (username, password) values (?1, ?2)",
        vec![setup_req.username.clone(), encrypt_password],
    );

    if let Err(e) = db::execute(query, &mut db.conn).await {
        eprintln!("{}", e);
    }

    let mut first_run = state.first_run.lock().unwrap();
    *first_run = false;

    Ok(())
}

#[post("/setup", rank = 2)]
fn post_setup_default() -> Status {
    Status::BadRequest
}
