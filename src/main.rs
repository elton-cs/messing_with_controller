use cainome::cairo_serde::ByteArray;
use messing_with_controller::crosswordle_crosswordle::CrosswordleCrosswordle;
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::types::Felt,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, Url},
    signers::{LocalWallet, SigningKey},
};

const PRIVATE_KEY: &str = "0x33003003001800009900180300d206308b0070db00121318d17b5e6262150b";
const WALLET_ADDRESS: &str = "0x5b6b8189bb580f0df1e6d6bec509ff0d6c9be7365d10627e0cf222ec1b47a71";
const RPC_URL: &str = "https://api.cartridge.gg/x/crosswordle/katana";
const SELECTOR_ADDRESS: &str = "0x2ccc78c905d9fa42a542b8de8c438559ebac7eaecedf5cd932ffbd46fd98729";

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(RPC_URL).unwrap()));
    let chain_id = provider.chain_id().await.unwrap();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(Felt::from_hex_unchecked(
        PRIVATE_KEY,
    )));
    let address = Felt::from_hex_unchecked(WALLET_ADDRESS);

    let account =
        SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);

    let crosswordle =
        CrosswordleCrosswordle::new(Felt::from_hex_unchecked(SELECTOR_ADDRESS), account);

    let guess = ByteArray::from_string("STARK").unwrap();

    // let transaction1 = crosswordle.start_game().send().await.unwrap();
    let transaction2 = crosswordle.submit_guess(&guess).send().await.unwrap();
    println!("{:?}", transaction2);
}
