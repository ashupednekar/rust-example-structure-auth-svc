use async_trait::async_trait;

use crate::{pkg::users::models::User, prelude::Result};

#[async_trait]
pub trait RegisterActions{
    async fn initiate_registration(&self) -> Result<()>;
    async fn verify_registration(&self, code: &str) -> Result<()>;
}


#[async_trait]
impl RegisterActions for User{
    async fn initiate_registration(&self) -> Result<()>{
        Ok(())
    }
    async fn verify_registration(&self, code: &str) -> Result<()>{
        Ok(())
    }
}
