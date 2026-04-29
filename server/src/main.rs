use crate::{
    config::AppConfig,
    routes::{api_routes, static_routes},
};
use actix_web::{App, HttpServer, middleware::Logger, web};
mod config;
mod handler;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init();
    let app_state = web::Data::new(config.state);

    // Initial HTTP workers to handle inncomming TCP connections.
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::new("%a\t | %s\t | %Dms\t | %r\t"))
            .configure(api_routes)
            .configure(static_routes)
    })
    .workers(config.n_worker)
    .backlog(config.n_queue)
    .bind((config.ip, config.port))?
    .run()
    .await
}
