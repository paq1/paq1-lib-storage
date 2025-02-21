use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub env: String,
    pub database: Database
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "dev".into());

        let config = Config::builder()
            .add_source(File::with_name("tests/integration_tests/config/default.toml"))
            .add_source(
                File::with_name(&format!("tests/integration_tests/config/{env}.toml"))
                    .required(true),
            )
            .set_override("env", env)?
            .build()?;

        config.try_deserialize()
    }


    pub fn unsafe_get_lazy() -> &'static Self {
        static INSTANCE: OnceLock<Settings> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            println!("init settings!");
            dotenv::dotenv().unwrap();
            Settings::new().expect("Failed to initialize settings")
        })
    }
}
