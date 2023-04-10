use std::net::TcpStream;
use std::io::prelude::*;
use ssh2::Session;
use clap::Parser;

fn main() -> Result<(), std::io::Error> {
    let tcp_stream = TcpStream::connect("")?;
// todo : add parameters or args via cli

    let mut tcp_session = Session::new()?;
    tcp_session.set_tcp_stream(tcp_stream);
    tcp_session.handshake()?;
    tcp_session.userauth_password("rui", "")?;
    let mut tcp_channel = tcp_session.channel_session()?;
    tcp_channel.exec("ls")?;
    let mut s = String::new();
    tcp_channel.read_to_string(&mut s)?;
    println!("{}", s);
    tcp_channel.wait_close()?;
    println!("{}", tcp_channel.exit_status()?);
Ok(())
}


#[test]
fn runs() {

    
}