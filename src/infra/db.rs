use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use dotenv::dotenv;
use std::{env, time::Duration};

pub async fn launch_conn() -> Result<DatabaseConnection, sea_orm::DbErr> {
    dotenv().ok();
    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(db_url);

    let db_pool_max_size_str = &env::var("DB_POOL_MAX_SIZE").unwrap_or("5".to_string());
    let db_pool_max_size: u32 = db_pool_max_size_str.parse().unwrap();
    let db_pool_min_size_str = &env::var("DB_POOL_MIN_SIZE").unwrap_or("5".to_string());
    let db_pool_min_size: u32 = db_pool_min_size_str.parse().unwrap();
    let db_connect_timeout_str = &env::var("DB_CONNECT_TIMEOUT").unwrap_or("8".to_string());
    let db_connect_timeout: u64 = db_connect_timeout_str.parse().unwrap();
    let db_acquire_timeout_str = &env::var("DB_ACQUIRE_TIMEOUT").unwrap_or("8".to_string());
    let db_acquire_timeout: u64 = db_acquire_timeout_str.parse().unwrap();
    let db_idle_timeout_str = &env::var("DB_IDLE_TIMEOUT").unwrap_or("8".to_string());
    let db_idle_timeout: u64 = db_idle_timeout_str.parse().unwrap();
    let db_max_lifetime_str = &env::var("DB_MAX_LIFETIME").unwrap_or("8".to_string());
    let db_max_lifetime: u64 = db_max_lifetime_str.parse().unwrap();

    opt
        .max_connections(db_pool_max_size)
        .min_connections(db_pool_min_size)
        .connect_timeout(Duration::from_secs(db_connect_timeout))
        .acquire_timeout(Duration::from_secs(db_acquire_timeout))
        .idle_timeout(Duration::from_secs(db_idle_timeout))
        .max_lifetime(Duration::from_secs(db_max_lifetime))
        .sqlx_logging(true);

    Database::connect(opt).await
}
