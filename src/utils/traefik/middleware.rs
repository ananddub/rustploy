#[derive(Debug, Clone)]
pub enum Middleware {
    StripPrefix {
        name: String,
        prefixes: Vec<String>,
    },

    AddPrefix {
        name: String,
        prefix: String,
    },

    RedirectScheme {
        name: String,
        scheme: String,
        permanent: bool,
    },

    Compress {
        name: String,
    },

    Headers {
        name: String,
        headers: Vec<(String, String)>,
    },

    Custom {
        name: String,
    },
}

impl Middleware {
    pub fn labels(&self) -> Vec<(String, String)> {
        match self {
            Middleware::StripPrefix { name, prefixes } => vec![(
                format!("traefik.http.middlewares.{name}.stripprefix.prefixes"),
                prefixes.join(","),
            )],

            Middleware::AddPrefix { name, prefix } => vec![(
                format!("traefik.http.middlewares.{name}.addprefix.prefix"),
                prefix.clone(),
            )],

            Middleware::RedirectScheme {
                name,
                scheme,
                permanent,
            } => vec![
                (
                    format!("traefik.http.middlewares.{name}.redirectscheme.scheme"),
                    scheme.clone(),
                ),
                (
                    format!("traefik.http.middlewares.{name}.redirectscheme.permanent"),
                    permanent.to_string(),
                ),
            ],

            Middleware::Compress { name } => vec![(
                format!("traefik.http.middlewares.{name}.compress"),
                "true".into(),
            )],

            Middleware::Headers { name, headers } => headers
                .iter()
                .map(|(k, v)| {
                    (
                        format!(
                            "traefik.http.middlewares.{name}.headers.customResponseHeaders.{k}"
                        ),
                        v.clone(),
                    )
                })
                .collect(),

            Middleware::Custom { .. } => vec![],
        }
    }

    pub fn reference(&self) -> String {
        match self {
            Middleware::StripPrefix { name, .. } => name.clone(),
            Middleware::AddPrefix { name, .. } => name.clone(),
            Middleware::RedirectScheme { name, .. } => name.clone(),
            Middleware::Compress { name } => name.clone(),
            Middleware::Headers { name, .. } => name.clone(),
            Middleware::Custom { name } => name.clone(),
        }
    }
}
