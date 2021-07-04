use actix_web::dev::Server;
use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use std::sync::mpsc::{self, Sender};
use std::{io, thread};

#[actix_web::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || run_sys(tx));
    let server = rx.recv().unwrap();

    println!("Running...");
    thread::sleep(std::time::Duration::from_secs(5));
    println!("stopping server");
    server.stop(true).await;
    println!("stopped");
    thread::sleep(std::time::Duration::from_secs(10));
}

fn run_sys(tx: Sender<Server>) -> io::Result<()> {
    let sys = System::new("http-server");

    let srv = HttpServer::new(|| {
        App::new().route(
            "/",
            web::get().to(|| HttpResponse::Ok().body("Hello world")),
        )
    })
    .bind("127.0.0.1:8080")?
    .shutdown_timeout(3)
    .run();

    let _ = tx.send(srv);
    sys.run()
}
