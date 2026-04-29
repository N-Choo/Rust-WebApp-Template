use crate::{
    config::AppState, handler::get_auth_token, models::user::UserProfile,
    services::graph::GraphService,
};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use log::error;
use serde::Serialize;
pub struct GraphHandler;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl GraphHandler {
    pub async fn get_me(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
        let access_token = match get_auth_token(&req, &state) {
            Ok(t) => t,
            Err(e) => return e,
        };

        match GraphService::fetch_profile(&access_token).await {
            Ok(profile) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: profile,
            }),
            Err(e) => {
                error!("Failed to fetch_profile: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    data: "Failed to fetch profile from Microsoft Graph".to_string(),
                })
            }
        }
    }

    pub async fn get_users(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
        let access_token = match get_auth_token(&req, &state) {
            Ok(t) => t,
            Err(e) => return e,
        };

        match GraphService::fetch_all_users(&access_token).await {
            Ok(profile) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: profile,
            }),
            Err(e) => {
                error!("Failed to fetch_profile: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    data: "Failed to fetch profile from Microsoft Graph".to_string(),
                })
            }
        }
    }

    pub async fn patch_me(
        req: HttpRequest,
        state: web::Data<AppState>,
        body: web::Json<UserProfile>,
    ) -> impl Responder {
        let access_token = match get_auth_token(&req, &state) {
            Ok(t) => t,
            Err(e) => return e,
        };

        match GraphService::update_profile(&access_token, &body.into_inner()).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: "Profile updated successfully".to_string(),
            }),
            Err(e) => {
                error!("Failed to update profile: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    data: format!("Failed to update Graph: {}", e),
                })
            }
        }
    }
}
