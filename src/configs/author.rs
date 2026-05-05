use miette::{Diagnostic, NamedSource, SourceSpan};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

impl Author {
    pub fn parse(input: &str) -> Result<Self, AuthorParseError> {
        let trimmed = input.trim();

        let make_src = || NamedSource::new("author", input.to_string());
        let full_span = SourceSpan::from((0, input.len()));

        if trimmed.is_empty() {
            return Err(AuthorParseError::Empty {
                src: make_src(),
                span: full_span,
            });
        }

        let Some(open) = trimmed.rfind('<') else {
            return Ok(Author {
                name: trimmed.to_string(),
                email: None,
            });
        };

        if !trimmed.ends_with('>') {
            // offset of '<' relative to original input
            let offset = input.find('<').unwrap_or(0);
            return Err(AuthorParseError::MalformedEmail {
                src: make_src(),
                span: SourceSpan::from((offset, input.len() - offset)),
            });
        }

        let name = trimmed[..open].trim().to_string();
        let email = trimmed[open + 1..trimmed.len() - 1].trim().to_string();

        if name.is_empty() {
            let offset = input.find('<').unwrap_or(0);
            return Err(AuthorParseError::MissingName {
                src: make_src(),
                span: SourceSpan::from((0, offset.max(1))),
            });
        }

        if email.is_empty() {
            let offset = input.find('<').unwrap_or(0);
            return Err(AuthorParseError::MalformedEmail {
                src: make_src(),
                span: SourceSpan::from((offset, input.len() - offset)),
            });
        }

        Ok(Author {
            name,
            email: Some(email),
        })
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum AuthorParseError {
    #[error("Author string is empty")]
    #[diagnostic(
        code(config::author::empty),
        help("Provide an author as `Name` or `Name <email>`.")
    )]
    Empty {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Author has email brackets but no name")]
    #[diagnostic(
        code(config::author::missing_name),
        help("Add a name before the `<...>`, e.g. `Jane Doe <jane@example.com>`.")
    )]
    MissingName {
        #[source_code]
        src: NamedSource<String>,
        #[label("missing name before this")]
        span: SourceSpan,
    },

    #[error("Malformed author — expected `Name <email>`")]
    #[diagnostic(
        code(config::author::malformed_email),
        help("Make sure the email is wrapped in `<...>` and not empty.")
    )]
    MalformedEmail {
        #[source_code]
        src: NamedSource<String>,
        #[label("malformed here")]
        span: SourceSpan,
    },
}

impl std::str::FromStr for Author {
    type Err = AuthorParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl<'de> Deserialize<'de> for Author {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Author::parse(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Author {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.email {
            Some(e) => write!(f, "{} <{}>", self.name, e),
            None => write!(f, "{}", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_email() {
        let a: Author = "Max <max@example.com>".parse().unwrap();
        assert_eq!(a.name, "Max");
        assert_eq!(a.email.as_deref(), Some("max@example.com"));
    }

    #[test]
    fn parse_without_email() {
        let a: Author = "Jane Doe".parse().unwrap();
        assert_eq!(a.name, "Jane Doe");
        assert_eq!(a.email, None);
    }

    #[test]
    fn display_roundtrip() {
        let original = "Max Mustermann <max@example.com>";
        let parsed: Author = original.parse().unwrap();
        assert_eq!(parsed.to_string(), original);
    }
}
