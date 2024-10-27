use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Empty, StdResult, Uint64};
use cw3::Vote;
// re-export the structs from cw_authenticator
use crate::counter::params::EmailAuthParams;
use crate::dkim::DkimAuthConfig;
pub use cw_authenticator::AuthenticatorSudoMsg as SudoMsg;
use cw_ownable::cw_ownable_execute;
use crate::authenticator::AuthenticatorError;
use crate::ContractError;

pub type EmailAddress = String;

// TODO: maybe everything should be in params????
#[cw_serde]
pub struct InstantiateMsg {
    pub params: EmailAuthParams,
    pub domain_auth: DkimAuthConfig,
    pub member_emails: Vec<EmailAddress>
}

/// Key authentication information from the email to be able to verify that the sender is part of our domain.
#[cw_serde]
pub struct EmailAuthDetails {
    pub headers: String,
    pub signature: String
}

impl EmailAuthDetails {
pub fn get_sender(&self) -> Result<EmailAddress, AuthenticatorError> {
    if let Some(message) = mail_parser::MessageParser::default()
        .parse(&self.headers.clone()) {
        // TODO HACKATHON: fix this thing
        if let Some(from) = message.from() {
            from.first().ok_or(AuthenticatorError::NoSenderFound(self.headers.clone()))?
        } else {
            println!("No sender found in headers: {}", self.headers);
            return Err(AuthenticatorError::NoSenderFound(self.headers.clone()));
        }
    } else {
        return Err(AuthenticatorError::FailedToParseHeaders(self.headers.clone()));
    };


    Ok("test@abstract.money".to_string())
}
}

#[cfg(test)]
mod email_auth_details_test {
    use crate::counter::params::TEST_USER_HEADER;
    use super::*;

    #[test]
    fn test_get_sender() {
        let test_details = EmailAuthDetails {
            headers: TEST_USER_HEADER.to_string(),
            signature: "garbage".to_string()
        };

        test_details.get_sender().unwrap();
    }
}

#[cw_ownable_execute]
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Count {},
    Propose {
        auth: EmailAuthDetails,
        title: String,
        description: String,
        msgs: Vec<CosmosMsg<Empty>>,
    },
    Vote {
        auth: EmailAuthDetails,
        proposal_id: u64,
        vote: Vote,
    },
    #[cw_orch(fn_name("execute_proposal"))]
    Execute {
        auth: EmailAuthDetails,
        proposal_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    #[returns(CounterResponse)]
    Counter {
        account: String,
        authenticator_id: String,
    },
}

#[cw_serde]
pub struct CounterResponse {
    pub count: Uint64,
}
