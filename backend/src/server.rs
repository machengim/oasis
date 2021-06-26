use crate::db;
use crate::entity::Site;
use sqlx::{Pool, Sqlite};
use warp::{http::Uri, reply, Filter};

pub async fn run(pool: Pool<Sqlite>) {
    let site = read_site(&pool).await;
    debug!("Get config config: {:?}", &site);
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
    let react = warp::fs::dir("../frontend/build");
    let redirect = warp::path::end().map(|| warp::redirect::see_other(Uri::from_static("/setup")));
    let routes = react.or(redirect);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
