use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::all::ComponentInteraction;

use crate::db_wrapper::DBWrapper;

use self::{menu::MenuMechanicsHandler, team::TeamMechanicsHandler};

pub mod menu;
pub mod team;

#[async_trait]
pub trait MechanicHandler: Send + Sync {
    async fn handle(&self, handler: MechanicHandlerWrapper);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MechanicFunction {
    Team(TeamMechanicsHandler),
    Menu(MenuMechanicsHandler),
}

pub struct MechanicHandlerWrapper {
    pub db: DBWrapper,
    pub interaction: ComponentInteraction,
}

impl MechanicFunction {
    pub async fn handle(&self, handler: MechanicHandlerWrapper) {
        match self {
            MechanicFunction::Team(team_mechanics_handler) => {
                team_mechanics_handler.handle(handler).await
            }
            MechanicFunction::Menu(menu_mechanics_handler) => {
                menu_mechanics_handler.handle(handler).await
            }
        }
    }
}
