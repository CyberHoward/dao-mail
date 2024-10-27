use ring::digest::{digest, SHA256};
use ring::signature::{UnparsedPublicKey, RSA_PKCS1_2048_8192_SHA256};

// Example email body and DKIM header data
const EMAIL_BODY: &str = "body";
// DKIM-Signature b=
const ENCODED_DKIM_SIGNATURE: &str = "Obrd9EICo/5bJi7cY6cS34q7KqqdZmJyM5wEentJM1oYcgYC8+h1XRa8C0FJGgyzvLnuIyrBXmG8A0Mxc32I12fuPO5RCS5bldopicsYQzCGaXTJwYs7fTGSzryqTW0CxfMeOl8wtmr97xH0S7dEUVcfQ+PRSmRfXvswpkm8eHU=";
// DKIM-Signature bh=
const DKIM_BODY_HASH: &str = "Bxv3VDj2sR90cDRYylWgRfEM8mvoPM1xB5NTdo0G/WI=";
const PUBLIC_KEY_PEM: &str = r#"
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDMVvsjckzls1f0hGbpvCpdF6ZtfaFs3u/kbciltOa/gQRJ6LoJUtUuTejXfD1z9vyP49XVo+2Cut9XjvqB+jb0AHA5lROMrZjjJ9gcPvIth+/3tg5+L50Pvm6pLW7Z2NNXyt908ysjXDbBcHA46uk9jlcAskOyLdn891ScAs9ZsQIDAQAB
-----END PUBLIC KEY-----
"#;

fn base64_decode(input: &str) -> Vec<u8> {
    base64::decode(input).expect("Failed to decode base64")
}

// Canonicalize the body according to DKIM's relaxed canonicalization rules
fn canonicalize_body(body: &str) -> String {
    // Remove trailing empty lines, and process each line for relaxed canonicalization
    let trimmed_body = body.trim_end_matches(|c| c == '\r' || c == '\n');

    let canonicalized = trimmed_body
        .lines()
        .map(|line| line.trim_end()) // Trim each line's trailing whitespace
        .collect::<Vec<_>>() // Collect lines as a vector
        .join("\r\n"); // Join with CRLF

    // Debug output for troubleshooting
    println!("Canonicalized Body:\n{}", canonicalized);

    canonicalized
}

// Hashes the email body to verify the `bh=` header in DKIM
fn hash_email_body(email_body: &str) -> String {
    // Canonicalize the body
    let canonicalized_body = canonicalize_body(email_body);

    // Generate the SHA-256 hash
    let body_hash = digest(&SHA256, canonicalized_body.as_bytes());

    // Debug output for troubleshooting
    println!(
        "Computed SHA-256 Hash (Base64): {}",
        base64::encode(body_hash.as_ref())
    );

    // Encode in base64 to match DKIM's `bh=` value format
    base64::encode(body_hash.as_ref())
}

// Hashes the email body in raw bytes for direct comparison with the `bh=` value
fn hash_email_body_raw(email_body: &str) -> Vec<u8> {
    // Canonicalize the body
    let canonicalized_body = canonicalize_body(email_body);

    // Generate the SHA-256 hash and return raw bytes
    digest(&SHA256, canonicalized_body.as_bytes())
        .as_ref()
        .to_vec()
}

// // Hashes the email body to verify the `bh=` header in DKIM
// fn hash_email_body(email_body: &[u8]) -> String {
//     let body_hash = digest(&SHA256, email_body);
//     base64::encode(body_hash.as_ref())
// }

// Verifies the DKIM signature using a precomputed public key
fn verify_dkim_signature(
    public_key_pem: &str,
    dkim_headers: &[u8],
    signature: &[u8],
) -> Result<(), ring::error::Unspecified> {
    let public_key_bytes = pem::parse(public_key_pem)
        .expect("Invalid PEM format")
        .contents;
    let public_key = UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, public_key_bytes);

    // Verify the signature
    public_key.verify(dkim_headers, signature)
}

// Test case for the PoC
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dkim_body_hash_verification_raw() {
        // Provided DKIM `bh=` value (Base64 encoded SHA-256 hash)
        let dkim_body_hash_b64 = "Bxv3VDj2sR90cDRYylWgRfEM8mvoPM1xB5NTdo0G/WI=";

        // Decode the base64 `bh=` value to get the expected raw hash bytes
        let dkim_body_hash_raw =
            base64::decode(dkim_body_hash_b64).expect("Failed to decode base64 `bh=` value");

        // Compute the hash of the canonicalized email body in raw bytes
        let computed_body_hash_raw = hash_email_body_raw("body"); // assuming plain text `body`

        // Debug output for verification
        println!(
            "Expected (Decoded `bh=` Raw Hash): {:?}",
            dkim_body_hash_raw
        );
        println!("Computed Body Hash Raw Bytes: {:?}", computed_body_hash_raw);

        // Compare the decoded `bh=` hash with our computed body hash
        assert_eq!(
            computed_body_hash_raw, dkim_body_hash_raw,
            "Computed body hash does not match the decoded `bh=` value"
        );
    }

    #[test]
    fn test_dkim_verification() {
        // Step 1: Verify body hash matches `bh=` value
        // let computed_body_hash = hash_email_body(EMAIL_BODY);
        // assert_eq!(computed_body_hash, DKIM_BODY_HASH, "Body hash does not match DKIM `bh=` value");

        let dkim_signature = base64_decode(ENCODED_DKIM_SIGNATURE);

        // Step 2: Verify DKIM signature
        let headers_to_sign = b"from:robin@abstract.money\nsubject:test"; // Simplified for PoC
        match verify_dkim_signature(PUBLIC_KEY_PEM, headers_to_sign, &dkim_signature) {
            Ok(_) => println!("DKIM signature verified successfully."),
            Err(_) => panic!("DKIM signature verification failed."),
        }
    }
}
