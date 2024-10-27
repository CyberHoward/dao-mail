use cw_orch::daemon::networks::{OSMOSIS_1, PION_1};
use cw_orch::{anyhow, prelude::*};
use warden_poller::{
    interface::WardenPollerI,
    msg::{ExecuteMsgFns, InstantiateMsg, QueryMsgFns},
};

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok(); // Used to load the `.env` file if any
    env_logger::init(); // Used to log contract and chain interactions

    let network: ChainInfo = ChainInfo {
        // grpc_urls: &["https://497eb440-0152-48f8-85ff-f0cf7c8a5632.osmo-test-5.mesa-grpc.newmetric.xyz:443"],
        ..networks::OSMO_5
    };

    // let network = OSMOSIS_1;
    // let network = PION_1;

    let chain = DaemonBuilder::new(network.clone()).build()?;

    let counter = WardenPollerI::new(chain);

    counter.upload()?;

    let msg = InstantiateMsg { count: 1i32 };
    counter.instantiate(&msg, None, &[])?;

    counter.increment()?;
    let count = counter.get_count()?;
    assert_eq!(count.count, 1);
    Ok(())
}
