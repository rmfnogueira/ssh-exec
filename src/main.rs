use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Stdout,Result};
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
fn run() -> Result<()> {
    let args = Cli::parse();
    
    // handle and make mandatory, as argument
    println!("Please enter {}'s password", &args.username);
    let pass =  read_password()?;
    
    let hosts = args.hosts;

    for host in hosts.iter() {
        let tcp_stream = TcpStream::connect(args.socket.to_string())?;
        // todo : add parameters or args via cli
    
        let mut tcp_session = Session::new()?;
        tcp_session.set_tcp_stream(tcp_stream);
        tcp_session.handshake()?;
        tcp_session.userauth_password(&args.username.to_string(), &pass)?; // get password on command line, all other args in config file
        
        let mut channel = tcp_session.channel_session()?;
        channel.exec(&args.command)?;
        
        let mut s = String::new();
        channel.read_to_string(&mut s)?;
        
        println!("{}", s);
        channel.wait_close()?;
        println!("session exit status: {}", channel.exit_status()?);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}


#[test]
fn runs_ok() {
    
    
}