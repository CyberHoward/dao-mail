use crate::ContractError;
use cosmwasm_std::{DepsMut, Env, Response};
use cw_authenticator::TrackRequest;

use super::validate_and_parse_params;

pub fn track(
    _deps: DepsMut,
    _env: Env,
    TrackRequest {
        account,
        authenticator_id,
   //     fee_payer,
   //     fee_granter,
   //     fee,
        authenticator_params,
        ..
    }: TrackRequest,
) -> Result<Response, ContractError> {
    let _params = validate_and_parse_params(authenticator_params)?;
    let _key = (&account, authenticator_id.as_str());

    Ok(Response::new().add_attribute("action", "track"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::{
        testing::{mock_dependencies_with_balances, mock_env},
        to_json_binary, Addr, Binary, Coin, Uint128,
    };
    use cw_authenticator::TrackRequest;
    use crate::counter::params::CounterParams;

    #[test]
    fn test_track_success() {
        let mut deps = mock_dependencies_with_balances(&[("addr", &[Coin::new(1000, "uusdc")])]);
        let fee = vec![Coin::new(1000, "uosmo"), Coin::new(1000, "usdc")];
        let track_request = TrackRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            fee_payer: Addr::unchecked("addr"),
            fee_granter: None,
            fee: fee.clone(),
            authenticator_params: Some(
                to_json_binary(&CounterParams {
                    limit: Uint128::new(500_000_000),
                })
                .unwrap(),
            ),
            msg: cw_authenticator::Any {
                type_url: "".to_string(),
                value: Binary::default(),
            },
            msg_index: 0,
        };

        let response = track(deps.as_mut(), mock_env(), track_request).unwrap();
        assert_eq!(response, Response::new().add_attribute("action", "track"));

        // Verify that the pre_exec_balance is updated
//        let key = (&Addr::unchecked("addr"), "2");
//        let pre_exec_balance = PRE_EXEC_BALANCES.load(deps.as_ref().storage, key).unwrap();
//        assert_eq!(pre_exec_balance, vec![Coin::new(1000, "uusdc")]);
//
//        let untracked_spent_fee = UNTRACKED_SPENT_FEES
//            .load(deps.as_ref().storage, key)
//            .unwrap_or_default();
//        assert_eq!(
//            untracked_spent_fee,
//            UntrackedSpentFee {
//                fee,
//                updated_at: mock_env().block.time,
//            }
//        );
    }
}
