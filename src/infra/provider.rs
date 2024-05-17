use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::{domain::repository::user::UserRepository, infra::db::launch_conn};

pub struct Provider {
    db_conn: Arc<DatabaseConnection>
}

impl Provider {
    pub async fn new() -> Self {
        println!("provider start!");
        let db_conn = Arc::new(launch_conn().await.expect("Connection error"));
        Provider{ db_conn }
    }

    pub fn provide_user_repository(&self) -> Arc<impl UserRepository> {
        println!("provide user repository, yes!");
        Arc::new(super::repository::user::UserRepositoryImpl{ conn: self.db_conn.clone()})
    }
}
