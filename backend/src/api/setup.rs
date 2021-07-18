use crate::db;
use crate::entity::auth::{Db, FirstRun};
use crate::entity::request::SetupRequest;
use crate::entity::site::{AppState, Query};
use crate::entity::user::UserId;
use bcrypt::{hash, DEFAULT_COST};
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
    let error500 = Err(Status::InternalServerError);

    let query_user_sql = Query::new(
        "select user_id from USER where username = ?1",
        vec![&setup_req.username],
    );

    if db::fetch_single::<UserId>(query_user_sql, &mut db.conn)
        .await
        .is_ok()
    {
        return Err(Status::Conflict);
    }

    let encrypt_password = match hash(&setup_req.password, DEFAULT_COST) {
        Ok(v) => v,
        Err(_) => return error500,
    };

    let insert_user_sql = Query::new(
        "insert into USER (username, password, permission) values (?1, ?2, ?3)",
        vec![&setup_req.username, &encrypt_password, "9"],
    );

    let update_site_sql = Query::new(
        "update SITE set first_run = ?1, storage = ?2",
        vec!["0", &setup_req.storage],
    );

    if let Err(e) = db::tx_execute(vec![insert_user_sql, update_site_sql], &mut db.conn).await {
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
