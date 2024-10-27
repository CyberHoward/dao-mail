use cw_orch::{anyhow, prelude::*};
use dkim_auth::{
    interface::DkimAuthI,
    msg::{ExecuteMsgFns, InstantiateMsg, QueryMsgFns},
};

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok(); // Used to load the `.env` file if any
    env_logger::init(); // Used to log contract and chain interactions

    let network = networks::PION_1;
    let chain = DaemonBuilder::new(network.clone()).build()?;

    let counter = DkimAuthI::new(chain);

    counter.upload()?;

    let msg = InstantiateMsg { count: 1i32 };
    counter.instantiate(&msg, None, &[])?;

    counter.increment()?;
    let count = counter.get_count()?;
    assert_eq!(count.count, 1);
    Ok(())
}
