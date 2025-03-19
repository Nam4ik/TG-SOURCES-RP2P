use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::env; 
use serde_json; 

const    help:    &str = "h";                                                                                      
const verbose:    &str = "v";
static v_mode:    bool = false;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    msg_type: String,
    data: String,
}

async fn handle_peer(mut socket: TcpStream) {
    let args: Vec<String> = env::args.collect(); 
    
    let mut buffer = [0; 1024];
    // Чтение данных от пира
    let n = socket.read(&mut buffer).await.unwrap();
    let msg: Message = serde_json::from_slice(&buffer[..n]).unwrap();
    println!("Получено: {:?}", msg);

    // Отправка ответа
    let response = Message {
        msg_type: "response".to_string(),
        data: "Message received".to_string(),
    };
    let response_data = serde_json::to_vec(&response).unwrap();
    socket.write_all(&response_data).await.unwrap();
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Сервер запущен на порту 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_peer(socket).await;
        });
    }
}

async fn process_response(msg: &str){
 struct ProceesedMessage {
     msg_type: String,
     data:     String,
     args:     String
 }
    println!("Получено сообщение, обработка...");
    let str4process: Result<ProceesedMessage, serde_json::Error> = serde_json::from_str(msg);
 match ProceesedMessage {
      Ok(p)  => println!("Parsed msg: {:?}", p),
      Err(e) => println!("Error while parsing {}", e)
 }

 if ProceesedMessage.msg_type != "command" || ProceesedMessage.msg_type != "message" {
  println!("Unknown mesage type error. Message type must be 'message' or 'command'");
 }
 
 if ProceesedMessage.msg_type == "command" {
     let output = Command::new(ProceesedMessage.data)
         .arg(ProceesedMessage.arg)
         .output()
         .except("Не удалось выполнить команду!");
     
     if output.status.sucess() {
         let mut out = String::from_utf8_lossy(&output.stdout);
    }else {
         let mut out = String::from_utf8_lossy(&output.stderr);
  
     } 
     if stdout.is_empty(){
         let response_out = Message {
             msg_type: "output".to_string(),
             data:  out.to_string()   
         };
     let output_remote = serde_json::to_vec(&response_out).unwrap();
     socket.write_all(&response_out).await.unwrap();
     println!("Command output sended to remote!");
    }
  }
}

/*
fn print_help(){
    println!("Помощь");
    println!("")
}
*/ 
