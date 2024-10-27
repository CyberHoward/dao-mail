use base64::engine::general_purpose;
use base64::Engine as _;
use chrono::{Duration, Utc};
use core::str;
use futures::StreamExt;
use google_gmail1::api::{Message, MessagePart};
use google_gmail1::yup_oauth2::{
    AccessTokenAuthenticator, ApplicationSecret, ConsoleApplicationSecret,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Display;
use tokio::net::TcpStream;

#[derive(Debug)]
struct GmailOAuth2 {
    user: String,
    access_token: String,
}

impl async_imap::Authenticator for GmailOAuth2 {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&mut self, data: &[u8]) -> Self::Response {
        format!("user={}^Aauth=Bearer {}^A^A", self.user, self.access_token)
    }
}

async fn email_client() -> anyhow::Result<()> {
    extern crate google_gmail1 as gmail1;
    use gmail1::api::Message;
    use gmail1::{hyper_rustls, hyper_util, yup_oauth2, FieldMask, Gmail};
    use gmail1::{Error, Result};
    use std::fs;

    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    let file = File::open("oauth/token.json")?;
    let reader = BufReader::new(file);
    let token_json: Value = serde_json::from_reader(reader)?;
    let token = token_json["access_token"].as_str().unwrap();

    let auth = AccessTokenAuthenticator::builder(token.into())
        .build()
        .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    let mut hub = Gmail::new(client, auth);
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let mut req = Message::default();

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `upload_resumable(...)`.
    let (_, list) = hub
        .users()
        .messages_list("me")
        .add_label_ids("UNREAD")
        .doit()
        .await?;

    for msg in list.messages.unwrap_or_default() {
        let (_, m) = hub
            .users()
            .messages_get("me", &msg.id.unwrap())
            .doit()
            .await?;
        let payload = m.payload.as_ref().unwrap();

        // Get the email subject
        let subject = payload.subject();

        // Initial proposal logic
        if subject.starts_with("Proposal:") {
            let body = payload.body();

            println!("------####  New Proposal:   ####------");
            println!("Subject: {}", subject);
            println!("Body: {}", body);

            let payment = gpt::process_body(body).await?;

            if let Some(payment) = payment {
                println!("Recipient: {}", payment.recipient);
                println!("Amount: {}", payment.amount);
                println!("Denomination: {}", payment.denomination);

                // Execute the on-chain proposal
                warden_poller::propose(subject, body.to_string(), payment).await?;
            }

            // // Extract details from the existing message
            // let thread_id = m.thread_id.as_deref().unwrap_or("");
            // let message_id = m.id.as_deref().unwrap_or("");
            // let recipient = extract_recipient(&m)?;

            // eprintln!("Recipient: {}", recipient);
            // eprintln!("Thread ID: {}", thread_id);
            // eprintln!("Message ID: {}", message_id);

            // // Create the reply message
            // let reply_body = "This is a reply to your email!";
            // let reply_message = Message {
            //     thread_id: Some(thread_id.to_string()),
            //     raw: Some(create_reply_raw(
            //         &recipient,
            //         "Re: Your Subject",
            //         reply_body,
            //         message_id,
            //     )?.as_bytes().to_vec()),
            //     ..Default::default()
            // };

            // // https://github.com/PgBiel/mentoriabot/blob/main/crates/lib/src/notification/email.rs
            // let message_buffer = tempfile::tempfile().unwrap();
            // const EMAIL_MIMETYPE: &str = "message/rfc822";

            // // Send the reply message
            // let result = hub
            //     .users()
            //     .messages_send(reply_message, "me")
            //     .upload(message_buffer, EMAIL_MIMETYPE.parse().unwrap())
            //     .await?;

            // This is the vote logic
        } else if subject.starts_with("Re: ") {
            let mut cleaned_subject = subject.to_string();
            let prefix = "Re: ";

            while cleaned_subject.starts_with(prefix) {
                cleaned_subject = cleaned_subject[prefix.len()..].to_string();
            }

            println!("------####  New Vote:   ####------");
            println!("Subject: {}", cleaned_subject);

            // filter body on YES or NO

            let body = payload.body();

            let mut lines = body.lines();

            // Process the first line to remove all whitespace
            let first_line = lines
                .next()
                .unwrap_or("")
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();

            let vote = if first_line == "YES" {
                Vote::Yes
            } else if first_line == "NO" {
                Vote::No
            } else {
                Vote::Invalid
            };

            println!("Voted: {}", vote);
        }

        // TODO: Set as read
        // TODO: Respond to mail

        println!();
        println!();
    }

    Ok(())
}

