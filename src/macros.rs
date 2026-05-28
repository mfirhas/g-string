use crate::{Err, GString, GStringError, Validator};

/// GString from const generic constructor.
///
/// Const generic `gstring!` is compile-time check and return this temporary type before you validate it.
///
/// It must be validated before getting actual `GString`.
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

/// Compile-time check for GString without validation.
///
/// Validation can only happen at runtime, so this macro returned [`NotValidatedGString`] needed to be validated.
///
/// It's only useful for checking bounds and ASCII statically at compile-time.
///
/// # Examples
///
/// ```rust
/// use g_string::gstring;
///
/// let ret = gstring!("aslkdm");
/// let ret = ret.validate().unwrap();
/// assert_eq!(ret, "aslkdm");
///
/// // let ret = gstring!("aslkdm", (), 2, 4); // won't even compile
/// ```
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

/// `gformat` formats string like stdlib's `format!` does, with generic/config params pluggable.
///
/// # Examples
/// ```rust
/// use g_string::gformat;
///
/// let ret = gformat!("--> {} - {} equals to {}", 5, 2, "three"; (), 2, 100, true).unwrap();
/// assert_eq!(ret, "--> 5 - 2 equals to three");
///
/// let ret = gformat!("--> fire: {}", "🔥"; (), 2, 102, true);
/// assert!(ret.is_err()); // ASCII_ONLY is true
/// ```
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
