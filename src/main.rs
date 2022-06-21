use clap::{ArgEnum, Parser};
use flexi_logger::Logger;
use std::fs;
use zksync::{provider::RpcProvider, zksync_types::H256, Network, Wallet, WalletCredentials};
use zksync_eth_signer::{EthereumSigner, PrivateKeySigner};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(long)]
    private_key: Option<String>,

    #[clap(long)]
    private_key_file: Option<String>,

    #[clap(long, arg_enum, value_parser, default_value_t = ArgNetwork::Rinkeby)]
    network: ArgNetwork,

    #[clap(long)]
    provider_url: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum ArgNetwork {
    Rinkeby,
    Mainnet,
}

impl From<ArgNetwork> for Network {
    fn from(n: ArgNetwork) -> Self {
        match n {
            ArgNetwork::Rinkeby => Network::Rinkeby,
            ArgNetwork::Mainnet => Network::Mainnet,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Logger::try_with_env()?.start()?;

    let args = Args::parse();

    let raw_private_key = if let Ok(val) = std::env::var("ETH_PRIVKEY") {
        val
    } else if let Some(key) = args.private_key {
        key
    } else if let Some(file) = args.private_key_file {
        fs::read_to_string(file)?
    } else {
        return Err(anyhow::anyhow!("Please specify private key either via ETH_PRIVKEY environment variable, or one of the cli arguments!"));
    }.trim().to_owned();
    // TODO: add support for mnemonic formatted private keys, right now only raw private
    // keys are supported.
    let private_key = if raw_private_key.len() == 64 {
        let mut data = [0u8; 32];
        hex::decode_to_slice(&raw_private_key, &mut data[..])?;
        H256(data)
    } else {
        return Err(anyhow::anyhow!("Private key is not in a valid format!"));
    };

    let provider = RpcProvider::new(args.network.into());
    let eth_signer = PrivateKeySigner::new(private_key);
    let address = eth_signer.get_address().await?;
    let credential =
        WalletCredentials::from_eth_signer(address, eth_signer, args.network.into()).await?;

    let wallet = Wallet::new(provider, credential).await?;

    let provider_url = if let Ok(val) = std::env::var("ETH_PROVIDER_URL") {
        val
    } else if let Some(val) = args.provider_url {
        val
    } else {
        return Err(anyhow::anyhow!("Please specify ethereum provider URL via ETH_PROVIDER_URL environment variable or a cli argument!"));
    }.trim().to_owned();

    let ethereum = wallet.ethereum(provider_url).await?;

    // Below is the playground now

    Ok(())
}
