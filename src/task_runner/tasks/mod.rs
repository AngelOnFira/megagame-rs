use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{client::Context, model::id::UserId};

use self::{create_dropdown::CreateDropdown, message_user::MessageUser};

pub mod create_dropdown;
pub mod message_user;

/// Store the different tasks the bot can do in the database. Each variant has
/// its own struct that can store the rest of the data required for the task.
/// Each of these structs might have their own `impl`s to operate on the data.
#[derive(Serialize, Deserialize, Debug)]
pub enum TaskType {
    ChangeTeam { team_id: u64, user_id: UserId },
    CreateButtons,
    CreateCategory,
    CreateCategoryChannel,
    CreateDropdown(CreateDropdown),
    CreateMessage,
    CreateRole,
    CreateTeamChannel,
    CreateTeamVoiceChannel,
    CreateThread,
    MessageUser(MessageUser),
}

impl TaskType {
    pub fn route(&self) -> &dyn Task {
        match self {
            TaskType::CreateDropdown(create_dropdown) => create_dropdown,
            TaskType::MessageUser(message_user) => message_user,
            _ => unimplemented!(),
        }
    }
}

#[async_trait]
pub trait Task: Send {
    async fn handle(&self, ctx: Arc<Context>);
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Task {
//     pub task: TaskType,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRole {}

// impl Task {
//     pub async fn message_user(&self, ctx: Arc<Context>) {

//     }

//     pub async fn create_dropdown(&self, ctx: Arc<Context>) {
//         let dropdown = if let TaskType::CreateDropdown(dropdown) = &self.task {
//             dropdown
//         } else {
//             log::error!("Not a dropdown task");
//             return;
//         };

//         let message = ChannelId(dropdown.channel_id)
//             .send_message(ctx.http(), |m| {
//                 m.content("Hello, world!");
//                 m.components(|c| {
//                     c.add_action_row({
//                         let mut ar = CreateActionRow::default();
//                         ar.add_button({
//                             let mut b = CreateButton::default();
//                             b.label("test1");
//                             b
//                         });
//                         ar
//                     })
//                 })
//             })
//             .await
//             .unwrap();
//     }

//     pub async fn create_team_channel(&self, ctx: Arc<Context>) {
//         let team_channel = if let TaskType::CreateTeamChannel(team_channel) = &self.task {
//             team_channel
//         } else {
//             log::error!("Not a team channel task");
//             return;
//         };

// }
