use std::{fmt, net::SocketAddr, str::FromStr};

use anyhow::Result;
use rand::thread_rng;

use proto_api::mpc_gateway::{
    gateway_server::{Gateway, GatewayServer},
    CreateReply, CreateRequest, SignReply, SignRequest,
};

use tonic::{Request, Response, Status};

pub struct Dealer {
    pub grpc_address: String,
    pub grpc_port: u16,
    pub timeout: Option<u64>,
    pub concurrent: Option<usize>,
}

impl Dealer {
    pub fn new(
        grpc_address: String,
        grpc_port: u16,
        timeout: Option<u64>,
        concurrent: Option<usize>,
    ) -> Self {
        Self {
            grpc_address,
            grpc_port,
            timeout,
            concurrent,
        }
    }
    pub async fn start(self) -> Result<()> {
        let grpc_hostname = std::net::Ipv4Addr::from_str(&self.grpc_address)?;
        let grpc_addr = SocketAddr::V4(std::net::SocketAddrV4::new(grpc_hostname, self.grpc_port));

        let timeout = std::time::Duration::new(self.timeout.unwrap_or(30), 0);
        let concurrency = self.concurrent.unwrap_or(30);

        let dealer_service = GatewayServer::new(self);

        tonic::transport::Server::builder()
            .concurrency_limit_per_connection(concurrency)
            .timeout(timeout)
            .add_service(dealer_service)
            .serve(grpc_addr)
            .await?;

        Ok(())
    }
}

impl fmt::Debug for Dealer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dealer_grpc_server")
    }
}

#[tonic::async_trait]
impl Gateway for Dealer {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateReply>, Status> {
        let req = request.into_inner();
        Ok(Response::new(CreateReply { label: req.label }))
    }

    async fn sign(&self, request: Request<SignRequest>) -> Result<Response<SignReply>, Status> {
        let req = request.into_inner();
        // === Generate FROST key shares ===
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

        let reply = SignReply {
            label: req.label,
            signature: vec![], //signature,
        };
        Ok(Response::new(reply))
    }
}
