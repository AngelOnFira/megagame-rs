use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{
    builder::{CreateActionRow, CreateButton, CreateSelectMenu, CreateSelectMenuOption},
    client::Context,
    http::CacheHttp,
    model::{
        id::{ChannelId, UserId},
        interactions::message_component::{ActionRow, SelectMenuOption},
    },
};
use tracing::log;

use self::message_user::MessageUser;

pub mod message_user;

/// Store the different tasks the bot can do in the database. Each variant has
/// its own struct that can store the rest of the data required for the task.
/// Each of these structs might have their own `impl`s to operate on the data.
#[derive(Serialize, Deserialize, Debug)]
pub enum TaskType {
    ChangeTeam{
        team_id: u64,
        user_id: UserId,
    },
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

#[async_trait]
pub trait Task {
    async fn handle(&self, ctx: Arc<Context>);
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Task {
//     pub task: TaskType,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRole {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDropdown {
    guild_id: u64,
    channel_id: u64,
    custom_id: String,
    options: Vec<SelectMenuOption>,
    action_row: ActionRow,
}

impl CreateDropdown {
    // fn menu_option(&self) -> CreateSelectMenuOption {
    //     let mut opt = CreateSelectMenuOption::default();
    //     // This is what will be shown to the user
    //     opt.label(format!("{} {}", self.emoji(), self));
    //     // This is used to identify the selected value
    //     opt.value(self.to_string().to_ascii_lowercase());
    //     opt
    // }

    // fn select_menu(&self) -> CreateSelectMenu {
    //     let mut menu = CreateSelectMenu::default();
    //     menu.custom_id("animal_select");
    //     menu.placeholder("No animal selected");
    //     menu.options(|f| {
    //         for option in self.options.iter() {
    //             f.add_option(CreateSelectMenuOption::from(option));
    //         }
    //         f.add_option(Self::Cat.menu_option());
    //         f.add_option(Self::Dog.menu_option());
    //         f.add_option(Self::Horse.menu_option());
    //         f.add_option(Self::Alpaca.menu_option())
    //     });
    //     menu
    // }

    // fn action_row(&self) -> CreateActionRow {
    //     let mut ar = CreateActionRow::default();
    //     // A select menu must be the only thing in an action row!
    //     ar.add_select_menu(Self::select_menu());
    //     ar
    // }
}

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
