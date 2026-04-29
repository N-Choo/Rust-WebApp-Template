use std::env;

use crate::config::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
pub struct Auth {}
use awc::cookie::{Cookie, SameSite};
use log::error;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse, reqwest};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AuthCallbackParams {
    pub code: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
    pub state: String,
}

impl Auth {
    pub async fn login(state: web::Data<AppState>) -> impl Responder {
        let scope = env::var("OAUTH_SCOPE").expect("FATAL: Missing OAUTH_SCOPE in env");
        let (auth_url, csrf_token) = state
            .oauth
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(scope))
            .url();

        state.cache.insert(csrf_token.into_secret(), "init".into());
        HttpResponse::Found()
            .append_header(("Location", auth_url.to_string()))
            .finish()
    }

    pub async fn logout(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
        let tenant = std::env::var("TENANT_ID").unwrap_or_else(|_| "common".to_string());
        let logout_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/logout?",
            tenant
        );

        // Clear cookies & session
        if let Some(cookie) = req.cookie("session_id") {
            let session_id = cookie.value();
            state.cache.remove(session_id);
        }

        let clear_session = Cookie::build("session_id", "")
            .path("/")
            .max_age(actix_web::cookie::time::Duration::ZERO)
            .finish();

        HttpResponse::Found()
            .append_header(("Location", logout_url))
            .cookie(clear_session)
            .finish()
    }

    pub async fn callback(
        state: web::Data<AppState>,
        params: web::Query<AuthCallbackParams>,
    ) -> impl Responder {
        // security reasons state code must be stored in cache.
        if let None = state.cache.get(&params.state) {
            return HttpResponse::BadRequest().finish();
        }

        if let Some(err) = &params.error {
            error!("MS Error: {} - {:?}", err, params.error_description);
            return HttpResponse::Unauthorized()
                .append_header(("Location", "/#/restricted"))
                .finish();
        }

        // Following redirects opens the client up to SSRF vulnerabilities.
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = state
            .oauth
            .exchange_code(AuthorizationCode::new(params.code.clone().unwrap()))
            .request_async(&http_client)
            .await;

        match token_result {
            Ok(token) => {
                state.cache.invalidate(&params.state);

                let session_id = Uuid::new_v4().to_string();
                let access_token = token.access_token().secret();
                let session_cookie = Cookie::build("session_id", session_id.clone())
                    .path("/")
                    .http_only(true)
                    .same_site(SameSite::Lax)
                    .finish();

                state.cache.insert(session_id.clone(), access_token.into());
                return HttpResponse::Found()
                    .cookie(session_cookie)
                    .append_header(("Location", "/"))
                    .finish();
            }
            Err(e) => {
                error!("Failed to exchange token: {:?}", e);
                return HttpResponse::InternalServerError()
                    .append_header(("Location", "/500"))
                    .finish();
            }
        }
    }
}
