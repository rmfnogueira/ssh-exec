use async_ssh2_tokio::client::{Client, AuthMethod, ServerCheckMethod};
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    /// should be the same for all hosts
    username: String,
    /// should be the same for all hosts
    password: String,
    /// command to be run on all hosts
    command: String,
    /// hosts socket addresses
    ipv4addr: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), async_ssh2_tokio::Error> {
    let args = Cli::parse();

    for ip in args.ipv4addr {
        let client = Client::connect(
            (ip, 22),
            &args.password,
            AuthMethod::with_password(&args.password),
            ServerCheckMethod::NoCheck,
        ).await?;
        client.execute("ls").await?;
    };
    Ok(())
}