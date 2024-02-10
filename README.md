# Function and `proc_macro` with same name

It is not possible to define both a `proc_macro` and a function
in the same crate.

```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn foo(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

pub fn foo() {}
```

The reason is that, while functions and macros don't share the
same namespace, in a `proc_macro` crate the `proc_macro` is defined
using a function with the same name as the function.

In fact, it's not possible at all to export anything else than
`proc_macro`s, and will give the following error.

> `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`

What you can do is define the `proc_macro` in a separate crate and
re-export it, even with the same name.

```rust
pub use foo_macro::foo;
pub fn foo() {}
```

Then you can use the top crate like this:

```rust
use foo::*;
foo();
foo!();
```

This gist contains the full example you can clone and verify.

```sh
cargo test
```
