use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Claim{
    pub uuid: i32,
    pub expiration: usize
}

pub fn create_jwt(uuid: i32, secret_key: &[u8]) -> String{
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Valid timestamp")
        .timestamp();
    
    let claim = Claim {
        uuid,
        expiration: expiration as usize
    };

    let header = Header::new(Algorithm::HS256);
    encode(&header, &claim, &EncodingKey::from_secret(secret_key))
        .unwrap()
}

