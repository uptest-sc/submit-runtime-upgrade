use libuptest::metadata::read_wasm_binary;
use libuptest::ws_mod::get_runtime_version;
use libuptest::jsonrpseeclient::JsonrpseeClient;

use std::path::Path;
use subxt::{OnlineClient, PolkadotConfig};

use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_url = "ws://127.0.0.1:9944")]
pub mod nodetemplate {}

use nodetemplate::runtime_types::sp_weights::weight_v2::Weight;

//type Call = nodetemplate::Call;//nodetemplate::runtime_types::node_template_runtime::Call; //nodetemplate::Call;
//type Call = nodetemplate::runtime_types::node_template_runtime::Call;
type SystemCall = nodetemplate::runtime_types::frame_system::pallet::Call;

#[tokio::main]
async fn main() {
    // define wasm path
    let wasm_path = Path::new("/tmp/substrate-node-template/target/release/wbuild/node-template-runtime/node_template_runtime.compact.wasm");
    // read binary
    let wasm_binary: Option<&[u8]> = Some(include_bytes!("/tmp/substrate-node-template/target/release/wbuild/node-template-runtime/node_template_runtime.compact.compressed.wasm"));
    // this is wrong and left unfixed until it goes upstream
    let _code: Vec<u8> = vec![read_wasm_binary(wasm_path).await.unwrap()];
    // create system set_code call
    let call = nodetemplate::runtime_types::node_template_runtime::RuntimeCall::System(
        SystemCall::set_code {
            code: wasm_binary.expect("could not decode wasm binary").into(),
        },
    ); //Call::System(
    let weight = Weight {
        ref_time: 0,
        proof_size: 0,
    };
    let apij = JsonrpseeClient::with_default_url().expect("Could not connect to 127.0.0.1:9944");
    let old_runtime_version = get_runtime_version(apij.clone()).await.expect("Could not detect runtimeversion").spec_version;
    println!("Current Runtime Version: {:?}", old_runtime_version);

    // create the sudo tx
    let sudo_tx = nodetemplate::tx()
        .sudo()
        .sudo_unchecked_weight(call, weight);
    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    // alice is the sudo key holder
    let from = dev::alice();
    println!("sending tx");
    // send tx and print the blockhash
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&sudo_tx, &from)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
    println!("Sent tx: {:?}", events.block_hash());
    println!("Runtime is now");
    let runtime_version = get_runtime_version(apij).await.expect("Could not detect runtimeversion");
    println!("Runtime Version changed from {:?} to {:?}", runtime_version.spec_version, old_runtime_version);
}

