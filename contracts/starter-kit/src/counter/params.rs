use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct CounterParams {
    /// Is a static counter to increment
    pub limit: Uint128,
}
