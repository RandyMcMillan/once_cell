use async_std::net::UdpSocket as AsyncUdpSocket;
use async_std::sync::{Arc as AsyncArc, Mutex as AsyncMutex};

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

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

    let port_str = &args[1];

    match port_str.parse::<u16>() {
        Ok(port) => {
            println!("Port: {}", port);

            let port: u16 = env::var("PORT")
                .map(|s| s.parse().unwrap_or(port))
                .unwrap_or(port);

            println!("Port: {}", port);
            let address = String::from(format!("127.0.0.1:{}", port));
            let next_address = String::from(format!("127.0.0.1:{}", port + 1));
            let next_next_address = String::from(format!("127.0.0.1:{}", port + 2));

            let receiver_ready = Arc::new(Mutex::new(false));
            let receiver_ready_clone = receiver_ready.clone();

            let socket = UdpSocket::bind(address)?;
            println!("Listening on {}", socket.local_addr()?);

            let connected = socket.connect(next_address);
            println!("connected {:?}", connected);

            // Receiver setup
            let receiver = UdpSocket::bind(next_next_address)?;

            // Spawn a thread for receiving
            let receiver_thread = thread::spawn(async move || {
                let mut buf = [0; 1024];
                loop {
                    //break;
                    match receiver.recv_from(&mut buf) {
                        Ok((size, addr)) => {
                            let received_data = &buf[..size];
                            let received_string = str::from_utf8(received_data).unwrap();
                            println!("Received from {}: {}", addr, received_string);
                        }
                        Err(e) => {
                            eprintln!("Error receiving data: {}", e);
                            break;
                        }
                    }
                }
            });

            // Wait for receiver to become ready
            //while !*receiver_ready.lock().unwrap() {
				println!("waiting...");
            //    thread::sleep(Duration::from_millis(10));
            //}




//            let _ = receiver_thread.join().unwrap();



            let mut buf = vec![0u8; 1024];

            for i in 0..10 {
                let message = format!("Hello from sender! {}", i);
                let sent = socket.send(message.as_bytes());
                println!("Sent: {:?}\n{}", sent, message);
                let sent = socket.send(&buf[..1024])?;
                println!("Sent: {:?}\n{}", sent, message);
                thread::sleep(Duration::from_millis(2000)); // Simulate some delay
            }

            //let mut buf = vec![0u8; 1024];

            //loop {
            //    let (recv, peer) = socket.recv_from(&mut buf)?;
            //    //let sent = socket.send(message.as_bytes());
            //    let sent = socket.send(&buf[..recv])?;
            //    println!("Sent {} out of {} bytes to {}", sent, recv, peer);
            //}
        }
        Err(e) => {
            eprintln!("Error parsing port: {}", e);
        }
    }
    Ok(())
}
