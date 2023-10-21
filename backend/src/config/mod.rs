use clap::{ArgAction, Args, Parser};
use dotenvy::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Debug, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Config {
    fn new() -> Self {
        dotenv().ok();
        let args = Cli::parse();

        Self {
            app: args.app,
            server: args.server,
            database: args.database,
        }
    }
}

#[derive(Debug, Clone, Args)]
#[clap(next_help_heading = "App options")]
pub struct AppConfig {
    /// Log level for the application
    #[clap(
        long,
        default_value = "warn,anime_watcher_backend=info",
        env = "RUST_LOG"
    )]
    pub log_level: String,
}

#[derive(Debug, Clone, Args)]
#[clap(next_help_heading = "Server options")]
pub struct ServerConfig {
    /// Host to listen on
    #[clap(short, long, default_value = "127.0.0.1", env = "HOST")]
    pub host: String,
    /// Port to listen on
    #[clap(short, long, default_value = "3001", env = "PORT")]
    pub port: u16,
}

#[derive(Debug, Clone, Args)]
#[clap(next_help_heading = "Database options")]
pub struct DatabaseConfig {
    /// Database URL.
    ///
    /// Should be in the format of eg. `sqlite:///absolute/path/to/database.sqlite` or just `sqlite://./path/to/database.sqlite`.
    ///
    /// If not specified, an in-memory database will be used.
    #[clap(long = "database-url", env = "DATABASE_URL")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Parser)]
#[clap(disable_help_flag = true)]
struct Cli {
    /// Print help
    #[clap(action = ArgAction::Help, long)]
    help: Option<bool>,

    #[command(flatten)]
    app: AppConfig,

    #[command(flatten)]
    server: ServerConfig,

    #[command(flatten)]
    database: DatabaseConfig,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;

    Cli::command().debug_assert();
}
