use std::sync::Arc;

use f5a_services::{context::AppContext, routes};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

pub struct TestContext {
    pub db: Arc<DatabaseConnection>,
}

impl TestContext {
    pub async fn new() -> Self {
        let conn = Database::connect("sqlite::memory:").await.unwrap();

        Self { db: Arc::new(conn) }
    }

    pub async fn setup_db_schema(&self) -> &Self {
        let db_schema = sea_orm::Schema::new(self.db.get_database_backend());

        let stmt = db_schema.create_table_from_entity(schemas::user::Entity);

        self.db.execute(&stmt).await.unwrap();
        self
    }

    pub fn configure(&self) -> axum::Router {
        let ctx = AppContext {
            conn: Arc::clone(&self.db),
        };

        routes::router().with_state(ctx)
    }
}
