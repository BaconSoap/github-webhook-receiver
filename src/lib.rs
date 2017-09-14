mod from_hex;

extern crate hmac;
extern crate sha_1;

use hmac::{Hmac, Mac};
use sha_1::Sha1;
use from_hex::FromHex;

#[derive(Debug)]
pub enum WebhookEventType {
    PullRequest(PullRequestEventType),
    Push,
}

#[derive(Debug)]
pub enum PullRequestEventType {
    Opened,
    Closed,
    Merged,
    Reopened,
}

#[derive(Debug)]
pub struct WebhookRequest {
    event: WebhookEventType,
    github_signature: String,
    payload: String,
}

pub fn hi() {
    println!("hi");
    let a = WebhookRequest {
        event: WebhookEventType::PullRequest(PullRequestEventType::Opened),
        github_signature: String::from(""),
        payload: String::from("hi"),
    };

    println!("{:?}", a);
}

pub fn is_signature_valid(secret: String, signature: String, payload: String) -> bool {
    let mut hmac = Hmac::<Sha1>::new(secret.as_bytes());
    let bytes = payload.as_bytes();
    hmac.input(bytes);

    let signature = signature.replace("sha1=", "");

    let signature_bytes = signature.from_hex().unwrap();

    hmac.verify(&signature_bytes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_signature_valid_works() {
        let secret = "test";
        let sig = "sha1=a3ad816405dec2204d9a1c4159ae51c8b030f463";
        let json = "{\"test\": true}";

        assert!(is_signature_valid(
            String::from(secret),
            String::from(sig),
            String::from(json)
        ));
    }
}
