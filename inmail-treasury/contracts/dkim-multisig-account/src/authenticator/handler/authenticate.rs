use cosmwasm_std::{DepsMut, Env, Order, Response};
use cw_authenticator::AuthenticationRequest;

use super::validate_and_parse_params;
use crate::counter::params::EmailAuthParams;
use crate::msg::EmailAuthDetails;
use crate::state::{DKIM_AUTH_CONFIG, DOMAIN, EMAILS};
use crate::ContractError;

pub fn authenticate(
    deps: DepsMut,
    _env: Env,
    auth_request: AuthenticationRequest,
) -> Result<Response, ContractError> {
    let EmailAuthParams {
        auth: email_auth, ..
    } = validate_and_parse_params(auth_request.authenticator_params)?;

    let _ = (
        &auth_request.account,
        auth_request.authenticator_id.as_str(),
    );

    // First, verify that the sender is authorized on the domain
    let dkim_auth_config = DKIM_AUTH_CONFIG.load(deps.storage)?;
    dkim_auth_config.verify_email_auth(&email_auth)?;

    // Next, verify that the sender is a member
    let sender_email = email_auth.get_sender()?;
    EMAILS
        .load(deps.storage, &sender_email)
        .map_err(|_| ContractError::NotMember {
            members: EMAILS
                .range(deps.storage, None, None, Order::Ascending)
                .map(|x| x.unwrap().0)
                .collect::<Vec<_>>()
                .join(","),
            sender: sender_email.clone(),
        })?;

    // TODO HACKATHON: Finally, verify that the member is performing an authorized action...

    Ok(Response::new()
        .add_attribute("action", "authenticate")
        .add_attribute("sender", &sender_email))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::params::EmailAuthParams;
    use cosmwasm_std::{
        testing::{mock_dependencies_with_balances, mock_env},
        to_json_binary, Addr, Binary, Timestamp,
    };
    use cw_authenticator::{Any, SignModeTxData, SignatureData, TxData};
    use rstest::rstest;

    #[rstest]
    #[case::no_time_limit(0, true)]
    fn test_authenticate_time_limit(#[case] current: u64, #[case] expected: bool) {
        // Setup the environment
        let mut deps = mock_dependencies_with_balances(&[("addr", &[])]);

        let _ = (&Addr::unchecked("addr"), "2");

        let request = AuthenticationRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            fee_payer: Addr::unchecked("addr"),
            fee_granter: None,
            fee: vec![],
            authenticator_params: Some(
                to_json_binary(&EmailAuthParams {
                    limit: 1000u128.into(),
                    auth: EmailAuthDetails::mock(),
                })
                .unwrap(),
            ),
            msg: Any {
                type_url: "".to_string(),
                value: Binary::default(),
            },
            msg_index: 0,
            signature: Binary::default(),
            sign_mode_tx_data: SignModeTxData {
                sign_mode_direct: Binary::default(),
                sign_mode_textual: None,
            },
            tx_data: TxData {
                chain_id: "osmosis-1".to_string(),
                account_number: 0,
                sequence: 0,
                timeout_height: 0,
                msgs: vec![],
                memo: "".to_string(),
            },
            signature_data: SignatureData {
                signers: vec![],
                signatures: vec![],
            },
            simulate: false,
        };

        let mut env = mock_env();

        env.block.time = Timestamp::from_nanos(current);

        let response = authenticate(deps.as_mut(), env.clone(), request);

        if expected {
            response.expect("expected authenticated");
        } else {
            //assert_eq!(
            //    response.unwrap_err(),
            //    ContractError::NotWithinTimeLimit {
            //        current: env.block.time,
            //        start,
            //        end,
            //    }
            //);
        }
    }
}
