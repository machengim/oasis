use warp::{Filter, reply};
use sqlx::{Pool, Sqlite};
use crate::db;
use crate::entity::Config;

pub async fn run(pool: Pool<Sqlite>) {
    let config = read_config(&pool).await;

    warp::serve(get_config(config.clone()))
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
    -> impl Filter<Extract = (reply::Json,)> + Clone + Send + Sync + 'static {

    warp::path!("config")
    .map(move|| reply::json(&config))
}