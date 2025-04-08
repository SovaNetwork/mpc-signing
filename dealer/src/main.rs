#[warn(dead_code)]
// dealer/src/main.rs
use anyhow::Result;
use dealer::Dealer;
use rand::thread_rng;

mod dealer;

#[tokio::main]
async fn main() -> Result<()> {
    let dealer = Dealer;

    // Simulate receiving a message from the gateway via p2p (for testing purposes)
    let message = b"hello from gateway";

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

    Ok(())
}
