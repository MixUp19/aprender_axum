pub struct PartialUpdateUser {
    pub id: i32,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub disabled: Option<bool>,
}