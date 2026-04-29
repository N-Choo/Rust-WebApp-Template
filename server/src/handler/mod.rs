use actix_web::{HttpRequest, HttpResponse, web};

use crate::{config::AppState, handler::graph::ApiResponse};

pub mod auth;
pub mod graph;

fn get_auth_token(req: &HttpRequest, state: &web::Data<AppState>) -> Result<String, HttpResponse> {
    let cookie = req.cookie("session_id").ok_or_else(|| {
        HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            data: "Missing session_id cookie".to_string(),
        })
    })?;

    let token = state.cache.get(cookie.value()).ok_or_else(|| {
        HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            data: "Session expired or invalid".to_string(),
        })
    })?;

    Ok(token)
}
