use anyhow::Result;
use proto_api::mpc_gateway::{gateway_client::GatewayClient, SignRequest};
use tonic::transport::{Channel, Endpoint};
use tower::timeout::Timeout;

#[derive(Debug, Clone)]
pub struct GatewayGrpcClient {
    pub max_signers: u8,
    pub min_signers: u8,
}

impl GatewayGrpcClient {
    pub fn new(max_signers: u8, min_signers: u8) -> Result<Self> {
        Ok(Self {
            max_signers,
            min_signers,
        })
    }

    async fn connect_client(&self) -> anyhow::Result<GatewayClient<Timeout<Channel>>> {
        let endpoint = Endpoint::from_static("https://hardcoded-endpoint.example.com")
            .tls_config(tonic::transport::ClientTlsConfig::new())?;
        let channel = endpoint.connect().await?;
        let timeout_channel = Timeout::new(channel, std::time::Duration::from_millis(5000));
        let client = GatewayClient::new(timeout_channel);
        Ok(client)
    }

    pub async fn sign(&self, label: String, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let req = SignRequest {
            max_signers: (self.max_signers as u32).to_string(),
            min_signers: (self.min_signers as u32).to_string(),
            label,
            message: data,
        };
        let response = self.connect_client().await?.sign(req).await?;
        let res = response.into_inner();
        Ok(res.signature)
    }
}
