use failure::ResultExt;
use std::str::FromStr;

#[derive(Debug)]
pub enum InternetKind {
    SafeEmail,
}

#[derive(Debug)]
pub enum FakerKind {
    Internet(InternetKind),
}

impl FromStr for InternetKind {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::InternetKind::*;
        Ok(match s.to_lowercase().as_str() {
            "safe_email" | "safeemail" => SafeEmail,
            _ => bail!("Unkown minor faker kind: '{}'", s),
        })
    }
}

impl FromStr for FakerKind {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::FakerKind::*;
        let mut tokens = s.split('.');
        Ok(match (tokens.next(), tokens.next()) {
            (Some(major), Some(minor)) => match major.to_lowercase().as_str() {
                "internet" => Internet(minor.parse()?),
                _ => bail!("Unknown major faker kind: '{}'", major),
            },
            _ => bail!("Invalid faker kind: '{}'", s),
        })
    }
}

#[derive(Debug)]
pub struct Spec {
    pub column: usize,
    pub kind: FakerKind,
}

impl FromStr for Spec {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(':');
        Ok(match (tokens.next(), tokens.next()) {
            (Some(column), Some(kind)) => Spec {
                column: column.parse::<usize>().with_context(|_| {
                    format!("Could not parse '{}' as unsigned integer", column)
                })?,
                kind: kind.parse()?,
            },
            _ => bail!(
                "Invalid format for rewrite spec. It should be <column>:<type>, but was '{}'",
                s
            ),
        })
    }
}
