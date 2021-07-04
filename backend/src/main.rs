mod db;
mod entity;
mod utils;
use actix_files::{NamedFile, self};
use actix_web::dev::Server;
use actix_web::{rt::System, get, web, App, HttpResponse, HttpServer, Result};
use std::sync::mpsc::{self, Sender};
use std::{io, thread};
use std::sync::Mutex;
use sqlx::sqlite:: SqlitePool;
use sqlx::{Pool, Sqlite};
use tokio::runtime::Runtime;

#[macro_use]
extern crate log;

struct AppState {
    first_run: Mutex<bool>,
    pool: Mutex<SqlitePool>
}

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env("LOG_LEVEL").init();

    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || init_server(tx));

    // let server = rx.recv().unwrap();
    // thread::sleep(std::time::Duration::from_secs(5));
    // server.stop(true).await;

    if let Err(e) =  handle.join() {
        println!("Got error in thread joining: {:?}", e);
    }
}

fn init_server(tx: Sender<Server>) -> io::Result<()> {
    let system = System::new("http-server");
    let rt  = Runtime::new().unwrap();
    let state = web::Data::new(rt.block_on(init_app_state()));
    let react_dir = std::env::var("REACT_DIR").expect("Cannot get frontend dir from env");

    let server = HttpServer::new(move || {
        App::new().app_data(state.clone())
            .service(index)
            .service(actix_files::Files::new("/", react_dir.clone()).index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .shutdown_timeout(3)
    .run();

    let _ = tx.send(server);
    system.run()
}

async fn init_app_state() -> AppState {
    let pool = db::get_db_conn().await;
    let first_run = read_site(&pool).await.first_run == 1;

    AppState{first_run: Mutex::new(first_run), pool: Mutex::new(pool)}
}

async fn read_site(pool: &Pool<Sqlite>) -> entity::Site {
    let sql = "SELECT * FROM site";
    let args = vec![];

    match db::fetch_single::<entity::Site>(sql, args, pool).await {
        Ok(site) => site,
        Err(e) => panic!("Cannot read configuration: {}", e),
    }
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> HttpResponse {
    let first_run = state.first_run.lock().unwrap();
    if *first_run {
        HttpResponse::TemporaryRedirect().header("Location", "/setup/index.html").finish()
    } else {
        // HttpResponse::TemporaryRedirect().header("Location", "/index.html").finish()
        HttpResponse::Ok().finish()
    }
}