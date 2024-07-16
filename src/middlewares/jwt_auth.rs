use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::models::jwt::{Claims, UserEmail};


pub fn get_jwt(user: UserEmail) -> Result<String, String> {
  let token = encode(
    &Header::default(),
    &Claims {
      email: user.email,
      exp: (Utc::now() + Duration::minutes(1)).timestamp(),
    },
    &EncodingKey::from_secret("mykey".as_bytes()),
  )
  .map_err(|e| e.to_string());

  return token;
}

pub fn decode_jwt(token: &str) -> Result<UserEmail, String> {
    let token_data = decode(
        token,
        &DecodingKey::from_secret("mykey".as_bytes()),
        &Validation::default()
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(e.to_string()),
    }
}