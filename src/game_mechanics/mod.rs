use async_trait::async_trait;

use crate::db_wrapper::DBWrapper;

pub mod team;

#[async_trait]
pub trait MechanicHandler: Send + Sync {
    async fn handle(&self, db: DBWrapper);
}
