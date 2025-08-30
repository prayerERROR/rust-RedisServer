#![allow(unused_imports)]
// Std lib
use std::{
    sync::Arc,
    collections::HashMap,
};

// Tokio lib
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

// Parser
mod parser; 
use parser::{Command, parse};

// Database
mod database;
use database::DataBase;


// main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db = Arc::new(Mutex::new(DataBase::new()));
    
    while let Ok((stream, _addr)) = listener.accept().await {
        let db_clone = Arc::clone(&db);
        tokio::spawn(async move {
            handle_connection(stream, db_clone).await;
        });
    }

    Ok(())
}


// handle connections
async fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<DataBase>>) {
    let mut buffer = [0u8; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(_) => {
                let command = String::from_utf8_lossy(&buffer).into_owned();
                match parse(command) {
                    Command::ECHO(size, info) => {
                        stream.write_all(format!("{size}\r\n{info}\r\n").as_bytes()).await.unwrap();
                    },
                    Command::GET(key) => {
                        let mut database = db.lock().await;
                        match database.get(&key) {
                            Some(val) => {
                                stream.write_all(format!("${}\r\n{}\r\n", val.len(), val).as_bytes()).await.unwrap();
                            },
                            None => {
                                stream.write_all(b"$-1\r\n").await.unwrap()
                            },
                        }
                    },
                    Command::PING => {
                        stream.write_all(b"+PONG\r\n").await.unwrap();
                    },
                    Command::SET(key, value, valid_time) => {
                        let mut database = db.lock().await;
                        database.insert(key, value, valid_time);
                        stream.write_all(b"+OK\r\n").await.unwrap();
                    },
                    Command::ERROR => {},
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }
}
