use std::net::SocketAddr;
use std::net::TcpListener;
use std::error::Error;

pub async fn start_server(addr: SocketAddr) -> Result<(), Box<dyn Error>> {

    let listener = TcpListener::bind(addr)?;

    

    Ok(())
}