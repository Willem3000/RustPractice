use tokio::net::{ TcpListener, TcpStream } ;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

const HOST_ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // Address to bind the server
    let listener = TcpListener::bind(HOST_ADDRESS).await.unwrap();
    println!("Listening on: {}", HOST_ADDRESS);

    // Accept incoming connections
    while let Ok((stream, _)) = listener.accept().await {
        let handle = tokio::spawn(echo_on(stream));
        handle.await?;
    }
    Ok(())
}

async fn echo_on(stream: TcpStream) -> Result<(), Box<dyn std::error::Error + Send>> {
    let mut ws_stream = accept_async(stream).await.unwrap();
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }
    Ok(())
}