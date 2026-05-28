use core::{error::Error, fmt::Debug, fmt::Display};

// Internal error for const fn
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Err {
    TooShort(usize),
    TooLong(usize),
    NotAscii,
}

impl Display for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort(min) => {
                write!(f, "minimum length allowed is {}", min)
            }
            Self::TooLong(max) => {
                write!(f, "maximum length allowed is {}", max)
            }
            Self::NotAscii => {
                write!(f, "only ASCII characters are allowed")
            }
        }
    }
}

impl Error for Err {}

impl<VE> From<Err> for GStringError<VE> {
    fn from(value: Err) -> Self {
        match value {
            Err::TooShort(min) => Self::TooShort(min),
            Err::TooLong(max) => Self::TooLong(max),
            Err::NotAscii => Self::NotAscii,
        }
    }
}

#[cfg(test)]
mod err_test {
    use super::Err;

    #[test]
    fn test_err() {
        let fmt_err = |err: Err| -> String { err.to_string() };

        assert_eq!(fmt_err(Err::TooShort(0)), "minimum length allowed is 0");
        assert_eq!(fmt_err(Err::TooLong(100)), "maximum length allowed is 100");
        assert_eq!(fmt_err(Err::NotAscii), "only ASCII characters are allowed");
    }
}

/// GString error type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GStringError<VE> {
    /// When string is shorter than MIN=usize.
    TooShort(usize),
    /// When string is longer than MAX=usize.
    TooLong(usize),
    /// When ASCII_ONLY is true but string is not all ASCII.
    NotAscii,
    /// When validation failed with VE = Validator associated error type.
    Validation(VE),
    /// When mutation failed with error message.
    Mutation(&'static str),
}

impl<VE: Display + Debug> Display for GStringError<VE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort(min) => {
                write!(f, "minimum length allowed is {}", min)
            }
            Self::TooLong(max) => {
                write!(f, "maximum length allowed is {}", max)
            }
            Self::NotAscii => {
                write!(f, "only ASCII characters are allowed")
            }
            Self::Validation(err) => write!(f, "validation error: {}", err),
            Self::Mutation(err) => write!(f, "mutation error: {}", err),
        }
    }
}

impl<VE: Display + Debug> Error for GStringError<VE> {}
