use warden_poller::{
    interface::WardenPollerI,
    msg::{GetDnsServerResponse, QueryMsg},
    ContractError,
};
// Use prelude to get all the necessary imports
use cw_orch::anyhow;
use cw_orch::prelude::*;

// constants for testing
const USER: &str = "user";
const ADMIN: &str = "admin";

pub const MOCK_DNS_SERVER: &str = "https://api.cloudflare.com";

#[test]
fn count() -> anyhow::Result<()> {
    // Create the mock. This will be our chain object throughout
    let mock = Mock::new(ADMIN);

    let user = mock.addr_make(USER);

    // Set up the contract
    let contract = WardenPollerI::setup(mock.clone(), mock.sender().clone(), MOCK_DNS_SERVER.into())?;

    // Increment the count of the contract
    contract
        // Set the caller to user
        .call_as(&user)
        // Call the increment function (auto-generated function provided by ExecuteMsgFns)
        .increment()?;

    // Get the count.
    use warden_poller::msg::QueryMsgFns;
    let count1 = contract.get_count()?;

    // or query it manually
    let count2: GetDnsServerResponse = contract.query(&QueryMsg::GetDnsServer {})?;
    assert_eq!(count1.count, count2.count);

    // Or get it manually from the chain
    let count3: GetDnsServerResponse =
        mock.query(&QueryMsg::GetDnsServer {}, &contract.address()?)?;
    assert_eq!(count1.count, count3.count);

    // Check the count
    assert_eq!(count1.count, 2);
    // Reset
    use warden_poller::msg::ExecuteMsgFns;
    contract.reset(0)?;

    let count = contract.get_count()?;
    assert_eq!(count.count, 0);

    // Check negative case
    let exec_res: Result<cw_orch::mock::cw_multi_test::AppResponse, CwOrchError> =
        contract.call_as(&user).reset(0);

    let expected_err = ContractError::Unauthorized {};
    assert_eq!(
        exec_res.unwrap_err().downcast::<ContractError>()?,
        expected_err
    );

    Ok(())
}
