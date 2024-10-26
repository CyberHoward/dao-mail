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
        ExecuteMsg::Execute { msgs } => return Ok(Response::new().add_messages(msgs)),
        ExecuteMsg::AddAuthenticator { params, contract } => {
            let auth_data = CosmwasmAuthenticatorData {
                contract: contract.to_string(),
                params: to_json_binary(&params).unwrap().to_vec(),
            };

            let add_auth_msg = MsgAddAuthenticator {
                sender: env.contract.address.to_string(),
                r#type: "CosmwasmAuthenticatorV1".to_string(),
                data: to_json_binary(&auth_data).unwrap().to_vec(),
            };

            return Ok(Response::new().add_message(add_auth_msg));
        }
    }
    Ok(Response::new().add_attribute("action", "excute"))
}
