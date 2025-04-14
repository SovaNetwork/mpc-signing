use proto_api::mpc_gateway::{gateway_client::GatewayClient, SignRequest};
use tonic::transport::{Channel, Endpoint};
use tower::timeout::Timeout;

#[derive(Debug, Clone)]
pub struct GatewayGrpcClient {
    pub max_signers: i32,
    pub min_signers: i32,
}

impl GatewayGrpcClient {
    pub fn new(max_signers: i32, min_signers: i32) -> anyhow::Result<Self> {
        Ok(Self {
            max_signers,
            min_signers,
        })
    }

    async fn connect_client(&self) -> anyhow::Result<GatewayClient<Timeout<Channel>>> {
        let endpoint = "http://127.0.0.1:50052";
        let channel = Endpoint::from_shared(endpoint)
            .map_err(|e| anyhow::anyhow!("Failed to create endpoint: {}", e))?
            .connect()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to gRPC server: {}", e))?;
        let timeout_channel = Timeout::new(channel, std::time::Duration::from_millis(5000));
        let client = GatewayClient::new(timeout_channel);
        Ok(client)
    }

    pub async fn sign(&self, label: String, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let req = SignRequest {
            max_signers: self.max_signers,
            min_signers: self.min_signers,
            label,
            message: data,
        };
        let response = self.connect_client().await?.sign(req).await?;
        let res = response.into_inner();
        Ok(res.signature)
    }
}
