use std::{env, error::Error, fs, path::Path};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use serde::{Serialize, Deserialize};

const BUFFER_SIZE: usize = 4096;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    FileRequest(String),
    FileData(Vec<u8>),
    PeerList(Vec<String>),
}

struct PeerNode {
    address: String,
    peers: Vec<String>,
}

impl PeerNode {
    async fn new(address: String, known_peers: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut node = PeerNode {
            address: address.clone(),
            peers: known_peers,
        };
        
        node.start().await?;
        Ok(node)
    }

    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Listening on {}", self.address);

        let (tx, mut rx) = mpsc::channel(32);

        // Server task
        let server_handle = tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let tx = tx.clone();

                tokio::spawn(async move {
                    let mut buf = [0u8; BUFFER_SIZE];
                    let n = socket.read(&mut buf).await.unwrap();
                    
                    let msg: Message = bincode::deserialize(&buf[..n]).unwrap();
                    match msg {
                        Message::FileRequest(filename) => {
                            let file_data = fs::read(&filename).unwrap();
                            let response = Message::FileData(file_data);
                            let bytes = bincode::serialize(&response).unwrap();
                            socket.write_all(&bytes).await.unwrap();
                        }
                        _ => {}
                    }
                });
            }
        });

        // Client task
        let client_handle = tokio::spawn(async move {
            loop {
                // Пример: запрос файла у первого пира
                if let Some(peer) = self.peers.first() {
                    let mut stream = TcpStream::connect(peer).await.unwrap();
                    let request = Message::FileRequest("test.txt".to_string());
                    let bytes = bincode::serialize(&request).unwrap();
                    stream.write_all(&bytes).await.unwrap();

                    let mut buf = [0u8; BUFFER_SIZE];
                    let n = stream.read(&mut buf).await.unwrap();
                    let response: Message = bincode::deserialize(&buf[..n]).unwrap();
                    
                    if let Message::FileData(data) = response {
                        fs::write("received.txt", data).unwrap();
                        println!("File received!");
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        });

        tokio::select! {
            _ = server_handle => {},
            _ = client_handle => {},
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let address = args[1].clone();
    let known_peers = args[2..].to_vec();

    let mut node = PeerNode::new(address, known_peers).await?;
    
    Ok(())
}
