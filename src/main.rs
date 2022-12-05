use clap::Parser;
use entity::entities::task;
use handler::Handler;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, Database};
use serenity::{
    async_trait,
    model::{application::interaction::Interaction, channel::Message, gateway::Ready, id::GuildId},
    prelude::*,
};
use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tracing::{log, Level};
use tracing_subscriber::EnvFilter;

use crate::task_runner::{
    task_queue::memory::MemoryTaskQueue,
    tasks::{message_user::MessageUser, TaskType},
    TaskRunner,
};

pub mod task_runner;
pub mod commands;
pub mod handler;



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

    let gateway_intents = GatewayIntents::all();

    let mut client = Client::builder(&token, gateway_intents)
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


