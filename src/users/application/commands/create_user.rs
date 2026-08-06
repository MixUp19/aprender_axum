
#[derive(Validate)]
pub struct CreateUserCommand {
    #[validate(length(
        min = 1,
        max = 200,
        message = "name must be between 1 and 200 characters"
    ))]
    pub full_name: String,

    #[validate(email(message = "email address is not valid"))]
    pub email: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "username must be between 3 and 100 characters"
    ))]
    pub username: String,

    #[validate(url(message = "website url is not valid"))]
    pub website: String,

    #[validate(range(min=18, max = 100, message = "age must be between 18 and 100"))]
    pub age: u8,

    #[validate(custom(function = "crate::validators::password_strength"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "password do not match"))]
    pub confirm_password: String,
}

pub struct CreateUserCommandHandler {
    pub conn: Arc<DatabaseConnection>,
}

impl CreateUserCommandHandler {
    pub async fn handler(&self, command:CreateUserCommand) -> Result<i32, ApiError> {
        command.validate()?;

        let model = schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(command.username),
        full_name: ActiveValue::Set(command.full_name),
        password: ActiveValue::Set("1234".into()),
        disabled: ActiveValue::Set(true),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        creator_id: ActiveValue::Set(1),
    }
    .insert(self.conn.as_ref())
    .await?;
    }

    Ok(model.id)
}