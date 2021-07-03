use crate::entity::Site;
use crate::{api, db};
use sqlx::{Pool, Sqlite};
use warp::{http::Uri, Filter};

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
    let api = warp::path("api").and(api::get_system_volumes());
    let redirect = warp::path::end().map(|| warp::redirect::temporary(Uri::from_static("/setup")));
    let routes = redirect.or(react).or(api);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
