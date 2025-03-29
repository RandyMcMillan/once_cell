use async_std::net::UdpSocket;
use std::env;
use std::env::args;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <port>", args[0]);
        return Ok(());
    }

    let mut port_str = &args[1];

    match port_str.parse::<u16>() {
        Ok(port) => {
            println!("Port: {}", port);

            let port: u16 = env::var("PORT")
                .map(|s| s.parse().unwrap_or(port))
                .unwrap_or(port); // Default port

            println!("Port: {}", port);
            let address = String::from(format!("127.0.0.1:{}", port));

            let socket = UdpSocket::bind(address).await?;
            println!("Listening on {}", socket.local_addr()?);

            let mut buf = vec![0u8; 1024];

            loop {
                let (recv, peer) = socket.recv_from(&mut buf).await?;
                let sent = socket.send_to(&buf[..recv], &peer).await?;
                println!("Sent {} out of {} bytes to {}", sent, recv, peer);
            }
        }
        Err(e) => {
            eprintln!("Error parsing port: {}", e);
        }
    }
    Ok(())
}
