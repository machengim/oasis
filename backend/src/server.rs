use crate::entity::Site;
use crate::{db, filesystem};
use sqlx::{Pool, Sqlite};
use std::convert::TryFrom;
use std::io::ErrorKind;
use warp::hyper::StatusCode;
use warp::{http::Uri, reply, Filter};

pub async fn run(pool: Pool<Sqlite>) {
    let site = read_site(&pool).await;
    debug!("Get site info: {:?}", &site);
    if site.first_run == 1 {
        run_setup_server().await;
    }
}

async fn read_site(pool: &Pool<Sqlite>) -> Site {
    let sql = "SELECT * FROM site";
    let args = vec![];

    match db::fetch_single::<Site>(sql, args, pool).await {
        Ok(site) => site,
        Err(e) => panic!("Cannot read configuration: {}", e),
    }
}

async fn run_setup_server() {
    let react_dir = std::env::var("REACT_DIR").expect("Cannot get frontend dir from env");
    let react = warp::fs::dir(react_dir);
    let redirect = warp::path::end().map(|| warp::redirect::temporary(Uri::from_static("/setup")));
    let routes = redirect.or(react).or(api_get_volumes());

    let route = warp::path("api")
    .map(|| {
        let a = filesystem::get_system_volumes();
        if let Ok(v) = a {
            Ok(v)
        } else {
            Err("")
        }
    });

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

fn api_get_volumes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let route = warp::path("api")
    .map(|| Response{value:filesystem::get_system_volumes() } 
    );

    return route;
}

struct Response {
    value: anyhow::Result<Vec<String>>
}

impl warp::Reply for Response {
    fn into_response(self) -> warp::reply::Response {
        match self.value {
            Ok(volumes) => warp::http::Response::new(format!("message: {:?}", volumes).into()),
            Err(e) => warp::http::Response::new(format!("error: {:?}", e).into())
        }
    }
}
