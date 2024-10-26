use cosmwasm_std::{DepsMut, Env, Response};
use cw_authenticator::ConfirmExecutionRequest;

use crate::counter::params::EmailAuthParams;
// use crate::state::COUNTERS;
use crate::ContractError;

use super::validate_and_parse_params;

pub fn confirm_execution(
    mut _deps: DepsMut,
    _env: Env,
    ConfirmExecutionRequest {
        authenticator_params,
        ..
    }: ConfirmExecutionRequest,
) -> Result<Response, ContractError> {
    let _params: EmailAuthParams = validate_and_parse_params(authenticator_params)?;

    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies_with_balances, mock_env},
        to_json_binary, Addr, Binary, Coin, Response, Uint128,
    };
    use cw_authenticator::ConfirmExecutionRequest;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::spend_at_limit(1000, 500, 500, vec![Coin::new(1_000_000_000, "uosmo")], Ok(Response::new()))]
    fn test_confirm_execution(
        #[case] initial_balance: u128,
        #[case] limit: u128,
        #[case] spent: u128,
        #[case] _untracked_spent_fee: Vec<Coin>,
        #[case] expected: Result<Response, ContractError>,
    ) {
        let fixed_balance = Coin::new(500, "uosmo");
        // Setup the environment
        let mut deps = mock_dependencies_with_balances(&[(
            "account",
            &[
                Coin::new(initial_balance - spent, "uusdc"),
                fixed_balance.clone(),
            ],
        )]);

        let _key = (&Addr::unchecked("account"), "2");

        // Confirm the execution
        let confirm_execution_request = ConfirmExecutionRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("account"),
            fee_payer: Addr::unchecked("account"),
            fee_granter: None,
            fee: vec![],
            authenticator_params: Some(
                to_json_binary(&EmailAuthParams {
                    limit: Uint128::new(limit),
                })
                .unwrap(),
            ),
            msg: cw_authenticator::Any {
                type_url: "".to_string(),
                value: Binary::default(),
            },
            msg_index: 0,
        };

        let res = confirm_execution(deps.as_mut(), mock_env(), confirm_execution_request);
        match expected {
            Ok(expected_res) => {
                assert_eq!(res.unwrap(), expected_res);

                // Verify that the spending is updated correctly
                //let spending = SPENDINGS.load(deps.as_ref().storage, key).unwrap();
                //assert_eq!(
                //    spending,
                //    Spending {
                //        value_spent_in_period: spent.into(),
                //        last_spent_at: mock_env().block.time
                //    }
                //);

                //// verify that the untracked spent fee is cleaned up
                //let untracked_spent_fee = UNTRACKED_SPENT_FEES
                //    .may_load(deps.as_ref().storage, key)
                //    .unwrap();
                //assert_eq!(untracked_spent_fee, None);
            }
            Err(expected_err) => {
                assert_eq!(res.unwrap_err(), expected_err);

                // verify that untracked spent fee is not cleaned up
                //assert_eq!(
                //    UNTRACKED_SPENT_FEES
                //        .may_load(deps.as_ref().storage, key)
                //        .unwrap(),
                //    Some(UntrackedSpentFee {
                //        fee: untracked_spent_fee,
                //        updated_at: mock_env().block.time,
                //    })
                //);
            }
        }
    }
}
