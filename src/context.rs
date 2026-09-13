use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub conn: Arc<DatabaseConnection>,
}
