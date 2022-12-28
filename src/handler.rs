use crate::{
    commands::{fake_trade::FakeTrade, initialize_game::InitializeGame},
    db_wrapper::DBWrapper,
    task_runner::{tasks::run_tests, TaskRunner},
};

use crate::commands::GameCommand;

use serenity::{
    all::Interaction,
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    model::{gateway::Ready, id::GuildId},
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
                    println!("Cannot respond to slash command: {}", why);
                }

                let _content = command_handler(
                    &command.data.options(),
                    command.guild_id.unwrap(),
                    self.db.clone(),
                )
                .await;
            }
            Interaction::Component(component) => {
                dbg!(&component.data);
            }
            _ => (),
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ctx.cache.guilds().iter() {
            GuildId(guild.0)
                .set_application_commands(
                    &ctx.http,
                    vec![FakeTrade::register(), InitializeGame::register()],
                )
                .await
                .unwrap();
        }
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx2 = Arc::clone(&ctx);
            let db_clone = self.db.clone();
            tokio::spawn(async move {
                let runner = TaskRunner {
                    ctx: ctx2,
                    db: db_clone,
                };

                // // Seed an example test
                // runner.sample_tasks().await;

                loop {
                    runner.run_tasks().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });

            // If the testing flag is active, start a thread and run the tests
            let db_clone = self.db.clone();
            if self.run_tests {
                tokio::spawn(async move {
                    run_tests(ctx, db_clone).await;
                    // Log test complete
                    log::info!("Tests complete");
                });
            }

            // Now that the loop is running, we set the bool to true
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

// async fn run_tests(ctx: Arc<Context>) {
//     let db: DatabaseConnection = match Database::connect("sqlite://./django/db.sqlite3").await {
//         Ok(db) => db,
//         Err(err) => panic!("Error connecting to database: {:?}", err),
//     };

//     dbg!(ctx.cache.current_user_id().0);

//     let task = TaskType::MessageUser(MessageUser {
//         player_id: 133358326439346176,
//         message: String::from("Good dayyy"),
//     });

//     task::ActiveModel {
//         payload: Set(serde_json::to_string(&task).unwrap()),
//         completed: Set(false),
//         ..Default::default()
//     }
//     .insert(&db)
//     .await
//     .unwrap();

//     log::info!("Task inserted");
// }
