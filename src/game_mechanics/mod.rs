use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::db_wrapper::DBWrapper;

use self::{team::TeamMechanicsHandler, menu::MenuMechanicsHandler};

pub mod team;
pub mod menu;

#[async_trait]
pub trait MechanicHandler: Send + Sync {
    async fn handle(&self, db: DBWrapper);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MechanicFunction {
    Team(TeamMechanicsHandler),
    Menu(MenuMechanicsHandler),
}

impl MechanicFunction {
    pub async fn handle(&self, db: DBWrapper) {
        match self {
            MechanicFunction::Team(team_mechanics_handler) => team_mechanics_handler.handle(db).await,
            MechanicFunction::Menu(menu_mechanics_handler) => menu_mechanics_handler.handle(db).await,
        }
    }
}