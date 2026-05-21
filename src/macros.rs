use crate::{GString, Validator};

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    #[doc(hidden)]
    pub const fn __constant(s: &str) -> Self {
        Self::build(s).comptime_check()
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
        const RET: $crate::GString<$validator, $min, $max, $ascii_only> =
            $crate::GString::<$validator, $min, $max, $ascii_only>::__constant($s);
        RET
    }};
}
