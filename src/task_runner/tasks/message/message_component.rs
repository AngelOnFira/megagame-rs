use entity::entities::message_component_data;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use serenity::builder::{CreateButton, CreateSelectMenu};

use uuid::Uuid;

use crate::db_wrapper::DBWrapper;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponent<C: SerenityComponent> {
    pub component: C,
    pub data: Option<MessageData>,
}

impl<C: SerenityComponent> MessageComponent<C> {
    // TODO: should this be refactored out of the type? More use shall tell.
    pub async fn new(component: C, data: Option<MessageData>) -> Self {
        MessageComponent { component, data }
    }

    pub fn get_component(&self) -> &C {
        &self.component
    }

    pub async fn get_id(&self, db: DBWrapper) -> C {
        // Serialize the data
        let data = serde_json::to_string(&self.data).unwrap();

        // Add it to the database
        let database_data = message_component_data::ActiveModel {
            id_uuid: Set(Uuid::new_v4().to_string()),
            payload: Set(data),
            ..Default::default()
        }
        .insert(&*db)
        .await
        .unwrap();

        // Update the component with the id
        self.component.update_id(database_data.id_uuid)
    }
}

// pub async fn component_with_data<C: SerenityComponent>(
//     data: Option<MessageData>,
//     db: DBWrapper,
// ) -> C {
//     // Serialize the data
//     let data = serde_json::to_string(&data).unwrap();

//     // Add it to the database
//     let database_data = message_component_data::ActiveModel {
//         id_uuid: Set(Uuid::new_v4().to_string()),
//         payload: Set(data),
//         ..Default::default()
//     }
//     .insert(&*db)
//     .await
//     .unwrap();
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageData {}

pub trait SerenityComponent {
    fn update_id(self, id: String) -> Self;
}

impl SerenityComponent for CreateButton {
    fn update_id(self, id: String) -> Self {
        self.custom_id(id)
    }
}

impl SerenityComponent for CreateSelectMenu {
    fn update_id(self, id: String) -> Self {
        self.custom_id(id)
    }
}
