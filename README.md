# `g-string`

`g-string` — a stack-allocated, Copy, and generically configurable string type with:
- Compile-time parsing and checks via `gstring!(...)` macro — wrong length or non-ASCII caught at compile-time.
- Runtime parsing and checks along with validation.
- Custom validation via the Validator trait — pluggable, zero-cost with NoValidation as default.
- Copy — pass around freely.
