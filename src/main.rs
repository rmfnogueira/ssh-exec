use std::error::Error;
use std::io;
use std::net::TcpStream;
use std::vec::IntoIter;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;
use ssh2::Session;
use clap::Parser;
use rpassword::read_password;

/// Arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Username with permission to connect via ssh
    #[arg(short, long)]
    username: String,

    /// Command to be run in ssh session
    #[arg(short, long, default_value = "ls")]
    command: String,

    /// Ipv4Adress and TCP Port
    #[arg(short, long, default_value = "127.0.0.1:22")]
    hosts: Vec<String>
    // TODO => Accept multiple sockets to run commands on, assuming same username and password on multiple systems
}

/// ssh-exec

async fn setup_streams(hosts: Vec<String>) -> Vec<AsyncTcpStream> {
    let mut streams = Vec::new();

    let mut iter: IntoIter<String> = hosts.into_iter();

    while let Some(host) = iter.next() {
        match AsyncTcpStream::connect(&host).await {
            Ok(stream) => {
                streams.push(stream);
            }
            Err(e) => {
                eprintln!("Failed to connect to f{}: {}", host, e);
            }
        }
    }
    streams
}

async fn exec_cmds(streams: Vec<AsyncTcpStream>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut session = Session::new()?;
    let streams = streams.iter().map(|| {
        sessio
    })

    session.set_tcp_stream(tcp_stream.into_std()?);
    session.handshake().unwrap();
    session.userauth_password(username, password).unwrap();
    let mut channel = session.channel_session().unwrap();
    channel.exec(command).unwrap();
    let mut output = String::new();
    channel.read_to_string(&mut output).await.unwrap();
    channel.close().unwrap();
    session.disconnect(None, None).unwrap();
     
    Ok((output))
}

async fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    
    // handle and make mandatory, as argument
    println!("Please enter {}'s password", &args.username);
    let pass =  read_password()?;
    
    let streams = setup_streams(args.hosts).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}

#[test]
fn runs_ok() {
    
    
}