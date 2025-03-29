use async_std::net::UdpSocket; // as AsyncUdpSocket;
use async_std::sync::{Arc as AsyncArc, Mutex as AsyncMutex};
use async_std::task;
use std::process;

//use std::net::UdpSocket;
//use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use std::env;
use std::str;
use std::thread;
use std::time::Duration;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <port>", args[0]);
        return Ok(());
    }

    let pid = process::id();
    let pid_string = pid.to_string();

    let port_str = &args[1];

    match port_str.parse::<u16>() {
        Ok(port) => {
            println!("Port: {}", port);

            let port: u16 = env::var("PORT")
                .map(|s| s.parse().unwrap_or(port))
                .unwrap_or(port);

            println!("Port: {}", port);
            let shared_listen = String::from(format!("127.0.0.1:{}", port + 10));
            let address = String::from(format!("127.0.0.1:{}", port + 1));
            let next_address = String::from(format!("127.0.0.1:{}", port + 2));
            let next_next_address = String::from(format!("127.0.0.1:{}", port + 3));
            let next_next_next_address = String::from(format!("127.0.0.1:{}", port + 4));
            let next_next_next_next_address = String::from(format!("127.0.0.1:{}", port + 5));

            //s bind
            let socket = UdpSocket::bind(address.clone()).await?;
            println!("Listening on {}", socket.local_addr()?);
            let connected = socket.connect(next_address.clone()).await?;
            println!("connected {:?}", connected);
            let connected = socket.connect(next_next_address.clone()).await?;
            println!("connected {:?}", connected);
            let connected = socket.connect(next_next_next_address.clone()).await?;
            println!("connected {:?}", connected);

            let shared_listen = socket.connect(shared_listen.clone()).await?;
            println!("connected {:?}", shared_listen);

            //r bind
            let receiver = UdpSocket::bind(next_next_address.clone()).await?;
            println!("Listening on {}", receiver.local_addr()?);
            let r_connected = receiver.connect(address.clone()).await?;
            println!("r_connected {:?}", r_connected);
            let r_connected = receiver.connect(next_address.clone()).await?;
            println!("r_connected {:?}", r_connected);
            let r_connected = receiver.connect(next_next_address.clone()).await?;
            println!("r_connected {:?}", r_connected);

            //let shared_listen = receiver.connect(shared_listen.clone()).await?;
            println!("connected {:?}", shared_listen);

            //a bind
            let a_receiver = UdpSocket::bind(next_next_next_address.clone()).await?;
            println!("Listening on {}", a_receiver.local_addr()?);
            let a_connected = a_receiver.connect(address).await?;
            println!("a_connected {:?}", a_connected);
            let a_connected = a_receiver.connect(next_address).await?;
            println!("a_connected {:?}", a_connected);
            let a_connected = a_receiver.connect(next_next_address).await?;
            println!("a_connected {:?}", a_connected);

            //let shared_listen = a_receiver.connect(shared_listen.clone()).await?;
            println!("connected {:?}", shared_listen);

            let mut buf = vec![0u8; 1024];
            task::spawn(async move {
                loop {
                    match a_receiver.recv_from(&mut buf).await {
                        Ok((size, addr)) => {
                            let received_data = &buf[..size];
                            match str::from_utf8(received_data) {
                                Ok(received_string) => {
                                    println!("loop Received from {}: {}", addr, received_string);
                                }
                                Err(e) => {
                                    eprintln!("Error decoding UTF-8: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error receiving data: {}", e);
                        }
                    }
                }
            });

            task::spawn(async move {
                loop {
                    for i in 0..10 {
                        match SystemTime::now().duration_since(UNIX_EPOCH) {
                            Ok(duration) => {
                                let seconds = duration.as_secs();
                                let seconds_string = seconds.to_string();
                                println!(
                                    "Current time in seconds since the Unix epoch: {}",
                                    seconds_string
                                );

                                let message = format!(
                                    "S:Hello from {}! {}:{:?}",
                                    pid,
                                    i,
                                    SystemTime::now().duration_since(UNIX_EPOCH)
                                );
                                match socket.send(message.as_bytes()).await {
                                    Ok(sent) => {
                                        println!("{}:Sent: {:?}\nmessage:{}", pid, sent, message);
                                        thread::sleep(Duration::from_millis(2000)); // Simulate some delay
                                    }
                                    Err(sent) => {
                                        println!("{}:Sent: {:?}\n{}\nmessage:", pid, sent, message);
                                        thread::sleep(Duration::from_millis(2000)); // Simulate some delay
                                    }
                                }
                                let message = format!(
                                    "R:Hello from {}! {}:{:?}",
                                    pid,
                                    i,
                                    SystemTime::now().duration_since(UNIX_EPOCH)
                                );
                                match receiver.send(message.as_bytes()).await {
                                    Ok(sent) => {
                                        println!("{}:Sent: {:?}\nmessage:{}", pid, sent, message);
                                        thread::sleep(Duration::from_millis(2000)); // Simulate some delay
                                    }
                                    Err(sent) => {
                                        println!("{}:Sent: {:?}\nmessage:{}", pid, sent, message);
                                        thread::sleep(Duration::from_millis(2000)); // Simulate some delay
                                    }
                                }
                            }
                            Err(error) => {
                                eprintln!("Error getting time: {:?}", error);
                            }
                        }
                    }
                }
            });

            // Keep the main task running indefinitely.
            task::sleep(std::time::Duration::from_secs(u64::MAX)).await;
        }
        Err(e) => {
            eprintln!("Error parsing port: {}", e);
        }
    }
    Ok(())
}
