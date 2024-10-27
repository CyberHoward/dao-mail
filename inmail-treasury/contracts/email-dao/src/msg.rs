use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Empty};
use cw_utils::Expiration;
use serde::Serialize;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
#[serde(rename_all = "lowercase")]
pub enum Vote {
    /// Marks support for the proposal.
    Yes,
    /// Marks opposition to the proposal.
    No,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    DoExecute { msgs: Vec<CosmosMsg> },
    Propose {
        subject: String,
        body: String,
        msgs: Vec<CosmosMsg<Empty>>,
    },
    Vote {
        proposal_id: u64,
        vote: Vote,
    },
    #[cw_orch(fn_name("execute_proposal"))]
    Execute {
        proposal_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
