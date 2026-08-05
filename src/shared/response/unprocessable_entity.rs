use std::borrow::Cow;

use axum::{
    Json,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use validator::{ValidationError, ValidationErrors};

use crate::shared::response::{Field, ProblemDetails, problem_details};

const INVALID_DEFAULT_MESSAGE: Cow<'static, str> = Cow::Borrowed("Invalid Information");

pub struct UnprocessableParams(pub ValidationErrors);

impl IntoResponse for UnprocessableParams {
    fn into_response(self) -> axum::response::Response {
        println!("validation errors: {}", self.0);

        let fields_with_errors = self.0.into_fields();
    
        let problem_details = ProblemDetails {
            detail: "validation failed".into(),
            errors: fields_with_errors,
        };

        (StatusCode::UNPROCESSABLE_ENTITY, Json(problem_details)).into_response()
    }
}

trait IntoFields {
    fn into_fields(self) -> Vec<Field>;
}

impl IntoFields for ValidationErrors {
    fn into_fields(self) -> Vec<Field> {
        let field_errors = self.field_errors();

        let mut fields = Vec::with_capacity(field_errors.len());

        fields.extend(field_errors.into_iter().map(|(field_name, errs)| {

          let error: &ValidationError = &errs[0];

          let field_message = error.message.as_ref().unwrap_or(&INVALID_DEFAULT_MESSAGE);

          Field::new(&field_name,field_message,&error.code)
        }));

        fields
    }
}
