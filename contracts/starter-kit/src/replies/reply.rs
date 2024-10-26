use crate::replies::instantiate_reply::reply_on_instantiate;
use crate::ContractError;
use cosmwasm_std::{DepsMut, Env, Reply, Response};

pub const INSTANTIATE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INSTANTIATE_REPLY_ID => Ok(reply_on_instantiate(deps, env, reply)?),
        _ => Err(ContractError::InvalidReplyId(reply.id)),
    }
}
