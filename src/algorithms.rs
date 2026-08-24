use clap::{ValueEnum, builder::PossibleValue};

#[derive(Debug, Copy, Clone)]
pub enum Algorithms {
    Ntlm,
    Md5,
    Sha1,
    Sha256,
    Pbkdf2,
    Bcrypt,
    Scrypt,
    Argon2,
}

impl ValueEnum for Algorithms {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Ntlm,
            Self::Md5,
            Self::Sha1,
            Self::Sha256,
            Self::Pbkdf2,
            Self::Bcrypt,
            Self::Scrypt,
            Self::Argon2,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Ntlm => PossibleValue::new("ntlm"),
            Self::Md5 => PossibleValue::new("md5"),
            Self::Sha1 => PossibleValue::new("sha1"),
            Self::Sha256 => PossibleValue::new("sha256"),
            Self::Pbkdf2 => PossibleValue::new("pbkdf2"),
            Self::Bcrypt => PossibleValue::new("bcrypt"),
            Self::Scrypt => PossibleValue::new("scrypt"),
            Self::Argon2 => PossibleValue::new("argon2"),
        })
    }
}
