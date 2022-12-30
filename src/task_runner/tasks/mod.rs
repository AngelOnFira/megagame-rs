use std::{fmt::Debug, num::NonZeroU64, ops::Deref};

use async_trait::async_trait;

use sea_orm::ActiveModelTrait;
use serde::{Deserialize, Serialize};
use serenity::{
    all::{RoleId, UserId},
    client::Context,
    model::prelude::{ChannelId, GuildId},
};

use crate::db_wrapper::{DBWrapper, TaskResult};

use self::{
    category::CategoryHandler, channel::ChannelHandler, message::MessageHandler, role::RoleHandler,
    thread::ThreadHandler,
};

pub mod category;
pub mod channel;
pub mod message;
pub mod role;
pub mod test_helpers;
pub mod thread;

/// A wrapper for TaskType to store the id if the task in the database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbTask {
    pub id: i32,
    pub task: TaskType,
}

/// Store the different tasks the bot can do in the database. Each variant has
/// its own struct that can store the rest of the data required for the task.
/// Each of these structs might have their own `impl`s to operate on the data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    CategoryHandler(CategoryHandler),
    ChannelHandler(ChannelHandler),
    MessageHandler(MessageHandler),
    RoleHandler(RoleHandler),
    ThreadHandler(ThreadHandler),
}

impl TaskType {
    pub fn route(&self) -> &dyn Task {
        match self {
            TaskType::CategoryHandler(task_handler) => task_handler,
            TaskType::ChannelHandler(task_handler) => task_handler,
            TaskType::MessageHandler(task_handler) => task_handler,
            TaskType::RoleHandler(task_handler) => task_handler,
            TaskType::ThreadHandler(task_handler) => task_handler,
        }
    }
}

#[async_trait]
pub trait Task: Send + Sync {
    async fn handle(&self, ctx: Context, db: DBWrapper) -> TaskResult;
}

#[async_trait]
pub trait TaskTest: Send + Sync {
    async fn run_tests(ctx: Context, db: DBWrapper);
}

pub async fn run_tests(ctx: Context, db: DBWrapper) {
    CategoryHandler::run_tests(ctx, db).await;
}

pub fn assert_not_error<T>(result: Result<(), T>)
where
    T: Debug,
{
    match result {
        Ok(_) => {}
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DiscordId(pub u64);

impl Deref for DiscordId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// String
impl Into<String> for DiscordId {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl From<&String> for DiscordId {
    fn from(s: &String) -> Self {
        DiscordId(s.parse().unwrap())
    }
}

// NonZeroU64
impl From<NonZeroU64> for DiscordId {
    fn from(id: NonZeroU64) -> Self {
        DiscordId(id.get())
    }
}

impl Into<NonZeroU64> for DiscordId {
    fn into(self) -> NonZeroU64 {
        NonZeroU64::new(self.0).unwrap()
    }
}

// ChannelId
impl Into<ChannelId> for DiscordId {
    fn into(self) -> ChannelId {
        ChannelId(NonZeroU64::new(self.0).unwrap())
    }
}

impl From<ChannelId> for DiscordId {
    fn from(id: ChannelId) -> Self {
        DiscordId(id.0.get())
    }
}

// GuildId
impl Into<GuildId> for DiscordId {
    fn into(self) -> GuildId {
        GuildId(NonZeroU64::new(self.0).unwrap())
    }
}

impl From<GuildId> for DiscordId {
    fn from(id: GuildId) -> Self {
        DiscordId(id.0.get())
    }
}

// UserId
impl Into<UserId> for DiscordId {
    fn into(self) -> UserId {
        UserId(NonZeroU64::new(self.0).unwrap())
    }
}

impl From<UserId> for DiscordId {
    fn from(id: UserId) -> Self {
        DiscordId(id.0.get())
    }
}

// RoleId
impl Into<RoleId> for DiscordId {
    fn into(self) -> RoleId {
        RoleId(NonZeroU64::new(self.0).unwrap())
    }
}

impl From<RoleId> for DiscordId {
    fn from(id: RoleId) -> Self {
        DiscordId(id.0.get())
    }
}

// i64
impl Into<i64> for DiscordId {
    fn into(self) -> i64 {
        self.0 as i64
    }
}

impl From<i64> for DiscordId {
    fn from(id: i64) -> Self {
        DiscordId(id as u64)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DatabaseId(pub i32);

impl Deref for DatabaseId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl Task {
//     pub async fn message_user(&self, ctx: Context) {

//     }

//     pub async fn create_dropdown(&self, ctx: Context) {
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

//     pub async fn create_team_channel(&self, ctx: Context) {
//         let team_channel = if let TaskType::CreateTeamChannel(team_channel) = &self.task {
//             team_channel
//         } else {
//             log::error!("Not a team channel task");
//             return;
//         };

// }
