use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Duration in seconds
const EXP_DURATION: i64 = 60 * 60; // 1 hour 

use crate::Result;

pub fn create_token(subject: &str, secret: &str) -> Result<String> {
    let exp = Utc::now() + Duration::seconds(EXP_DURATION);

    let claims = Claims {
        sub: subject.to_string(),
        exp: exp.timestamp() as usize,
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
