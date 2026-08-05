use std::borrow::Cow;

use validator::ValidationError;

const MIN_PASSWORD_LENGTH: usize = 12;

pub fn password_strength(password: &str) -> Result<(), ValidationError> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ValidationError::new("password_strength")
            .with_message(Cow::from("password must be at least 12 characters long")));
    }

    Ok(())
}
