use auto_di::singleton;
use std::sync::Arc;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use chrono::{Utc, Duration};
use serde::Serialize;
use crate::utils::jwt::claim::{Claims, TokenType};
use super::config::JwtConfig;
use super::error::TokenError;

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct JwtService {
    config: Arc<JwtConfig>,
}
#[singleton]
impl JwtService {

    pub fn new(config: Arc<JwtConfig>) -> Self {
        Self { config }
    }

    pub fn generate_token_pair(&self, user_id: &str) -> Result<TokenPair, TokenError> {
        let access_token = self.generate_token(user_id, TokenType::Access)?;
        let refresh_token = self.generate_token(user_id, TokenType::Refresh)?;
        Ok(TokenPair { access_token, refresh_token })
    }

    fn generate_token(&self, user_id: &str, token_type: TokenType) -> Result<String, TokenError> {
        let now = Utc::now();
        let (secret, expiry) = match token_type {
            TokenType::Access => (
                &self.config.access_secret,
                now + Duration::minutes(self.config.access_expiry_mins),
            ),
            TokenType::Refresh => (
                &self.config.refresh_secret,
                now + Duration::days(self.config.refresh_expiry_days),
            ),
        };

        let claims = Claims {
            sub: user_id.to_string(),
            token_type,
            iat: now.timestamp() as usize,
            exp: expiry.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
            .map_err(|_| TokenError::EncodingFailed)
    }

    pub fn validate_access_token(&self, token: &str) -> Result<Claims, TokenError> {
        self.validate(token, &self.config.access_secret, TokenType::Access)
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, TokenError> {
        self.validate(token, &self.config.refresh_secret, TokenType::Refresh)
    }

    fn validate(&self, token: &str, secret: &str, expected: TokenType) -> Result<Claims, TokenError> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => TokenError::Expired,
                _ => TokenError::Invalid,
            })?;

        if token_data.claims.token_type != expected {
            return Err(TokenError::WrongType);
        }

        Ok(token_data.claims)
    }

    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String, TokenError> {
        let claims = self.validate_refresh_token(refresh_token)?;
        self.generate_token(&claims.sub, TokenType::Access)
    }
}