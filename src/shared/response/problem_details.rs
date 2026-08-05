use sea_orm::StringColumnNullable;
use serde::Serialize;


#[derive(Serialize)]
pub struct ProblemDetails {
    pub detail: String,
    pub errors: Vec<Field>,
}

#[derive(Serialize)]
pub struct Field {
    pub field: String,
    pub reason: String,
    pub code: String,
}

impl Field {
    pub fn new (field: &str, reason: &str, code: &str) -> Self{
        Self {
            field: field.into(),
            reason:reason.into(),
            code: code.into()
        }
    }
}