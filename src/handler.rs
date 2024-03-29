use crate::{
    commands::{fake_trade::FakeTrade, initialize_game::InitializeGame, nuke::Nuke},
    db_wrapper::DBWrapper,
    game_mechanics::MechanicHandlerWrapper,
    task_runner::{
        tasks::{message::message_component::MessageData, run_tests},
        TaskRunner,
    },
};

use crate::commands::GameCommand;

use entity::entities::message_component_data;
use sea_orm::EntityTrait;
use serenity::{
    all::{ComponentInteractionDataKind, Interaction},
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    model::{gateway::Ready, id::GuildId},
    prelude::*,
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use tracing::{info, log};

pub struct Handler {
    pub is_loop_running: AtomicBool,
    pub run_tests: bool,
    pub db: DBWrapper,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let command_handler = match command.data.name.as_str() {
                    "trade" => FakeTrade::run,
                    "initialize" => InitializeGame::run,
                    "reset" => Nuke::run,
                    _ => unreachable!(),
                };

                if let Err(why) = command
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content("Handling command..."),
                        ),
                    )
                    .await
                {
                    info!("Cannot respond to slash command: {}", why);
                }

                let _content = command_handler(
                    &command.data.options(),
                    command.guild_id.unwrap(),
                    self.db.clone(),
                    ctx.clone(),
                )
                .await;
            }
            Interaction::Component(component) => {
                // Get the payload of the custom_id
                let payload = message_component_data::Entity::find_by_id(
                    uuid::Uuid::parse_str(&component.data.custom_id).unwrap(),
                )
                .one(&*self.db)
                .await
                .unwrap()
                .unwrap()
                .payload;

                // Deserialize the payload
                let task = *serde_json::from_value::<Box<Option<MessageData>>>(payload).unwrap();

                // The task might be none, in which case return
                if task.is_none() {
                    if let Err(why) = component
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content("That interaction is empty!"),
                            ),
                        )
                        .await
                    {
                        info!("Cannot respond to slash command: {}", why);
                    }
                    return;
                }

                let task = task.unwrap();

                if let ComponentInteractionDataKind::Button = &component.data.kind {
                    info!("Button pressed: {:?}", task);

                    // If the task is a TaskType, add it to the database, if
                    // it's a function, run it
                    match task {
                        MessageData::Task(task_type) => {
                            let _ = self.db.add_await_task(task_type).await;
                        }
                        MessageData::Function(mechanic_function) => {
                            mechanic_function
                                .handle(MechanicHandlerWrapper {
                                    db: self.db.clone(),
                                    interaction: Some(component),
                                    ctx: ctx.clone(),
                                })
                                .await;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        for guild in ctx.cache.guilds().iter() {
            GuildId(guild.0)
                .set_application_commands(
                    &ctx.http,
                    vec![
                        FakeTrade::register(),
                        InitializeGame::register(),
                        Nuke::register(),
                    ],
                )
                .await
                .unwrap();
        }
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache built successfully!");

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let db_clone = self.db.clone();
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                let runner = TaskRunner {
                    ctx: ctx_clone,
                    db: db_clone,
                };

                // // Seed an example test
                // runner.sample_tasks().await;

                loop {
                    runner.run_tasks().await;
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            });

            // If the testing flag is active, start a thread and run the tests
            let db_clone = self.db.clone();
            if self.run_tests {
                tokio::spawn(async move {
                    run_tests(ctx.clone(), db_clone).await;
                    // Log test complete
                    log::info!("Tests complete");
                });
            }

            // Now that the loop is running, we set the bool to true
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}
