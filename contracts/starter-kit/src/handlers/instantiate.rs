use cosmwasm_std::{DepsMut, entry_point, Env, MessageInfo, Response, to_json_binary};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::smartaccount::v1beta1::MsgAddAuthenticator;

use crate::authenticator::CosmwasmAuthenticatorData;
use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};
use crate::ContractError;
use crate::counter::params::EmailAuthParams;
use crate::msg::InstantiateMsg;
use crate::state::EMAILS;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;

    // verify the domain public keys
    msg.domain_auth.verify_self()?;
    msg.domain_auth.save(deps.storage)?;

    setup_mulitsig(deps, &msg)?;

    let add_auth_msg = add_self_authenticator(env, &msg.params);


    Ok(
        Response::new()
            .add_attribute("action", "instantiate")
            .add_message(add_auth_msg), // Ok(Response::new().add_attribute("action", "instantiate").add_submessage(SubMsg::reply_always(add_auth_msg, INSTANTIATE_REPLY_ID))
                                        //     .add_submessage(SubMsg::new(wasm_execute(env.contract.address, &ExecuteMsg::Execute {
                                        //     msgs: vec![add_auth_msg.into()],
                                        // }, vec![])?))
    )
}

/// Setup the members on the multisig
/// Note that we override the weight to each member to 1
fn setup_mulitsig(deps: DepsMut, msg: &InstantiateMsg) -> Result<(), ContractError> {
    if msg.member_emails.is_empty() {
        return Err(ContractError::NoVoters {});
    }

    let total_weight = msg.member_emails.len() as u64;

    cw3_fixed_multisig::state::CONFIG.save(
        deps.storage,
        &cw3_fixed_multisig::state::Config {
            // Everyone should vote
            threshold: cw_utils::Threshold::AbsoluteCount {
                weight: total_weight,
            },
            total_weight,
            // A lot of blocks
            max_voting_period: cw_utils::Duration::Height(100000000000000000),
        },
    )?;

    // add all voters
    for email in msg.member_emails.iter() {
        EMAILS.save(deps.storage, &email, &1)?;
    }

    Ok(())
}

fn add_self_authenticator(env: Env, params: &EmailAuthParams) -> MsgAddAuthenticator {
    let auth_data = CosmwasmAuthenticatorData {
        contract: env.contract.address.to_string(),
        params: to_json_binary(params).unwrap().to_vec(),
    };

    let add_auth_msg = MsgAddAuthenticator {
        sender: env.contract.address.to_string(),
        r#type: "CosmwasmAuthenticatorV1".to_string(),
        data: to_json_binary(&auth_data).unwrap().to_vec(),
    };
    add_auth_msg
}
