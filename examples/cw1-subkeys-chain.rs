use boot_core::instantiate_daemon_env;
use boot_core::BootInstantiate;
use boot_core::BootUpload;
use boot_core::{networks::NetworkInfo, DaemonOptionsBuilder};
pub use cw1_subkeys::msg::{
    ExecuteMsgFns as Cw1SubkeysExecuteMsgFns, QueryMsgFns as Cw1SubkeysQueryMsgFns,
};
use cw1_whitelist::msg::InstantiateMsg;
use dotenv::dotenv;
use std::sync::Arc;

pub const CONTRACT_ID: &str = "Crate_test";
const NETWORK: NetworkInfo = boot_core::networks::juno::UNI_6;

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Create a runtime for the asynchronous actions
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    // Specify the options for the blockchain daemon
    let options = DaemonOptionsBuilder::default()
        // or provide `chain_data`
        .network(NETWORK)
        .deployment_id("my_deployment_version")
        .build()?;

    // Setup the environment
    let (sender, chain) = instantiate_daemon_env(&rt, options)?;

    let mut cw1_subkeys = cw1_subkeys::contract::Cw1Subkeys::new(CONTRACT_ID, chain);

    // We upload and instantiate on-chain
    cw1_subkeys.upload()?;

    let init_msg = InstantiateMsg {
        admins: vec![],
        mutable: true,
    };
    cw1_subkeys.instantiate(&init_msg, None, None)?;

    // Ouuf that passed !

    Ok(())
}
