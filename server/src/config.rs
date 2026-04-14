use std::env;
use std::fmt;
use std::io::Write;

pub struct AppConfig {
    pub ip: String,
    pub port: u16,
    pub n_worker: usize,
    pub n_queue: u32,
}

impl AppConfig {
    pub fn init() -> Self {
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

        // Load Configuration
        Self {
            ip: "127.0.0.1".into(),
            port: env::var("TS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            n_worker: 2,
            n_queue: num_cpus::get() as u32,
        }
    }
}
