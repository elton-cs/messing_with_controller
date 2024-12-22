use std::collections::HashMap;

use cainome::rs::{abigen, Abigen};
mod cw;

const PRIVATE_KEY: &str = "0x33003003001800009900180300d206308b0070db00121318d17b5e6262150b";
const WALLET_ADDRESS: &str = "0x5b6b8189bb580f0df1e6d6bec509ff0d6c9be7365d10627e0cf222ec1b47a71";
const RPC_URL: &str = "https://api.cartridge.gg/x/crosswordle/katana";
const SELECTOR_ADDRESS: &str = "0x2ccc78c905d9fa42a542b8de8c438559ebac7eaecedf5cd932ffbd46fd98729";

// abigen!(Crosswordle, "/Users/zkl10/dev/l10labs/crosswordle_contracts/target/release/crosswordle_Crosswordle.contract_class.json");

// abigen!(Crosswordle, "/Users/zkl10/dev/l10labs/crosswordle_contracts/target/release/crosswordle_Crosswordle.contract_class.json", type_aliases {
//     crosswordle::components::gamestart::GameStartComponent::Event as GameStartComponentEvent;
//     crosswordle::components::basewordles::BaseWordlesComponent::Event as BaseWordlesComponentEvent;
// });

#[tokio::main]
async fn main() {
    let mut aliases = HashMap::new();
    aliases.insert(
        String::from("crosswordle::components::gamestart::GameStartComponent::Event"),
        String::from("GameStartComponentEvent"),
    );
    aliases.insert(
        String::from("crosswordle::components::basewordles::BaseWordlesComponent::Event"),
        String::from("BaseWordlesComponentEvent"),
    );
    aliases.insert(
        String::from("crosswordle::systems::crosswordle::Crosswordle::Event"),
        String::from("CrosswordleEvent"),
    );
    aliases.insert(
        String::from("dojo::contract::components::upgradeable::upgradeable_cpt::Event"),
        String::from("DojoUpgradeableEvent"),
    );
    aliases.insert(
        String::from("dojo::contract::components::world_provider::world_provider_cpt::Event"),
        String::from("DojoWorldProviderEvent"),
    );

    let rustgen = Abigen::new("Crosswordle", "/Users/zkl10/dev/l10labs/crosswordle_contracts/target/release/crosswordle_Crosswordle.contract_class.json");
    let _gen_to_file = rustgen
        .with_types_aliases(aliases)
        .generate()
        .unwrap()
        .write_to_file("./src/cw.rs")
        .unwrap();
}
