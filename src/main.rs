use async_ssh2_tokio::client::{Client, AuthMethod, ServerCheckMethod, self};
use clap::{Parser};
use rpassword::read_password;


#[derive(Debug, Parser)]
struct Cli {
    /// should be the same for all hosts
    username: String,
    /// should be the same for all hosts
    command: String,
    /// hosts socket addresses
    ipv4addr: Vec<String>,
}

fn prompt_password() -> Option<Result<String, async_ssh2_tokio::Error>> {
    let password = read_password().ok()?;
    if password.is_empty() {
        None
    } else {
        Some(Ok(password))
    }
}

#[tokio::main]
async fn main() -> Result<(), async_ssh2_tokio::Error> {
    let password = match prompt_password() {
        Some(Ok(p)) => p,
        Some(Err(e)) => return Err(e),
        None => {
            // Handle the case where no password was provided
            println!("No password provided");
            return Ok(());
        }
    };

    let args = Cli::parse();
    for ip in &args.ipv4addr {
            let client = Client::connect(
                ip,
                &password,
                AuthMethod::with_password(&password),
                ServerCheckMethod::NoCheck,
            ).await?;
            client.execute("ls").await?;
        };
        Ok(())
    }
