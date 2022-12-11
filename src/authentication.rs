use std::env;
use worker::*;

pub async fn is_authenticated(req: &Request) -> Result<bool> {
    let header = req.headers().get("Authorization")?.unwrap_or_default();

    if header.is_empty() {
        return Ok(false);
    }

    let header_details = header.split(' ').collect::<Vec<_>>();

    if header_details[0] != "Basic" {
        return Ok(false);
    }

    let credentials = header_details[1];
    let verified = verify_basic(credentials);

    Ok(verified)
}

fn verify_basic(credentials: &str) -> bool {
    let decoded = base64::decode(credentials).unwrap();
    let stringified = String::from_utf8(decoded).unwrap();
    // let splitted = stringified
    //     .split(':')
    //     .map(|v| v.to_string())
    //     .collect::<Vec<_>>();

    let portal_auth = match env::var("PORTAL_AUTH") {
        Err(_) => panic!("PORTAL_AUTH not specified"),
        Ok(value) => value,
    };

    stringified == portal_auth
}
