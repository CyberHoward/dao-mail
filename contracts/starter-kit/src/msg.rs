use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint64;

// re-export the structs from cw_authenticator
pub use cw_authenticator::AuthenticatorSudoMsg as SudoMsg;

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    #[returns(CounterResponse)]
    Counter {
        account: String,
        authenticator_id: String,
    },
}

#[cw_serde]
pub struct CounterResponse {
    pub count: Uint64,
}
