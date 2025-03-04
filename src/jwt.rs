use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

use crate::Result;

pub fn create_token(subject: &str, secret: &str) -> Result<String> {
    let exp = SystemTime::now() + Duration::new(3600, 0);
    let Ok(elapsed) = exp.duration_since(SystemTime::UNIX_EPOCH) else {
        return Err("Error creating JWT token".into());
    };

    let claims = Claims {
        sub: subject.to_string(),
        exp: elapsed.as_secs() as usize,
    };

    let Ok(token) = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) else {
        return Err("Error creating JWT token".into());
    };

    Ok(token)
}
