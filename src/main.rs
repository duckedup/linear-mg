use clap::Parser;
use linear_mg::cli::{Cli, Commands};
use linear_mg::client::LinearClient;
use linear_mg::config::{self, Config};
use linear_mg::error::CliError;

fn main() {
    let cli = Cli::parse();

    if cli.global.verbose {
        tracing_subscriber::fmt()
            .with_env_filter("linear_mg=debug")
            .init();
    }

    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    let result = rt.block_on(dispatch(cli));

    if let Err(e) = result {
        eprintln!("{}", e.to_json());
        std::process::exit(e.exit_code());
    }
}

async fn dispatch(cli: Cli) -> Result<(), CliError> {
    let format = &cli.global.format;

    match cli.command {
        Commands::Auth(cmd) => cmd.run(format).await,
        cmd => {
            let config = Config::load()?;
            let api_key =
                config::auth::resolve_api_key(cli.global.api_key.as_deref(), config.api_key())?;
            let client = LinearClient::new(api_key);

            match cmd {
                Commands::Issues(c) => c.run(&client, format).await,
                Commands::Teams(c) => c.run(&client, format).await,
                Commands::Projects(c) => c.run(&client, format).await,
                Commands::Users(c) => c.run(&client, format).await,
                Commands::Comments(c) => c.run(&client, format).await,
                Commands::Labels(c) => c.run(&client, format).await,
                Commands::Cycles(c) => c.run(&client, format).await,
                Commands::WorkflowStates(c) => c.run(&client, format).await,
                Commands::Documents(c) => c.run(&client, format).await,
                Commands::Initiatives(c) => c.run(&client, format).await,
                Commands::Milestones(c) => c.run(&client, format).await,
                Commands::Attachments(c) => c.run(&client, format).await,
                Commands::Auth(_) => unreachable!(),
            }
        }
    }
}
