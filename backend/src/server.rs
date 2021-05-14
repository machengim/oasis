use warp::{Filter, reply};
use sqlx::{Pool, Sqlite};
use crate::db;
use crate::entity::Config;

pub async fn run(pool: Pool<Sqlite>) {
    let config = read_config(&pool).await;
    debug!("Get config: {:?}", &config);

    let home_dir = warp::path("home").and(warp::fs::dir("../frontend/build"));

    let routes = get_config(config.clone())
        .or(home_dir)
        .with(add_cors());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

async fn read_config(pool: &Pool<Sqlite>) -> Config {
    let sql = "SELECT * FROM site";
    let args = vec![];

    match db::fetch_single::<Config>(sql, args, pool).await {
        Ok(config) => config,
        Err(e) => panic!("Cannot read configuration: {}", e)
    }
}

fn get_config(config: Config) 
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    
    warp::path!("config")
        .and(warp::get())
        .map(move|| reply::json(&config))
}

fn add_cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTION"])
}