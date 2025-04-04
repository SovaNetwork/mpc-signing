
use tokio;
pub mod messages;
mod gateway;


use gateway::Gateway;

#[tokio::main]
async fn main() {
    let gateway = Gateway;

    // Simulate a signing request
    let message = b"hello world";
    let signature = gateway.handle_signing_request(message).await.unwrap();

    println!("Final signature: {:?}", signature);

}