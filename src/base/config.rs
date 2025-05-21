use deadpool_postgres::{Config, ManagerConfig, PoolConfig, RecyclingMethod, Runtime};
use std::env;

pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {        
        dotenv::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
        })
    }

    pub fn pg_pool(&self) -> deadpool_postgres::Pool {
        let mut cfg = Config::new();
        cfg.url = Some(self.database_url.clone());
        cfg.pool = Some(PoolConfig::new(20));
        
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        
        cfg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
            .expect("Failed to create pool")
    }
}
