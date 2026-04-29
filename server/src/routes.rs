use actix_files as fs;
use actix_web::web::{self, patch};

use crate::handler::{auth::Auth, graph::GraphHandler};

/// Serve static files for frontend.
pub fn static_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        fs::Files::new("/", "../client")
            .index_file("index.html")
            .show_files_listing(),
    );
}

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/login", web::get().to(Auth::login))
                    .route("/callback", web::get().to(Auth::callback))
                    .route("/logout", web::get().to(Auth::logout)),
            )
            // --- Microsoft Graph Proxy Routes ---
            .service(
                web::scope("/graph")
                    .route("/me", web::get().to(GraphHandler::get_me))
                    .route("/users", web::get().to(GraphHandler::get_users))
                    .route("/update", patch().to(GraphHandler::patch_me)),
            ),
    );
}
