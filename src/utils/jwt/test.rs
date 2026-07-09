#[cfg(test)]
mod tests {
    use crate::utils::jwt::claim::JwtSubject;
    use crate::utils::jwt::config::JwtConfig;
    use crate::utils::jwt::service::JwtService;

    #[test]
    fn full_flow() {
        let service = JwtService::new(std::sync::Arc::new(JwtConfig::default()));

        let subject = JwtSubject {
            user_id: 123,
            email: Some("user@example.com".into()),
            first_name: Some("Test".into()),
            last_name: Some("User".into()),
            avatar: "avatar.png".into(),
            role: Some("OWNER".into()),
            group_id: 1,
        };
        let pair = service.generate_token_pair(&subject).unwrap();

        let access_claims = service.validate_access_token(&pair.access_token).unwrap();
        assert_eq!(access_claims.sub, "123");
        assert_eq!(access_claims.user, subject);

        let refresh_claims = service.validate_refresh_token(&pair.refresh_token).unwrap();
        assert_eq!(refresh_claims.sub, "123");
        assert_eq!(refresh_claims.user, subject);

        let wrong = service.validate_refresh_token(&pair.access_token);
        assert!(wrong.is_err());

        let new_access = service.refresh_access_token(&pair.refresh_token).unwrap();
        let new_claims = service.validate_access_token(&new_access).unwrap();
        assert_eq!(new_claims.sub, "123");
        assert_eq!(new_claims.user, subject);
    }
}
