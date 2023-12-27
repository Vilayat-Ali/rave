use jsonwebtoken::{decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct JWT;

impl JWT {
    // generate token from a payload struct
    pub fn generate_token<'a, T>(token_payload: &T) -> Result<String>
    where
        T: Serialize + Deserialize<'a>,
    {
        let token = encode(
            &Header::default(),
            token_payload,
            &EncodingKey::from_secret("secret".as_bytes()),
        )?;
        Ok(token)
    }

    // decode / verify the token
    pub fn verify_token<'a, T>(token: &'a str) -> Result<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let token_data: jsonwebtoken::TokenData<T> = decode(
            token,
            &DecodingKey::from_secret("secret".as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims as T)
    }
}
