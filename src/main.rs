#![feature(async_closure)]

use clap::Parser;

use db_wrapper::DBWrapper;
use handler::Handler;

use console::{app::App, io::IoEvent, start_ui};
use eyre::Result;
use sea_orm::{prelude::*, Database};
use serenity::{all::ApplicationId, prelude::*};
use std::{
    env,
    num::NonZeroU64,
    sync::{atomic::AtomicBool, Arc},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

pub mod commands;
pub mod console;
pub mod db_wrapper;
pub mod game_mechanics;
pub mod handler;
pub mod task_runner;

pub const TEST_GUILD_ID: u64 = 345993194322001923;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Test flag
    #[clap(short, long)]
    test: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
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
        "tokio_tungstenite",
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

    let db: DatabaseConnection = match Database::connect("sqlite://./db.sqlite3").await {
        Ok(db) => db,
        Err(err) => panic!("Error connecting to database: {:?}", err),
    };

    let db_wrapper = DBWrapper::new(db.clone());

    // Start the Serenity client in a new Tokio thread
    tokio::spawn(async move {
        let mut client = Client::builder(&token, gateway_intents)
            .application_id(ApplicationId(NonZeroU64::new(451862707746897961).unwrap()))
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
    });

    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    // We need to share the App between thread
    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // Start the TUI on this thread
    start_ui(&app_ui).await?;

    Ok(())
}
