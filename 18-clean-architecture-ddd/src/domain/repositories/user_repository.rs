use crate::domain::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: String) -> Option<User>;

    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error>;
}
