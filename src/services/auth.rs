use std::{fmt, sync::Arc};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use auto_di::singleton;
use sqlx::{Sqlite, SqlitePool, Transaction};

use crate::{
    api::dto::auth::{AuthResponseDto, LoginDto, SignupDto},
    db::models::users::User,
    utils::jwt::{
        claim::{Claims, JwtSubject},
        error::TokenError,
        service::{JwtService, TokenPair},
    },
};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    InvalidToken,
    Database(sqlx::Error),
    Internal,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "invalid email or password"),
            Self::InvalidToken => write!(f, "invalid or revoked token"),
            Self::Database(error) => write!(f, "{error}"),
            Self::Internal => write!(f, "authentication operation failed"),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<sqlx::Error> for AuthError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}

impl From<TokenError> for AuthError {
    fn from(_: TokenError) -> Self {
        Self::InvalidToken
    }
}

pub struct AuthService {
    db: Arc<SqlitePool>,
    jwt: Arc<JwtService>,
}

#[singleton]
impl AuthService {
    fn new(db: Arc<SqlitePool>, jwt: Arc<JwtService>) -> Self {
        Self { db, jwt }
    }

    pub async fn signup(&self, input: SignupDto) -> Result<AuthResponseDto, AuthError> {
        let password = hash_password(input.password).await?;
        let mut tx = self.db.begin().await?;

        sqlx::query!("INSERT OR IGNORE INTO groups (name) VALUES ('owner')")
            .execute(&mut *tx)
            .await?;
        let group_id = sqlx::query_scalar!("SELECT id FROM groups WHERE name = 'owner'")
            .fetch_one(&mut *tx)
            .await?;

        let avatar = input.avatar.unwrap_or_default();
        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO users (
                    email, first_name, last_name, avatar, role,
                    password, is_registered, group_id, added_by
               ) VALUES (?, ?, ?, ?, 'OWNER', ?, 1, ?, NULL)
               RETURNING id AS "id?", email, last_name, first_name, avatar, role,
                         about_me, password, is_email_verify, email_verify_at,
                         two_factor_enable, is_registered, added_by, group_id,
                         created_at, updated_at"#,
            input.email,
            input.first_name,
            input.last_name,
            avatar,
            password,
            group_id
        )
        .fetch_one(&mut *tx)
        .await?;

        let subject = subject_from_user(&user)?;
        let tokens = self.jwt.generate_token_pair(&subject)?;
        self.store_token_pair(&mut tx, &tokens).await?;
        tx.commit().await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn login(&self, input: LoginDto) -> Result<AuthResponseDto, AuthError> {
        let user = self
            .get_user_by_email(&input.email)
            .await
            .map_err(|error| {
                if matches!(error, sqlx::Error::RowNotFound) {
                    AuthError::InvalidCredentials
                } else {
                    AuthError::Database(error)
                }
            })?;

        verify_password(input.password, user.password.clone()).await?;
        let subject = subject_from_user(&user)?;
        let tokens = self.issue_token_pair(&subject).await?;
        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<AuthResponseDto, AuthError> {
        let old_claims = self.jwt.validate_refresh_token(refresh_token)?;
        self.ensure_token_active(&old_claims.jti).await?;

        let user = self.get_user_by_id(old_claims.user.user_id).await?;
        let subject = subject_from_user(&user)?;
        let tokens = self.jwt.generate_token_pair(&subject)?;
        let mut tx = self.db.begin().await?;
        sqlx::query!(
            "UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = strftime('%s', 'now') WHERE jti = ?",
            old_claims.jti
        )
        .execute(&mut *tx)
        .await?;
        self.store_token_pair(&mut tx, &tokens).await?;
        tx.commit().await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn validate_access_token(&self, token: &str) -> Result<Claims, AuthError> {
        let claims = self.jwt.validate_access_token(token)?;
        self.ensure_token_active(&claims.jti).await?;
        Ok(claims)
    }

    pub async fn logout_all(&self, user_id: i64) -> Result<(), AuthError> {
        sqlx::query!(
            "UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = strftime('%s', 'now') WHERE user_id = ? AND is_blacklist = 0",
            user_id
        )
        .execute(self.db.as_ref())
        .await?;
        Ok(())
    }

    async fn issue_token_pair(&self, subject: &JwtSubject) -> Result<TokenPair, AuthError> {
        let tokens = self.jwt.generate_token_pair(subject)?;
        let mut tx = self.db.begin().await?;
        self.store_token_pair(&mut tx, &tokens).await?;
        tx.commit().await?;
        Ok(tokens)
    }

    async fn store_token_pair(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        tokens: &TokenPair,
    ) -> Result<(), AuthError> {
        let access = self.jwt.validate_access_token(&tokens.access_token)?;
        let refresh = self.jwt.validate_refresh_token(&tokens.refresh_token)?;
        for claims in [access, refresh] {
            let role = claims.user.role.as_deref().unwrap_or("MEMBER");
            let expired_at = claims.exp as i64;
            sqlx::query!(
                "INSERT INTO jwt_tokens (jti, role, user_id, expired_at) VALUES (?, ?, ?, ?)",
                claims.jti,
                role,
                claims.user.user_id,
                expired_at
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }

    async fn ensure_token_active(&self, jti: &str) -> Result<(), AuthError> {
        let active = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM jwt_tokens WHERE jti = ? AND is_blacklist = 0 AND expired_at > strftime('%s', 'now'))",
            jti
        )
        .fetch_one(self.db.as_ref())
        .await?;
        if active != 1 {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"SELECT id AS "id?", email, last_name, first_name, avatar, role,
                      about_me, password, is_email_verify, email_verify_at,
                      two_factor_enable, is_registered, added_by, group_id,
                      created_at, updated_at
               FROM users WHERE email = ?"#,
            email
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    async fn get_user_by_id(&self, id: i64) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"SELECT id AS "id?", email, last_name, first_name, avatar, role,
                      about_me, password, is_email_verify, email_verify_at,
                      two_factor_enable, is_registered, added_by, group_id,
                      created_at, updated_at
               FROM users WHERE id = ?"#,
            id
        )
        .fetch_one(self.db.as_ref())
        .await
    }
}

fn subject_from_user(user: &User) -> Result<JwtSubject, AuthError> {
    Ok(JwtSubject {
        user_id: user.id.ok_or(AuthError::Internal)?,
        email: user.email.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        avatar: user.avatar.clone(),
        role: user.role.clone(),
        group_id: user.group_id,
    })
}

async fn hash_password(password: String) -> Result<String, AuthError> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(|_| AuthError::Internal)
    })
    .await
    .map_err(|_| AuthError::Internal)?
}

