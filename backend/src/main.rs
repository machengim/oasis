mod entity;
mod db;
mod utils;

#[macro_use] extern crate log;
use entity::{Config, Language};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env("LOG_LEVEL").init();

    let dir = utils::get_config_dir();
    debug!("Using directory: {:?}", &dir);

    let pool = db::get_db_conn(&dir).await?;
    let sql = "SELECT * FROM lang WHERE id=? AND code=?";
    let args = "1, 'en'";
    let row: Language = sqlx::query_as(sql)
        .bind(args)
        .fetch_one(&pool).await?;
    println!("{:?}", &row);
    Ok(())
}

