use failure::ResultExt;
use std::{borrow::Cow, str::FromStr};
use strum::IntoEnumIterator;

#[derive(Debug, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum InternetKind {
    FreeEmailProvider,
    DomainSuffix,
    UserName,
    FreeEmail,
    SafeEmail,
    Password,
    Ip,
    Ipv4,
    Ipv6,
    Color,
    UserAgent,
}

#[derive(Debug, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum NameKind {
    FirstName,
    LastName,
    Prefix,
    Suffix,
    Name,
    NameWithMiddle,
    TitleDescriptor,
    TitleLevel,
    TitleJob,
    Title,
}

#[derive(Debug, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AddressKind {
    TimeZone,
    CityPrefix,
    CitySuffix,
    StreetSuffix,
    State,
    StateAbbr,
    City,
    StreetName,
    BuildingNumber,
    StreetAddress,
    SecondaryAddress,
    Zip,
    Postcode,
    Latitude,
    Longitude,
}

#[derive(Debug, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum CompanyKind {
    Suffix,
    Name,
    Buzzword,
    CatchPhrase,
    Bs,
    Profession,
    Industry,
}

#[derive(Debug, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum LoremKind {
    Word,
    Words,
    Sentence,
    Sentences,
    Paragraph,
    Paragraphs,
}

#[derive(Debug, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum NumberKind {
    PhoneNumber,
    CellNumber,
    Digit,
}

#[derive(Debug)]
pub enum FakerKind {
    Address(AddressKind),
    Boolean,
    Company(CompanyKind),
    Lorem(LoremKind),
    Number(NumberKind),
    Internet(InternetKind),
    Name(NameKind),
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
                "boolean" => Boolean,
                "company" => Company(minor.parse()?),
                "lorem" => Lorem(minor.parse()?),
                "number" => Number(minor.parse()?),
                _ => bail!("Unknown major faker kind: '{}'", major),
            },
            _ => bail!("Invalid faker kind: '{}'", s),
        })
    }
}

impl AsRef<str> for FakerKind {
    fn as_ref(&self) -> &'static str {
        use self::FakerKind::*;
        match self {
            Boolean => "Boolean.simple",
            Address(_) => "Address",
            Company(_) => "Company",
            Internet(_) => "Internet",
            Name(_) => "Name",
            Lorem(_) => "Lorem",
            Number(_) => "Number",
        }
    }
}

impl FakerKind {
    pub fn eprint_combinations() {
        fn eprint_major_minor<E, I>(major: &str)
        where
            E: IntoEnumIterator<Iterator = I> + Into<&'static str>,
            I: Iterator<Item = E>,
        {
            for item in E::iter() {
                eprintln!("{}.{}", major, item.into())
            }
            eprintln!();
        }
        eprint_major_minor::<NameKind, _>(FakerKind::Name(NameKind::Name).as_ref());
        eprint_major_minor::<AddressKind, _>(FakerKind::Address(AddressKind::City).as_ref());
        eprint_major_minor::<CompanyKind, _>(FakerKind::Company(CompanyKind::Name).as_ref());
        eprint_major_minor::<LoremKind, _>(FakerKind::Lorem(LoremKind::Word).as_ref());
        eprint_major_minor::<NumberKind, _>(FakerKind::Number(NumberKind::Digit).as_ref());
        eprint_major_minor::<InternetKind, _>(
            FakerKind::Internet(InternetKind::SafeEmail).as_ref(),
        );

        eprintln!("{}", FakerKind::Boolean.as_ref())
    }
    pub fn fake(&self) -> Cow<str> {
        use self::FakerKind::*;
        use fake::faker::*;
        match self {
            Number(minor) => {
                use self::NumberKind::*;
                match minor {
                    Digit => Faker::digit().into(),
                    CellNumber => Faker::cell_number().into(),
                    PhoneNumber => Faker::phone_number().into(),
                }
            }
            Lorem(minor) => {
                use self::LoremKind::*;
                match minor {
                    Word => Faker::word().into(),
                    Words => Faker::words(10).join(" ").into(),
                    Sentence => Faker::sentence(10, 15).into(),
                    Sentences => Faker::sentences(5).join("\n").into(),
                    Paragraph => Faker::paragraph(10, 15).into(),
                    Paragraphs => Faker::paragraphs(5).join("\n").into(),
                }
            }
            Company(minor) => {
                use self::CompanyKind::*;
                match minor {
                    Suffix => <Faker as fake::faker::Company>::suffix().into(),
                    Name => <Faker as fake::faker::Company>::name().into(),
                    Buzzword => Faker::buzzword().into(),
                    CatchPhrase => Faker::catch_phase().into(),
                    Bs => Faker::bs().into(),
                    Profession => Faker::profession().into(),
                    Industry => Faker::industry().into(),
                }
            }
            Boolean => if Faker::boolean() { "true" } else { "false" }.into(),
            Name(minor) => {
                use self::NameKind::*;
                match minor {
                    Name => <Faker as fake::faker::Name>::name().into(),
                    LastName => Faker::last_name().into(),
                    FirstName => Faker::first_name().into(),
                    NameWithMiddle => Faker::name_with_middle().into(),
                    Prefix => Faker::prefix().into(),
                    Suffix => <Faker as fake::faker::Name>::suffix().into(),
                    TitleDescriptor => Faker::title_descriptor().into(),
                    TitleLevel => Faker::title_level().into(),
                    TitleJob => Faker::title_job().into(),
                    Title => Faker::title().into(),
                }
            }
            Internet(minor) => {
                use self::InternetKind::*;
                match minor {
                    FreeEmailProvider => Faker::free_email_provider().into(),
                    DomainSuffix => Faker::domain_suffix().into(),
                    UserName => Faker::user_name().into(),
                    FreeEmail => Faker::free_email().into(),
                    SafeEmail => Faker::safe_email().into(),
                    Password => Faker::password(4, 6).into(),
                    Ip => Faker::ip().into(),
                    Ipv4 => Faker::ipv4().into(),
                    Ipv6 => Faker::ipv6().into(),
                    Color => Faker::color().into(),
                    UserAgent => Faker::user_agent().into(),
                }
            }
            Address(minor) => {
                use self::AddressKind::*;
                match minor {
                    TimeZone => Faker::time_zone().into(),
                    CityPrefix => Faker::city_prefix().into(),
                    CitySuffix => Faker::city_suffix().into(),
                    StreetSuffix => Faker::street_suffix().into(),
                    State => Faker::state().into(),
                    StateAbbr => Faker::state_abbr().into(),
                    City => Faker::city().into(),
                    StreetName => Faker::street_name().into(),
                    BuildingNumber => Faker::building_number().into(),
                    StreetAddress => Faker::street_address().into(),
                    SecondaryAddress => Faker::secondary_address().into(),
                    Zip => Faker::zip().into(),
                    Postcode => Faker::postcode().into(),
                    Latitude => Faker::latitude().into(),
                    Longitude => Faker::longitude().into(),
                }
            }
        }
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
