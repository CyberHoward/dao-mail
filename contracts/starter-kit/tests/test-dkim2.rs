use ring::signature::{UnparsedPublicKey, RSA_PKCS1_2048_8192_SHA256};
use base64;
use pem;
use dkim;

// Correctly ordered and canonicalized headers to sign based on `h=` tag
const HEADERS_TO_SIGN: &str = concat!(
"to: adair@abstract.money\n",
"subject: test\n",
"message-id: <CAN9aTKL3tmj3jS1JWqmcTdSvk91h+ccwjORR6d=TzkLh0aH2jQ@mail.gmail.com>\n",
"date: Sat, 26 Oct 2024 20:40:23 +0400\n",
"from: dao account <dao@abstract.money>\n",
"mime-version: 1.0\n",
"from: dao account <dao@abstract.money>\n",
"to: adair@abstract.money\n",
"cc: \n", // Assuming no cc header; adjust if actual content exists
"subject: test\n",
"date: Sat, 26 Oct 2024 20:40:23 +0400\n",
"message-id: <CAN9aTKL3tmj3jS1JWqmcTdSvk91h+ccwjORR6d=TzkLh0aH2jQ@mail.gmail.com>\n",
"reply-to: \n" // Assuming no reply-to header; adjust if actual content exists
);

// DKIM signature from `b=` tag in `DKIM-Signature` header
const DKIM_SIGNATURE_B: &str = "Obrd9EICo/5bJi7cY6cS34q7KqqdZmJyM5wEentJM1oYcgYC8+h1XRa8C0FJGgyzvLnuIyrBXmG8A0Mxc32I12fuPO5RCS5bldopicsYQzCGaXTJwYs7fTGSzryqTW0CxfMeOl8wtmr97xH0S7dEUVcfQ+PRSmRfXvswpkm8eHU=";

// Public key for verification from DNS TXT record
const PUBLIC_KEY_PEM: &str = r#"
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDMVvsjckzls1f0hGbpvCpdF6Zt
faFs3u/kbciltOa/gQRJ6LoJUtUuTejXfD1z9vyP49XVo+2Cut9XjvqB+jb0AHA5
lROMrZjjJ9gcPvIth+/3tg5+L50Pvm6pLW7Z2NNXyt908ysjXDbBcHA46uk9jlcA
skOyLdn891ScAs9ZsQIDAQAB
-----END PUBLIC KEY-----
"#;

// Function to decode the DKIM signature from base64
fn decode_dkim_signature(signature_b: &str) -> Vec<u8> {
    base64::decode(signature_b).expect("Failed to decode base64 DKIM signature")
}

// Verifies the DKIM signature using the precomputed public key
fn verify_dkim_signature(
    public_key_pem: &str,
    headers_to_sign: &str,
    signature: &[u8],
) -> Result<(), ring::error::Unspecified> {
    let public_key_bytes = pem::parse(public_key_pem)
        .expect("Invalid PEM format")
        .contents;
    let public_key = UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, public_key_bytes);

    // Verify the signature
    public_key.verify(headers_to_sign.as_bytes(), signature)
}

// Test case for verifying only the DKIM signature on headers
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dkim_header_verification() {
        // Decode the DKIM signature at runtime
        let decoded_signature = decode_dkim_signature(DKIM_SIGNATURE_B);

        // Verify DKIM signature
        match verify_dkim_signature(PUBLIC_KEY_PEM, HEADERS_TO_SIGN, &decoded_signature) {
            Ok(_) => println!("DKIM header signature verified successfully."),
            Err(_) => panic!("DKIM header signature verification failed."),
        }
    }
}