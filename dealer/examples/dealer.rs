use proto_api::mpc_gateway::{gateway_client::GatewayClient, SignRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the Dealer gRPC server
    let mut client = GatewayClient::connect("http://127.0.0.1:50052").await?;

    // Construct a signing request
    let request = tonic::Request::new(SignRequest {
        label: "test-sign-request".to_string(),
        max_signers: 3,
        min_signers: 2,
        message: vec![1, 2, 3, 4],
    });

    // Send the request
    let response = client.sign(request).await?;

    println!("RESPONSE = {:?}", response.into_inner());

    Ok(())
}
