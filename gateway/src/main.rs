use gateway::GatewayGrpcClient;
mod gateway;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "mpc-gateway-cli")]
#[clap(version = "1.0")]
#[clap(about = "SOVA MPC-Gateway command line tool", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[arg(long)]
    max_signers: u8,
    #[arg(long)]
    min_signers: u8,
}

#[derive(Subcommand)]
enum Commands {
    Sign { label: String, message: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let client = GatewayGrpcClient::new(args.max_signers, args.max_signers)?;

    match args.command {
        Commands::Sign { label, message } => {
            log::info!(
                "executing Sign command, label: {}, message: {}",
                label,
                message
            );

            let message_bytes = hex::decode(message).expect("hexadecimal string");
            let sig = client.sign(label.clone(), message_bytes).await?;
            println!("Signature: {:?}", hex::encode(sig));
        }
    };

    Ok(())
}
