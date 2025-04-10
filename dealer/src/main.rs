mod dealer;

use crate::dealer::Dealer;
use anyhow::{Context, Result};
use p2p::RawProtocolMessage;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Dealer");

    let grpc_address = "127.0.0.1".to_string();
    let grpc_port = 50051;

    println!("listening on {}:{}", grpc_address, grpc_port);

    // === Create communication channels ===
    let (_, command_receiver) = mpsc::unbounded_channel();
    let (event_sender, _) = mpsc::unbounded_channel();
    let (message_sender, message_receiver) = mpsc::unbounded_channel();

    let _node = p2p::Node::new(command_receiver, message_sender, event_sender)?;

    //node.run().await?;

    // === Start gRPC API ===
    let api = Dealer::new(
        grpc_address,
        grpc_port,
        None, // No timeout
        None, // No concurrency limit
    );
    tokio::spawn(async move {
        if let Err(e) = api.start().await.context("Unable to start gRPC API") {
            println!("API start failed: {:?}", e);
        }
    });

    handle_messages(message_receiver).await;

    Ok(())
}

pub async fn handle_messages(mut message_receiver: mpsc::UnboundedReceiver<RawProtocolMessage>) {
    while let Some(msg) = message_receiver.recv().await {
        println!("Received Message, ignored: {:?}", msg);
    }
}
