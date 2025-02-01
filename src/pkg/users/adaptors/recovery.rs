use async_trait::async_trait;

use crate::{pkg::users::models::User, prelude::Result};

#[async_trait]
pub trait RecoveryActions{
    async fn initiate(&self) -> Result<()>;
    async fn verify(&self, code: &str) -> Result<()>;
}

#[async_trait]
impl RecoveryActions for User{
    async fn initiate(&self) -> Result<()>{
        Ok(())
    }
    async fn verify(&self, code: &str) -> Result<()>{
        Ok(())
    }
}
