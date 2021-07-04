use std::convert::Infallible;

use crate::{db, filesystem};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use warp::{reply::Reply, Filter};

#[derive(Serialize, Clone)]
pub struct Response<T> {
    pub data: Option<T>,
    pub error: Option<Error>,
}

#[derive(Serialize, Clone)]
pub struct Error {
    pub code: i16,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
}

impl<T> Response<T> {
    fn new() -> Self {
        Response {
            data: None,
            error: None,
        }
    }

    fn from_anyhow_result(input: anyhow::Result<T>) -> Self {
        let mut response = Response::new();

        match input {
            Ok(v) => response.data = Some(v),
            Err(e) => {
                debug!("Get error: {}", e);
                response.error = Some(Error {
                    code: 500,
                    message: "Internal server error".to_string(),
                })
            }
        }

        response
    }
}

pub fn get_system_volumes() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone
{
    let result = filesystem::get_system_volumes();
    let response = Response::from_anyhow_result(result);

    warp::path!("fs" / "volumes").map(move || warp::reply::json(&response))
}

pub fn post_server_setup(
    pool: &'static Pool<Sqlite>,
) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path("setup")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 4))
        .and(warp::body::json())
        .and(with_pool(pool))
        .and_then(|setup: SetupRequest, pool: Pool<Sqlite>| async move {
            let result = init_server_setup(setup, &pool).await;
            let response = Response::from_anyhow_result(result);

            Ok::<_, warp::Rejection>(warp::reply::json(&response))
        })
}

fn with_pool(
    pool: &'static Pool<Sqlite>,
) -> impl Filter<Extract = (Pool<Sqlite>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn init_server_setup(setup: SetupRequest, pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let tx = pool.begin().await?;
    let sql = "INSERT INTO user(username, password, permission) VALUES($1, $2, $3)";
    let args = vec![setup.username, setup.password, "99".to_string()];
    db::execute(sql, args, pool).await?;
    tx.commit().await?;

    Ok(())
}
