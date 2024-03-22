use clap::{Parser, Subcommand};
use reqwest::Error;
use tokio;

#[derive(Parser, Debug)]
#[command(name = "cli-lxd", version = "0.0", author = "Ugly organization", about = "Interacts with letterboxd API")]
struct Cli {
    /// Token for authentication
    #[arg(short, long)]
    token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Operations on favorite list
    Favorite {
        #[command(subcommand)]
        action: FavoriteActions,
    },
}

#[derive(Subcommand, Debug)]
enum FavoriteActions {
    /// View favorite lists
    View,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let cli = Cli::parse();
    let token = cli.token;
    let url =  "http://httpbin.org/get";

    let mut request = client.get(url).bearer_auth(token);
    request = match cli.command {
        Commands::Favorite { action } => 
        match action {
            FavoriteActions::View => request.header("action", "view"),
        }
    };

    let res = request.send().await?;
    let body = res.text().await?;
    println!("Response body: {}", body);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_true() {
        assert_eq!(1, 1);
    }
}