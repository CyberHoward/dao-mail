use cosmwasm_std::{DepsMut, Env, Response};
use cw_authenticator::AuthenticationRequest;

use crate::ContractError;

use super::validate_and_parse_params;

pub fn authenticate(
    _deps: DepsMut,
    _env: Env,
    auth_request: AuthenticationRequest,
) -> Result<Response, ContractError> {
    let _ = validate_and_parse_params(auth_request.authenticator_params)?;

    let _ = (
        &auth_request.account,
        auth_request.authenticator_id.as_str(),
    );

    Ok(Response::new().add_attribute("action", "authenticate"))
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
