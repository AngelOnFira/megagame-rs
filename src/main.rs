use clap::Parser;

use db_wrapper::DBWrapper;
use handler::Handler;

use sea_orm::{prelude::*, Database};
use serenity::prelude::*;
use std::{env, sync::atomic::AtomicBool};
use tracing::Level;
use tracing_subscriber::EnvFilter;

pub mod commands;
pub mod db_wrapper;
pub mod handler;
pub mod task_runner;

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

    let db: DatabaseConnection = match Database::connect("sqlite://./django/db.sqlite3").await {
        Ok(db) => db,
        Err(err) => panic!("Error connecting to database: {:?}", err),
    };

    let db_wrapper = DBWrapper::new(db.clone());

    let mut client = Client::builder(&token, gateway_intents)
        .application_id(451862707746897961)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
            run_tests: args.test,
            db: db_wrapper,
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
