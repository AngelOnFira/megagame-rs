use entity::entities::message_component_data;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use serenity::builder::CreateButton;

use uuid::Uuid;

use crate::db_wrapper::DBWrapper;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponent<C: SerenityComponent> {
    pub component: C,
}

impl<C: SerenityComponent> MessageComponent<C> {
    // TODO: should this be refactored out of the type? More use shall tell.
    pub async fn new(data: Option<MessageData>, db: DBWrapper) -> Self {
        // Serialize the data
        let data = serde_json::to_string(&data).unwrap();

        // Add it to the database
        let database_data = message_component_data::ActiveModel {
            id_uuid: Set(Uuid::new_v4().to_string()),
            payload: Set(data),
            ..Default::default()
        }
        .insert(&*db)
        .await
        .unwrap();

        // Create the component with the id of the database entry
        let component = C::new_with_database_id(database_data.id_uuid);

        MessageComponent { component }
    }

    pub fn get_component(&self) -> &C {
        &self.component
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageData {}

pub trait SerenityComponent {
    fn new_with_database_id(id: String) -> Self;
}

impl SerenityComponent for CreateButton {
    fn new_with_database_id(id: String) -> Self {
        CreateButton::new(id)
    }
}
