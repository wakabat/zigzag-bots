#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;

mod zigzag;

use crate::zigzag::{LoginArgs, Operation};
use async_tungstenite::tokio::connect_async;
use clap::{ArgEnum, Parser};
use flexi_logger::Logger;
use futures::prelude::*;
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

    let _ethereum = wallet.ethereum(provider_url).await?;

    // Enable wallet if needed.
    if !wallet.is_signing_key_set().await? {
        log::info!("Setting signing key!");
        let change_pubkey = wallet
            .start_change_pubkey()
            .fee_token("ETH")?
            .send()
            .await?;
        let change_pubkey_receipt = change_pubkey.wait_for_commit().await?;

        if !change_pubkey_receipt.success.unwrap_or(false) {
            log::error!(
                "Change pubkey failure: {:?}",
                change_pubkey_receipt.fail_reason
            );
        }
    }

    let (zigzag_url, zigzag_chainid) = match args.network {
        ArgNetwork::Rinkeby => ("wss://secret-thicket-93345.herokuapp.com", 1000),
        ArgNetwork::Mainnet => ("wss://zigzag-exchange.herokuapp.com", 1),
    };

    let (mut ws_stream, _) = connect_async(zigzag_url).await?;
    log::info!("Connected to zigzag!");

    let login = serde_json::to_string(&Operation::Login(LoginArgs {
        chain_id: zigzag_chainid,
        user_id: wallet.account_id().unwrap().to_string(),
    }))?;
    ws_stream.send(login.into()).await?;

    // Below is the playground now

    Ok(())
}
