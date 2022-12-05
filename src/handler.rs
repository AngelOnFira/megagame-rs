use crate::{
    commands::{self, fake_trade::FakeTrade, initialize_game::InitializeGame},
    task_runner::{
        task_queue::memory::MemoryTaskQueue,
        tasks::{message_user::MessageUser, TaskType},
        TaskRunner,
    },
};

use crate::commands::GameCommand;
use entity::entities::task;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};
use serenity::{
    async_trait,
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tracing::log;

pub struct Handler {
    pub is_loop_running: AtomicBool,
    pub run_tests: bool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "trade" => FakeTrade::run(&command.data.options),
                "init" => InitializeGame::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ctx.cache.guilds().iter() {
            dbg!(guild.0);
            GuildId(guild.0)
                .set_application_commands(&ctx.http, |commands| {
                    commands
                        .create_application_command(|command| FakeTrade::register(command))
                        .create_application_command(|command| InitializeGame::register(command))
                })
                .await
                .unwrap();
        }
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            // If tests are enabled, start them in another thread
            let ctx1 = Arc::clone(&ctx);
            if self.run_tests {
                tokio::spawn(async move { run_tests(ctx1).await });
            }

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                let mut runner = TaskRunner {
                    ctx: ctx2,
                    db: Box::new(MemoryTaskQueue::new()),
                };

                // Seed an example test
                runner.sample_tasks().await;

                loop {
                    runner.run_tasks().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });

            // Now that the loop is running, we set the bool to true
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn run_tests(ctx: Arc<Context>) {
    let db: DatabaseConnection = match Database::connect("sqlite://./django/db.sqlite3").await {
        Ok(db) => db,
        Err(err) => panic!("Error connecting to database: {:?}", err),
    };

    dbg!(ctx.cache.current_user_id().0);

    let task = TaskType::MessageUser(MessageUser {
        player_id: 133358326439346176,
        message: String::from("Good dayyy"),
    });

    task::ActiveModel {
        payload: Set(serde_json::to_string(&task).unwrap()),
        completed: Set(false),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    log::info!("Task inserted");
}
