use std::env;
use std::io::Write;

use dotenvy::dotenv;
use moka::sync::Cache;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};

pub type AppClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Clone)]
pub struct AppState {
    pub oauth: AppClient,
    pub cache: Cache<String, String>,
}

pub struct AppConfig {
    pub ip: String,
    pub port: u16,
    pub n_worker: usize,
    pub n_queue: u32,
    pub state: AppState,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv().ok();
        // Initialize Logger.
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .write_style(env_logger::WriteStyle::Always)
            .format(|buf, record| {
                let style = buf.default_level_style(record.level());

                writeln!(
                    buf,
                    "[{}] [{style}{level:^9}] [{:^20}]{style:#} {}",
                    buf.timestamp(),
                    record.target().split("::").last().unwrap_or("SERVER"),
                    record.args(),
                    style = style,
                    level = record.level(),
                )
            })
            .init();

        let host = env::var("HOST").expect("FATAL: Missing HOST in env");
        let client_id = env::var("CLIENT_ID").expect("FATAL: Missing CLIENT_ID in env");
        let client_secret = env::var("CLIENT_SECRET").expect("FATAL: Missing CLIENT_SECRET in env");
        let auth_url = env::var("AUTH_URL").expect("FATAL: Missing AUTH_URL in env");
        let token_url = env::var("TOKEN_URL").expect("FATAL: Missing TOKEN_URL in env");
        let callback_url = format!("{}/api/auth/callback", host);

        let oauth_client = BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(AuthUrl::new(auth_url).unwrap())
            .set_token_uri(TokenUrl::new(token_url).unwrap())
            .set_redirect_uri(RedirectUrl::new(callback_url).unwrap());

        let session_cache: Cache<String, String> = Cache::builder()
            .max_capacity(250)
            .time_to_live(std::time::Duration::from_secs(60 * 60))
            .build();

        let state = AppState {
            oauth: oauth_client,
            cache: session_cache,
        };

        // Load Configuration
        Self {
            ip: "127.0.0.1".into(),
            port: env::var("TS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            n_worker: 2,
            n_queue: num_cpus::get() as u32,
            state,
        }
    }
}
