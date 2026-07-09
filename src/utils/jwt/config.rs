use auto_di::singleton;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_expiry_mins: i64,
    pub refresh_expiry_days: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            access_secret: "access-secret-change-me".into(),
            refresh_secret: "refresh-secret-change-me".into(),
            access_expiry_mins: 15,
            refresh_expiry_days: 7,
        }
    }
}

#[singleton]
impl JwtConfig {
    pub fn new() -> Self {
        Self {
            access_secret: std::env::var("JWT_ACCESS_SECRET")
                .unwrap_or_else(|_| Self::default().access_secret),
            refresh_secret: std::env::var("JWT_REFRESH_SECRET")
                .unwrap_or_else(|_| Self::default().refresh_secret),
            access_expiry_mins: std::env::var("JWT_ACCESS_EXPIRY_MINS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15),
            refresh_expiry_days: std::env::var("JWT_REFRESH_EXPIRY_DAYS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(7),
        }
    }
}
