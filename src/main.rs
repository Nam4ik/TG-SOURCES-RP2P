use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    msg_type: String,
    data: String,
}

async fn handle_peer(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
   
    let n = socket.read(&mut buffer).await.unwrap();
    let msg: Message = serde_json::from_slice(&buffer[..n]).unwrap();
    println!("Получено: {:?}", msg);

   
    let response = Message {
        msg_type: "response".to_string(),
        data: "Message received".to_string(),
    };
    let response_data = serde_json::to_vec(&response).unwrap();
    socket.write_all(&response_data).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на порту 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_peer(socket).await;
        });
    }
}

