

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
#[tokio::main]
async fn main() {
    let dealer = Dealer;

    // Simulate receiving a message from the gateway
    let message = b"hello world";
    let response = dealer.handle_message(message).await;

    println!("Dealer response: {:?}", response);


    // Example of generating a key package
    let mut rng = thread_rng();
    let max_signers = 3;
    let min_signers = 2;
    let (shares, pubkey_package) = frost_ed25519::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost_ed25519::keys::IdentifierList::Default,
        &mut rng,
    )
    .expect("Failed to generate key package");
    println!("Key package: {:?}", pubkey_package);
    println!("Shares: {:?}", shares);
    
}