async fn verify_password(password: String, encoded: String) -> Result<(), AuthError> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&encoded).map_err(|_| AuthError::InvalidCredentials)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| AuthError::InvalidCredentials)
    })
    .await
    .map_err(|_| AuthError::Internal)?
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;
    use crate::utils::jwt::config::JwtConfig;

    async fn service() -> AuthService {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE groups (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, email TEXT UNIQUE, last_name TEXT, first_name TEXT, avatar TEXT NOT NULL, role TEXT DEFAULT 'OWNER', about_me TEXT, password TEXT NOT NULL, is_email_verify INTEGER DEFAULT 0, email_verify_at INTEGER, two_factor_enable INTEGER DEFAULT 0, is_registered INTEGER NOT NULL DEFAULT 0, added_by INTEGER REFERENCES users(id), group_id INTEGER NOT NULL REFERENCES groups(id), created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE jwt_tokens (id INTEGER PRIMARY KEY AUTOINCREMENT, jti TEXT NOT NULL, role TEXT NOT NULL, user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, is_blacklist INTEGER DEFAULT 0, blacklist_at INTEGER, expired_at INTEGER, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        AuthService {
            db: Arc::new(pool),
            jwt: Arc::new(JwtService::new(Arc::new(JwtConfig::default()))),
        }
    }

    #[tokio::test]
    async fn signup_login_refresh_and_logout_flow() {
        let service = service().await;
        let signup = service
            .signup(SignupDto {
                email: "owner@example.com".into(),
                password: "strong-password".into(),
                first_name: Some("Owner".into()),
                last_name: None,
                avatar: None,
            })
            .await
            .unwrap();

        let stored_password: String = sqlx::query_scalar("SELECT password FROM users WHERE id = ?")
            .bind(signup.user.user_id)
            .fetch_one(service.db.as_ref())
            .await
            .unwrap();
        assert!(stored_password.starts_with("$argon2"));
        assert_ne!(stored_password, "strong-password");

        let login = service
            .login(LoginDto {
                email: "owner@example.com".into(),
                password: "strong-password".into(),
            })
            .await
            .unwrap();
        service
            .validate_access_token(&login.tokens.access_token)
            .await
            .unwrap();

        let refreshed = service.refresh(&login.tokens.refresh_token).await.unwrap();
        assert!(service.refresh(&login.tokens.refresh_token).await.is_err());
        service.logout_all(refreshed.user.user_id).await.unwrap();
        assert!(
            service
                .validate_access_token(&refreshed.tokens.access_token)
                .await
                .is_err()
        );
    }
}
