use account_sdk::{
    account,
    controller::Controller,
    provider,
    signers::{Owner, Signer},
};
use messing_with_controller::crosswordle_crosswordle::{
    CrosswordleCrosswordle, CrosswordleCrosswordleReader,
};
// use messing_with_controller::crosswordle_crosswordle::CrosswordleCrosswordle;
use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::types::{Call, Felt},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, Url},
    signers::{LocalWallet, SigningKey},
};

const PRIVATE_KEY: &str = "0x33003003001800009900180300d206308b0070db00121318d17b5e6262150b";
const WALLET_ADDRESS: &str = "0x5b6b8189bb580f0df1e6d6bec509ff0d6c9be7365d10627e0cf222ec1b47a71";
const RPC_URL: &str = "https://api.cartridge.gg/x/crosswordle/katana";
const USERNAME: &str = "zkl10";
const APP_ID: &str = "crosswordle";
const CLASS_HASH: &str = "0x45575a88cc5cef1e444c77ce60b7b4c9e73a01cbbe20926d5a4c72a94011410";
const CONTROLLER_ADDRESS: &str =
    "0x022997387672e7c476b338235b4b486b2d1e751d3833f035e514e26c5a80a6c6";
const WORLD_ADDRESS: &str = "0x02f69de741bf133c6ffc205439bebdb5f26f0271dcc1cbb652b02878d33574da";
const SELECTOR_ADDRESS: &str = "0x2ccc78c905d9fa42a542b8de8c438559ebac7eaecedf5cd932ffbd46fd98729";

#[tokio::main]
async fn main() {
    // let secret_scalar = Felt::from_hex_unchecked(PRIVATE_KEY);
    // let owner = Signer::Starknet(SigningKey::from_secret_scalar(secret_scalar));

    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(RPC_URL).unwrap()));
    let chain_id = provider.chain_id().await.unwrap();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(Felt::from_hex_unchecked(
        PRIVATE_KEY,
    )));
    let address = Felt::from_hex_unchecked(WALLET_ADDRESS);

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);

    let crosswordle =
        CrosswordleCrosswordle::new(Felt::from_hex_unchecked(SELECTOR_ADDRESS), account);
    let transaction1 = crosswordle.start_game().send().await.unwrap();
    println!("{:?}", transaction1);

    // let provider_reader = JsonRpcClient::new(HttpTransport::new(Url::parse(RPC_URL).unwrap()));
    // let cwreader = CrosswordleCrosswordleReader::new(
    //     Felt::from_hex_unchecked(SELECTOR_ADDRESS),
    //     provider_reader,
    // );
    // let transaction2 = cwreader.start_game().call().await.unwrap();
    // println!("{:?}", transaction2);
}
