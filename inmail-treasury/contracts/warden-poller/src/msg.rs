use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Empty};
use cw_utils::Expiration;
use serde::Serialize;

#[cw_serde]
pub struct InstantiateMsg {
    pub dns_server: String,
}

#[cfg(test)]
impl InstantiateMsg {
    pub fn new(dns_server: impl Into<String>) -> Self {
        InstantiateMsg {
            dns_server: dns_server.into(),
        }
    }
}

#[cw_serde]
pub struct WardenRequestFuture {
    pub host: String,
    pub request: String,
    pub params: Vec<String>,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    StartDkimTxtRequestFuture { domain: String },
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetDnsServerResponse)]
    GetDnsServer {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetDnsServerResponse {
    pub host: String,
}
