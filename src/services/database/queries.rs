use uuid::Uuid;

pub(super) fn generate_app_name(name: &str, prefix: &str) -> String {
    let slug = slug_value(name);
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}-{}", prefix, slug, &suffix[..6])
}

pub(super) fn slug_value(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in name.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "database".into()
    } else {
        slug.into()
    }
}

pub(super) fn random_secret() -> String {
    Uuid::new_v4().simple().to_string()
}
