use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::{client::Context, http::CacheHttp, model::{id::{UserId, ChannelId}, interactions::message_component::SelectMenuOption}, builder::{CreateActionRow, CreateSelectMenu, CreateSelectMenuOption}};

/// Store the different tasks the bot can do in the database. Each variant has
/// its own struct that can store the rest of the data required for the task.
/// Each of these structs might have their own `impl`s to operate on the data.
#[derive(Serialize, Deserialize)]
pub enum TaskType {
    ChangeTeam,
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

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub task: TaskType,
}

/// Send a message to a user with the provided player_id.
#[derive(Serialize, Deserialize)]
pub struct MessageUser {
    pub player_id: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRole {

}

#[derive(Serialize, Deserialize)]
pub struct CreateDropdown {
    guild_id: u64,
    channel_id: u64,
    custom_id: String,
    options: Vec<SelectMenuOption>,
}

impl CreateDropdown {
    fn menu_option(&self) -> CreateSelectMenuOption {
        let mut opt = CreateSelectMenuOption::default();
        // This is what will be shown to the user
        opt.label(format!("{} {}", self.emoji(), self));
        // This is used to identify the selected value
        opt.value(self.to_string().to_ascii_lowercase());
        opt
    }

    fn select_menu(&self) -> CreateSelectMenu {
        let mut menu = CreateSelectMenu::default();
        menu.custom_id("animal_select");
        menu.placeholder("No animal selected");
        menu.options(|f| {
            for option in self.options.iter() {
                f.add_option(CreateSelectMenuOption::from(option));
            }
            f.add_option(Self::Cat.menu_option());
            f.add_option(Self::Dog.menu_option());
            f.add_option(Self::Horse.menu_option());
            f.add_option(Self::Alpaca.menu_option())
        });
        menu
    }

    fn action_row(&self) -> CreateActionRow {
        let mut ar = CreateActionRow::default();
        // A select menu must be the only thing in an action row!
        ar.add_select_menu(Self::select_menu());
        ar
    }
}

impl Task {
    pub async fn message_user(&self, ctx: Arc<Context>) {
        let message = if let TaskType::MessageUser(message) = &self.task {
            message
        } else {
            panic!("Not a message task");
        };

        if let Ok(user) = UserId(message.player_id).to_user(ctx.http()).await {
            match user
                .direct_message(ctx.http(), |m| m.content(message.message.as_str()))
                .await
            {
                Ok(_) => println!("Message sent"),
                Err(why) => println!("Error sending message: {:?}", why),
            };
        };
    }

    pub async fn create_dropdown(&self, ctx: Arc<Context>) {
        let dropdown = if let TaskType::CreateDropdown(dropdown) = &self.task {
            dropdown
        } else {
            panic!("Not a dropdown task");
        };

        let message = ChannelId(dropdown.channel_id).send_message(ctx.http(), |m| {
            m.content("Hello, world!");
            m.components(|c| {
                let mut select_menu = CreateSelectMenu::default();
                select_menu.options(|f| {
                    f.add_option("test".to_string());
                    f.add_option("test2".to_string());
                    f.add_option("test3".to_string())
                });

                let mut action_row = CreateActionRow::default();
                action_row.add_select_menu(select_menu);

                c.add_action_row(action_row)
            })
        })
    }
}
