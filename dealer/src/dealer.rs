
pub struct Dealer;
//use frost_ed25519::keys::KeyPackage;
use rand::thread_rng;


impl Dealer{
    pub async fn handle_message(&self, message: &[u8]) -> Vec<u8> {
        println!("Dealer received message: {:?}", message);
        let parsed_message = self.parse_message(message).await;
        let response = self.process_message(parsed_message).await;
        response
    }

    async fn parse_message(&self, message: &[u8]) -> String {
        println!("Parsing message: {:?}", message);

        // TODO: Deserialize 
        String::from_utf8_lossy(message).to_string()
    }

    async fn process_message(&self, message: String) -> Vec<u8> {
        println!("Processing message: {:?}", message);

        // TODO: Process the message and generate a response
        message.into_bytes()
    }

}