fn extract_recipient(message: &Message) -> anyhow::Result<String> {
    if let Some(payload) = &message.payload {
        if let Some(headers) = &payload.headers {
            for header in headers.to_owned() {
                if header.name.unwrap_or_default().eq_ignore_ascii_case("From") {
                    return Ok(header.value.unwrap_or_default().clone());
                }
            }
        }
    }
    Err(anyhow::Error::msg(
        "Failed to extract recipient from the message",
    ))
}

fn create_reply_raw(
    to: &str,
    subject: &str,
    body: &str,
    message_id: &str,
) -> anyhow::Result<String> {
    let raw_message = format!(
        "To: {}\r\nSubject: {}\r\nIn-Reply-To: {}\r\nReferences: {}\r\n\r\n{}",
        to, subject, message_id, message_id, body
    );

    Ok(general_purpose::URL_SAFE_NO_PAD.encode(raw_message))
}

#[derive(Debug)]
enum Vote {
    Yes,
    No,
    Invalid,
}

impl std::fmt::Display for Vote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vote::Yes => write!(f, "YES"),
            Vote::No => write!(f, "NO"),
            Vote::Invalid => write!(f, "INVALID"),
        }
    }
}

trait PayloadBody {
    fn body(&self) -> String;
}

impl PayloadBody for MessagePart {
    fn body(&self) -> String {
        self.parts
            .as_ref()
            .unwrap()
            .iter()
            .map(|part| {
                String::from_utf8_lossy(part.body.as_ref().unwrap().data.as_ref().unwrap())
                    .to_string()
            })
            .next()
            .unwrap()
    }
}

trait PayloadSubject {
    fn subject(&self) -> String;
}

impl PayloadSubject for MessagePart {
    fn subject(&self) -> String {
        self.headers
            .as_ref()
            .unwrap()
            .iter()
            .filter(|h| h.name == Some("Subject".to_string()))
            .next()
            .unwrap()
            .value
            .clone()
            .unwrap_or_default()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    general_purpose::STANDARD
                            .decode("eyJzdGF0dXMiOiI0MDAiLCJzY2hlbWVzIjoiQmVhcmVyIiwic2NvcGUiOiJodHRwczovL21haWwuZ29vZ2xlLmNvbS8ifQ==").unwrap();

    if let Err(ref err) = email_client().await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}

pub mod gpt {
    use chatgpt::{prelude::*, types::Role};
    use std::{fs, time::Duration};

    use crate::Payment;

    pub async fn process_body(body: String) -> anyhow::Result<Option<Payment>> {
        let mut convo = prompt_setup()?;

        // process the body
        let resp = convo.send_message(body).await?;

        let output = resp.message().content.clone();
        eprintln!("Output: {:?}", output);

        let payment: Option<Payment> = serde_json::from_str(&output).ok();

        Ok(payment)
    }

    fn prompt_setup() -> anyhow::Result<Conversation> {
        let config = ModelConfigurationBuilder::default()
            .engine(ChatGPTEngine::Custom("gpt-4o-mini"))
            .temperature(0.0)
            .top_p(0.1)
            .max_tokens(4096u32)
            .timeout(Duration::from_secs(300))
            .build()?;

        let api_key = std::env::var("API_KEY").unwrap();

        let client = ChatGPT::new_with_config(api_key, config)?;

        let system_prompt = fs::read_to_string("prompt.md")?;

        let convo = Conversation::new_with_history(
            client,
            vec![ChatMessage {
                role: Role::System,
                content: system_prompt.to_string(),
            }],
        );
        Ok(convo)
    }
}
