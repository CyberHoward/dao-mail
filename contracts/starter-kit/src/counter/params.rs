use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use crate::msg::EmailAuthDetails;

#[cw_serde]
pub struct EmailAuthParams {
    /// Is a static counter to increment
    pub limit: Uint128,
    pub auth: EmailAuthDetails
}

// impl EmailAuthParams { for testing environment only

const TEST_USER_HEADER: &str = r#"
    From: Robin <robin@abstract.money>
    To: Dao <dao@abstract.money>
    Subject: Test Email for DKIM Verification
    Date: Mon, 30 Oct 2024 10:00:00 +0000
    Message-ID: <12345@abstract.money>
"#;

#[cfg(test)]
impl EmailAuthDetails {
    pub fn mock() -> Self {
        Self {
            headers: TEST_USER_HEADER.to_string(),
            signature: "signature".to_string()
        }
    }
}