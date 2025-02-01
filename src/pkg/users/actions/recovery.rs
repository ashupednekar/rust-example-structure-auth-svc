use async_trait::async_trait;

use crate::{pkg::users::models::User, prelude::Result};

#[async_trait]
pub trait RecoveryActions{
    async fn send_password_recovery_email(&self) -> Result<()>;
    async fn verify_password_recovery_email(&self, code: &str) -> Result<()>;
}

#[async_trait]
impl RecoveryActions for User{
    async fn send_password_recovery_email(&self) -> Result<()>{
        Ok(())
    }
    async fn verify_password_recovery_email(&self, code: &str) -> Result<()>{
        Ok(())
    }
}
