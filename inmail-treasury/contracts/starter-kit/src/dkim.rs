use crate::authenticator::AuthenticatorError;
use crate::msg::EmailAuthDetails;
use crate::state::{DKIM_AUTH_CONFIG, DOMAIN, PUBLIC_DKIM_KEY};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{StdResult, Storage};

#[cw_serde]
pub struct DkimAuthConfig {
    pub domain: String,
    pub dkim_pk: String,
}

impl DkimAuthConfig {
    pub fn verify_self(&self) -> StdResult<()> {
        // TODO: verify the public key
        Ok(())
    }

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        DKIM_AUTH_CONFIG.save(storage, self)
    }

    pub fn verify_email_auth(
        &self,
        email_auth: &EmailAuthDetails,
    ) -> Result<(), AuthenticatorError> {
        // TODO: implement verification logic here
        println!(
            "Verifying email auth for domain: {} on behalf of sender: {}",
            self.domain,
            email_auth.get_sender()?
        );

        Ok(())
    }
}
