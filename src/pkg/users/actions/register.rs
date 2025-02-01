use async_trait::async_trait;

use crate::{pkg::users::models::User, prelude::Result};

#[async_trait]
trait RegisterActions{
    async fn send_conformation_email(&self) -> Result<()>;
    async fn verify_conformation_email(&self, code: &str) -> Result<()>;
}


#[async_trait]
impl RegisterActions for User{
    async fn send_conformation_email(&self) -> Result<()>{
        Ok(())
    }
    async fn verify_conformation_email(&self, code: &str) -> Result<()>{
        Ok(())
    }
}
