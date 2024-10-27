use crate::authenticator::CosmwasmAuthenticatorData;
use crate::msg::ExecuteMsg;
use crate::state::COUNT_TEST;
use crate::ContractError;
use cosmwasm_std::{entry_point, to_json_binary, DepsMut, Env, MessageInfo, Response};
use osmosis_std::types::osmosis::smartaccount::v1beta1::MsgAddAuthenticator;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwnership(action) => {
            cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
        }
        ExecuteMsg::Count {} => {
            COUNT_TEST.update(deps.storage, |count| -> Result<_, ContractError> {
                Ok(count + 1)
            })?;
        }
        ExecuteMsg::Execute {
            auth: _,
            proposal_id,
        } => {
            crate::cw3::cw3_execute::execute_execute(deps, env, info, proposal_id)?;
        }
        ExecuteMsg::Propose {
            auth,
            title,
            description,
            msgs,
        } => {
            let sender_email = auth.get_sender()?;
            crate::cw3::cw3_execute::execute_propose(
                deps,
                env,
                info,
                sender_email,
                title,
                description,
                msgs,
            )?;
        }
        ExecuteMsg::Vote {
            auth,
            proposal_id,
            vote,
        } => {
            let sender_email = auth.get_sender()?;
            crate::cw3::cw3_execute::execute_vote(
                deps,
                env,
                info,
                sender_email,
                proposal_id,
                vote,
            )?;
        }
    }
    Ok(Response::new().add_attribute("action", "excute"))
}
