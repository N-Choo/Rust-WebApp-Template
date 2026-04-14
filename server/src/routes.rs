use actix_files as fs;
use actix_web::web::{self};

/// Serve static files for frontend.
pub fn static_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        fs::Files::new("/", "../client")
            .index_file("index.html")
            .show_files_listing(),
    );
}

// pub fn api_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         // Parent scope: All routes here start with "/api"
//         web::scope("/api").service(),
//     )
// }
