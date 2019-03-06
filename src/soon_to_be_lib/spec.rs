use failure::ResultExt;
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum InternetKind {
    SafeEmail,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum NameKind {
    Name,
    FirstName,
    LastName,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AddressKind {
    Zip,
    StreetName,
    City,
    StateAbbr,
}

#[derive(Debug)]
pub enum FakerKind {
    Internet(InternetKind),
    Name(NameKind),
    Address(AddressKind),
}

impl FakerKind {
    pub fn fake(&self) -> Cow<str> {
        use self::FakerKind::*;
        use fake::faker::*;
        match self {
            Name(minor) => {
                use self::NameKind::*;
                match minor {
                    Name => <Faker as fake::faker::Name>::name().into(),
                    LastName => Faker::last_name().into(),
                    FirstName => Faker::first_name().into(),
                }
            }
            Internet(minor) => {
                use self::InternetKind::*;
                match minor {
                    SafeEmail => Faker::safe_email().into(),
                }
            }
            Address(minor) => {
                use self::AddressKind::*;
                match minor {
                    Zip => Faker::zip().into(),
                    StreetName => Faker::street_name().into(),
                    City => Faker::city().into(),
                    StateAbbr => Faker::state_abbr().into(),
                }
            }
        }
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
                "name" => Name(minor.parse()?),
                "address" => Address(minor.parse()?),
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
