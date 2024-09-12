use std::path::PathBuf;
use std::time::Duration;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use clap::{Parser, Subcommand};
use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_fetch::PgFetchSettings;
use pg_embed::postgres::{PgEmbed, PgSettings};
use tempdir::TempDir;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a normal database instance
    Db,
    /// Load a database from a SQL file
    Load {
        /// Path to the SQL file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Make the directory if it does not exist
    let dir = PathBuf::from("local-postgres/postgres-data");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).unwrap();
    }

    // Postgresql settings
    let mut pg_settings = PgSettings {
        // Where to store the postgresql database
        database_dir: dir,
        port: 5432,
        user: "postgres".to_string(),
        password: "postgres".to_string(),
        // authentication method
        auth_method: PgAuthMethod::Plain,
        // If persistent is false clean up files and directories on drop, otherwise keep them
        persistent: false,
        // duration to wait before terminating process execution
        // pg_ctl start/stop and initdb timeout
        // if set to None the process will not be terminated
        timeout: Some(Duration::from_secs(150)),
        // If migration sql scripts need to be run, the directory containing those scripts can be
        // specified here with `Some(PathBuf(path_to_dir)), otherwise `None` to run no migrations.
        // To enable migrations view the **Usage** section for details
        migration_dir: None,
    };

    // Generate a random folder name and create it with proper permissions
    let random_folder_name = format!("db-{}", uuid::Uuid::new_v4());
    let temp_dir = std::env::temp_dir().join(random_folder_name);
    fs::create_dir(&temp_dir).map_err(|e| {
        eprintln!("Failed to create directory: {:?}", e);
        e
    }).unwrap();

    // Set permissions to 700 (owner read/write/execute)
    fs::set_permissions(&temp_dir, fs::Permissions::from_mode(0o777)).map_err(|e| {
        eprintln!("Failed to set permissions: {:?}", e);
        e
    }).unwrap();

    pg_settings.database_dir = temp_dir;
    println!("Database dir: {:?}", pg_settings.database_dir);

    // Verify that the directory exists
    if !pg_settings.database_dir.exists() {
        eprintln!("Database directory does not exist: {:?}", pg_settings.database_dir);
        return;
    }

    match &cli.command {
        Commands::Db => {}
        Commands::Load { file } => {
            pg_settings.migration_dir = Some(file.clone());
        }
    }

    // Postgresql binaries download settings
    let fetch_settings = PgFetchSettings {
        version: pg_embed::pg_fetch::PG_V15,
        ..Default::default()
    };

    // Create a new instance
    let mut pg = match PgEmbed::new(pg_settings, fetch_settings).await {
        Ok(pg) => pg,
        Err(e) => {
            eprintln!("Failed to create PgEmbed instance: {:?}", e);
            return;
        }
    };

    // Download, unpack, create password file and database cluster
    match pg.setup().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to setup PgEmbed: {:?}", e);
            return;
        }
    };

    // stop any running postgresql database
    let _ = pg.stop_db().await;

    // start postgresql database
    match pg.start_db().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to start database: {:?}", e);
            return;
        }
    };

    // pg.create_database("postgres").await.unwrap();
    // If they loaded a file, run the migration
    if let Commands::Load { .. } = &cli.command {
    }

    let pg_uri: &str = &pg.db_uri;

    println!("Postgres is running at: {pg_uri}");

    match &cli.command {
        Commands::Db => println!("Normal database instance started. You can now connect to it."),
        Commands::Load { .. } => {
            println!("Database loaded from SQL file. You can now connect to it.")
        }
    }

    // Endlessly wait until the user stops the program
    loop {
        tokio::time::sleep(Duration::from_secs(100)).await;
    }
}
