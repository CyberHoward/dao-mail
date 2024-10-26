use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use crate::msg::EmailAuthDetails;

#[cw_serde]
pub struct EmailAuthParams {
    /// Is a static counter to increment
    pub limit: Uint128,
    // pub auth: EmailAuthDetails
}
