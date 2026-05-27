use crate::{Err, GString, GStringError, Validator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotValidatedGString<
    V: Validator,
    const MIN: usize,
    const MAX: usize,
    const ASCII_ONLY: bool,
>(GString<V, MIN, MAX, ASCII_ONLY>);

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    NotValidatedGString<V, MIN, MAX, ASCII_ONLY>
{
    #[inline(always)]
    pub fn validate(self) -> Result<GString<V, MIN, MAX, ASCII_ONLY>, GStringError<V::Err>> {
        V::validate(&self.0).map_err(GStringError::Validation)?;
        Ok(self.0)
    }
}

macro_rules! errpanic {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(Err::TooShort(_)) => {
                panic!("minimum length below MIN")
            }
            Err(Err::TooLong(_)) => {
                panic!("maximum length exceeds MAX")
            }
            Err(Err::NotAscii) => {
                panic!("only ASCII characters are allowed")
            }
        }
    };
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    #[doc(hidden)]
    pub const fn __new(s: &str) -> NotValidatedGString<V, MIN, MAX, ASCII_ONLY> {
        let ret = errpanic!(Self::stack_allocate(s));
        errpanic!(ret.check_bounds());
        errpanic!(ret.check_ascii());
        NotValidatedGString(ret)
    }
}

#[macro_export]
macro_rules! gstring {
    ($s:literal) => {
        gstring!(
            $s,
            $crate::NoValidation,
            { $crate::DEFAULT_MIN },
            { $crate::DEFAULT_MAX },
            { $crate::DEFAULT_ASCII_ONLY }
        )
    };
    ($s:literal, $validator:ty) => {
        gstring!(
            $s,
            $validator,
            { $crate::DEFAULT_MIN },
            { $crate::DEFAULT_MAX },
            { $crate::DEFAULT_ASCII_ONLY }
        )
    };
    ($s:literal, $validator:ty, $min:expr) => {
        gstring!($s, $validator, $min, { $crate::DEFAULT_MAX }, {
            $crate::DEFAULT_ASCII_ONLY
        })
    };
    ($s:literal, $validator:ty, $min:expr, $max:expr) => {
        gstring!($s, $validator, $min, $max, { $crate::DEFAULT_ASCII_ONLY })
    };
    ($s:literal, $validator:ty, $min:expr, $max:expr, $ascii_only:expr) => {{
        const RET: $crate::NotValidatedGString<$validator, $min, $max, $ascii_only> =
            $crate::GString::<$validator, $min, $max, $ascii_only>::__new($s);
        RET
    }};
}

#[macro_export]
macro_rules! gformat {
    // -----------------------------------------------------------------------------
    // DEFAULT GENERICS
    // -----------------------------------------------------------------------------
    ($fmt:expr $(, $args:expr)* ) => {
        gformat!(
            $fmt $(, $args)* ;
            $crate::NoValidation,
            { $crate::DEFAULT_MIN },
            { $crate::DEFAULT_MAX },
            { $crate::DEFAULT_ASCII_ONLY }
        )
    };

    // -----------------------------------------------------------------------------
    // VALIDATOR ONLY
    // -----------------------------------------------------------------------------
    ($fmt:expr $(, $args:expr)* ; $validator:ty) => {
        gformat!(
            $fmt $(, $args)* ;
            $validator,
            { $crate::DEFAULT_MIN },
            { $crate::DEFAULT_MAX },
            { $crate::DEFAULT_ASCII_ONLY }
        )
    };

    // -----------------------------------------------------------------------------
    // VALIDATOR + MIN
    // -----------------------------------------------------------------------------
    ($fmt:expr $(, $args:expr)* ; $validator:ty, $min:expr) => {
        gformat!(
            $fmt $(, $args)* ;
            $validator,
            $min,
            { $crate::DEFAULT_MAX },
            { $crate::DEFAULT_ASCII_ONLY }
        )
    };

    // -----------------------------------------------------------------------------
    // VALIDATOR + MIN + MAX
    // -----------------------------------------------------------------------------
    ($fmt:expr $(, $args:expr)* ; $validator:ty, $min:expr, $max:expr) => {
        gformat!(
            $fmt $(, $args)* ;
            $validator,
            $min,
            $max,
            { $crate::DEFAULT_ASCII_ONLY }
        )
    };

    // -----------------------------------------------------------------------------
    // FULL FORM
    // -----------------------------------------------------------------------------
    (
        $fmt:expr $(, $args:expr)* ;
        $validator:ty,
        $min:expr,
        $max:expr,
        $ascii_only:expr
    ) => {{
        #[cfg(feature = "alloc")]
        {
            let s = format!($fmt $(, $args)*);

            $crate::GString::<
                $validator,
                $min,
                $max,
                $ascii_only
            >::try_new(&s)
        }

        #[cfg(not(feature = "alloc"))]
        compile_error!(
            "gformat! requires the `alloc` feature"
        );
    }};
}
