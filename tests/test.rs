mod constructors;
mod display;

#[path = "functor_test.rs"]
mod functor_test;

#[path = "equality_test.rs"]
mod equality_test;

#[path = "conversion_test.rs"]
mod conversion_test;

#[path = "order_test.rs"]
mod order_test;

#[path = "error_test.rs"]
mod error_test;

#[path = "macros_test.rs"]
mod macros_test;

#[path = "iterator_test.rs"]
mod iterator_test;

#[cfg(feature = "alloc")]
#[path = "query_test.rs"]
mod query_test;

#[path = "mutation_test.rs"]
mod mutation_test;

#[cfg(feature = "serde")]
#[path = "serde_test.rs"]
mod serde_test;

#[cfg(feature = "secret")]
#[path = "gsecret_test.rs"]
mod gsecret_test;
