use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::Parser;

use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::GuildId},
    prelude::*,
};
use tracing::{Level, log};
use tracing_subscriber::EnvFilter;

use crate::{
    schema::tasks_task,
    task_runner::{tasks::{TaskType, message_user::MessageUser}, TaskRunner},
};

mod schema;
mod task_runner;

struct Handler {
    is_loop_running: AtomicBool,
    run_tests: bool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        dbg!(msg.clone());
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);

        let db: DatabaseConnection = match Database::connect("sqlite://./django/db.sqlite3").await {
            Ok(db) => db,
            Err(err) => panic!("Error connecting to database: {:?}", err),
        };

        if !self.is_loop_running.load(Ordering::Relaxed) {
            // If tests are enabled, start them in another thread
            let ctx1 = Arc::clone(&ctx);
            if self.run_tests {
                tokio::spawn(async move { run_tests(ctx1).await });
            }

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                let runner = TaskRunner { ctx: ctx2, db: db };

                // Seed an example test
                // runner.sample_tasks().await;

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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Test flag
    #[clap(short, long)]
    test: bool,
}

#[tokio::main]
async fn main() {
    let append_info = |mut f: EnvFilter, list: &[&str], level: &str| {
        for l in list {
            f = f.add_directive(format!("{}={}", l, level).parse().unwrap());
        }
        f
    };

    let list = &[
        "tokio_util",
        "h2",
        "rustls",
        "serenity",
        "tungstenite",
        "async_tungstenite",
        "hyper",
        "trust_dns_resolver",
        "trust_dns_proto",
        "reqwest",
        "mio",
        "want",
        "kube",
        "tower",
    ];

    let off_list = &["sqlx"];

    let filter = EnvFilter::from_default_env();
    let filter = append_info(filter.add_directive(Level::TRACE.into()), off_list, "off");
    let filter = append_info(filter.add_directive(Level::TRACE.into()), list, "info");

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_env_filter(filter)
        .try_init()
        .unwrap();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let args = Args::parse();

    let mut client = Client::builder(&token)
        .application_id(451862707746897961)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
            run_tests: args.test,
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
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

    tasks_task::ActiveModel {
        payload: Set(serde_json::to_string(&task).unwrap()),
        completed: Set("false".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    log::info!("Task inserted");
}
