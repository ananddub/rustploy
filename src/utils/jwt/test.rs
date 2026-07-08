

#[cfg(test)]
mod tests {
    use crate::utils::jwt::config::JwtConfig;
    use crate::utils::jwt::service::JwtService;
    use super::*;

    #[test]
    fn full_flow() {
        let service = JwtService::new(std::sync::Arc::new(JwtConfig::default()));

        let pair = service.generate_token_pair("user_123").unwrap();

        let access_claims = service.validate_access_token(&pair.access_token).unwrap();
        assert_eq!(access_claims.sub, "user_123");

        let refresh_claims = service.validate_refresh_token(&pair.refresh_token).unwrap();
        assert_eq!(refresh_claims.sub, "user_123");

        let wrong = service.validate_refresh_token(&pair.access_token);
        assert!(wrong.is_err());

        let new_access = service.refresh_access_token(&pair.refresh_token).unwrap();
        let new_claims = service.validate_access_token(&new_access).unwrap();
        assert_eq!(new_claims.sub, "user_123");
    }
}