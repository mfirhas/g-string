use core::{error::Error, fmt::Debug, fmt::Display};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Err {
    TooShort,
    TooLong,
    NotAscii,
}

impl Display for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort => {
                write!(f, "string len is smaller than MIN")
            }
            Self::TooLong => {
                write!(f, "string len is bigger than MAX")
            }
            Self::NotAscii => {
                write!(f, "ASCII_ONLY is true, but not ascii")
            }
        }
    }
}

impl Error for Err {}

impl<VE> From<Err> for GStringError<VE> {
    fn from(value: Err) -> Self {
        match value {
            Err::TooShort => Self::TooShort,
            Err::TooLong => Self::TooLong,
            Err::NotAscii => Self::NotAscii,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GStringError<VE> {
    TooShort,
    TooLong,
    NotAscii,
    Validation(VE),
    Mutation(&'static str),
}

impl<VE: Display + Debug> Display for GStringError<VE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort => {
                write!(f, "string len is smaller than MIN")
            }
            Self::TooLong => {
                write!(f, "string len is bigger than MAX")
            }
            Self::NotAscii => {
                write!(f, "ASCII_ONLY is true, but not ascii")
            }
            Self::Validation(err) => write!(f, "validation error: {}", err),
            Self::Mutation(err) => write!(f, "mutation error: {}", err),
        }
    }
}

impl<VE: Display + Debug> Error for GStringError<VE> {}
