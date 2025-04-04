

use anyhow::Result;

pub struct Gateway;


impl Gateway {
    pub async fn handle_signing_request(&self, message: &[u8]) -> Result<Vec<u8>> {
        println!("Gateway received signing request: {:?}", message);
        let signature = self.call_dealer(message).await?;
        Ok(signature)
    }

    async fn call_dealer(&self, message: &[u8]) -> Result<Vec<u8>> {
        println!("Calling dealer with message...");
        Ok(message.to_vec()) // Simulate dealer response
    }
}
     


