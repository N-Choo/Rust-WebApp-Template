use actix_web::{App, HttpServer, middleware::Logger};

use crate::{config::AppConfig, routes::static_routes};
mod config;
mod handler;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init();

    // Initial HTTP workers to handle inncomming TCP connections.
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a\t | %s\t | %Dms\t | %r\t"))
            .configure(static_routes)
    })
    .workers(config.n_worker)
    .backlog(config.n_queue)
    .bind((config.ip, config.port))?
    .run()
    .await
}
