# Rust: How export a function and a `proc_macro` with same name

I was in the process of writing a macro helper when I realized that
the macro might also be useful as a function for other `proc_macro`
crates.
But then I ran into the problem that is not possible to export both
a `proc_macro` and a function in the same crate.

Because searching did not give an answer I decided to write down
the solution here.
In the end, I ended up defining a trait instead though :)

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
using a function with the same name.

In fact, it's not possible at all to export anything other than
`proc_macro`s, and will give the following error.

> `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`

After consulting the
[Rust Programming Language Community Discord](https://discord.gg/rust-lang-community), [bendn](https://github.com/bend-n) suggested using a separate crate and re-exporting.

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

This repo contains the full example you can clone and verify.

```sh
git clone https://github.com/fredrik-hammar/rust-function-and-proc-macro-with-same-name.git
cd rust-function-and-proc-macro-with-same-name
cargo test
```
