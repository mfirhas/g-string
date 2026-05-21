# `g-string`

`g-string` — a stack-allocated, Copy, and const-friendly bounded string type with:
- Compile-time checks via `gstring!(...)` macro — wrong length or non-ASCII caught at compile-time.
- Runtime checks along with validation.
- Custom validation via the Validator trait — pluggable, zero-cost with NoValidation as default.
- Copy — pass around freely.
- FromStr — ergonomic runtime parsing
- AsRef<str>, Display, Debug — plays well with the rest of the ecosystem
