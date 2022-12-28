#![feature(async_closure)]

use clap::Parser;

use db_wrapper::DBWrapper;
use handler::Handler;

use eyre::Result;
use sea_orm::{prelude::*, Database};
use serenity::{all::ApplicationId, prelude::*};
use std::{env, num::NonZeroU64, sync::atomic::AtomicBool};
use tracing::{info, Level};
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
    // Early initialization of the logger

    // // Set max_log_level to Trace
    // tui_logger::init_logger(log::LevelFilter::Info).unwrap();

    // // Set default level for unknown targets to Trace
    // tui_logger::set_default_level(log::LevelFilter::Info);

    // // Set the log level of noisy targets to Off
    // tui_logger::set_level_for_target("tokio_util", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("h2", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("rustls", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("serenity", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("tungstenite", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("async_tungstenite", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("hyper", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("trust_dns_resolver", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("trust_dns_proto", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("reqwest", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("mio", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("want", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("kube", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("tower", log::LevelFilter::Off);
    // tui_logger::set_level_for_target("tokio_tungstenite",
    // log::LevelFilter::Off);

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
    let serenity_handle = tokio::spawn(async move {
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
            info!("Client error: {:?}", why);
        }
    });

    // let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    // // We need to share the App between thread
    // let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    // let app_ui = Arc::clone(&app);

    // // Handle Console IO in a specifc thread
    // tokio::spawn(async move {
    //     let mut handler = IoAsyncHandler::new(app);
    //     while let Some(io_event) = sync_io_rx.recv().await {
    //         handler.handle_io_event(io_event).await;
    //     }
    // });

    // // Start the TUI on this thread
    // start_ui(&app_ui).await?;

    // Wait for the serenity client to finish
    serenity_handle.await?;

    Ok(())
}
