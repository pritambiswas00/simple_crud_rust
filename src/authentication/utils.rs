use jsonwebtoken::{ encode, errors::Error, DecodingKey, EncodingKey, Header  };

use crate::{ authentication::models::Token, user::model::User };

pub fn encode_token (
    user:&User, 
    expire:usize,
    token_type:&str,
    secret:&str
)->Result<String, Error>{
    let tokens = Token::from_user(user, token_type, expire);
    let encoded_token = encode(
        &Header::default(),
         &tokens, 
         &EncodingKey::from_secret(secret.as_ref()));

    encoded_token
}

pub fn validate_token(
    token:&str, secret_key:&str
)->Result<Token, Error>{
    let token = jsonwebtoken::decode::<Token>(token, &DecodingKey::from_secret(secret_key.as_ref()),
    &jsonwebtoken::Validation::default()
    )?;

    Ok(token.claims)
}