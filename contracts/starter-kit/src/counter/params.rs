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


pub const TEST_USER_HEADER: &str = r#"Delivered-To: adair@abstract.money
Received: by 2002:a05:7022:220d:b0:8b:d1b7:49b5 with SMTP id bu13csp626687dlb;
 Sat, 26 Oct 2024 09:40:35 -0700 (PDT)
X-Received: by 2002:a05:6a21:9203:b0:1d8:a67b:9224 with SMTP id adf61e73a8af0-1d9a8533f7dmr5073029637.34.1729960835474;
 Sat, 26 Oct 2024 09:40:35 -0700 (PDT)
Return-Path: <dao@abstract.money>
Received: from mail-sor-f41.google.com (mail-sor-f41.google.com. [209.85.220.41])
 by mx.google.com with SMTPS id d2e1a72fcca58-72057925a98sor2014470b3a.1.2024.10.26.09.40.35
 for <adair@abstract.money>
 (Google Transport Security);
 Sat, 26 Oct 2024 09:40:35 -0700 (PDT)
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed;
 d=abstract.money; s=google; t=1729960835; x=1730565635; darn=abstract.money;
 h=to:subject:message-id:date:from:mime-version:from:to:cc:subject
 :date:message-id:reply-to;
 bh=Bxv3VDj2sR90cDRYylWgRfEM8mvoPM1xB5NTdo0G/WI=;
 b=Obrd9EICo/5bJi7cY6cS34q7KqqdZmJyM5wEentJM1oYcgYC8+h1XRa8C0FJGgyzvL
 nuIyrBXmG8A0Mxc32I12fuPO5RCS5bldopicsYQzCGaXTJwYs7fTGSzryqTW0CxfMeOl
 8wtmr97xH0S7dEUVcfQ+PRSmRfXvswpkm8eHU=
X-Received: by 2002:a17:90a:f2d3:b0:2dd:5e86:8c2f with SMTP id 98e67ed59e1d1-2e8f107d4bdmr3712857a91.21.1729960835088; Sat, 26 Oct 2024 09:40:35 -0700 (PDT)
MIME-Version: 1.0
From: dao account <dao@abstract.money>
Date: Sat, 26 Oct 2024 20:40:23 +0400
Message-ID: <CAN9aTKL3tmj3jS1JWqmcTdSvk91h+ccwjORR6d=TzkLh0aH2jQ@mail.gmail.com>
Subject: test
To: adair@abstract.money
Content-Type: multipart/alternative; boundary="0000000000000d2866062563e55d"

--0000000000000d2866062563e55d
Content-Type: text/plain; charset="UTF-8"

body

--0000000000000d2866062563e55d
Content-Type: text/html; charset="UTF-8"

<div dir="ltr">body</div>

--0000000000000d2866062563e55d--
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