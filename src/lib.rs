//! # Rust - Builder/Setter crate benchmarking
//!
//! Rust project benchmarking builder/setter generation crates
//!
//! While writing data structure with several fields and sub-types, I was looking for a macro able to ease playing with them.
//! So I wanted to write and share a comparison of available crates, regarding either fluent setters or even builders.
//!
//! ## Comparison
//!
//! | Feature                                                  | [`derive-builder 0.12.0`](derivebuilder)                            | [`typed_builder 0.16.2`](typedbuilder)                             |
//! | -----------------------------------------------          | ---------------------------------------------------------    | -------------------------------------------------------     |
//! | [`fn builder()`](#feature---builder-function)            | [✋](derivebuilder/index.html#feature---builder-function)    | [👍](typedbuilder/index.html#feature---builder-function)     |
//! | [`Into` field](#feature---into-field)                    | [🤏](derivebuilder/index.html#feature---into-field)           | [🤏](typedbuilder/index.html#feature---into-field)          |
//! | [`Option` field](#feature---option-field)                | [👍](derivebuilder/index.html#feature---option-field)        | [☝](typedbuilder/index.html#feature---option-field)        |
//! | [`Default` struct](#feature---default-struct)            | [👍](derivebuilder/index.html#feature---default-struct)      | [👎](typedbuilder/index.html#feature---default-struct)      |
//! | [Collection field](#feature---collection-field)          | [☝](derivebuilder/index.html#feature---collection-field)    | [👎](typedbuilder/index.html#feature---collection-field)     |
//! | [`Builder` field](#feature---builder-field)              | [✋](derivebuilder/index.html#feature---builder-field)       | [👎](typedbuilder/index.html#feature---builder-field)        |
//! | [`Into` builder](#feature---into-builder)                | [✋](derivebuilder/index.html#feature---into-builder)        | [👎](typedbuilder/index.html#feature---into-builder)         |
//! | [Chain call](#feature---chain-call)                      | [👍](derivebuilder/index.html#feature---chain-call)           | [👍](typedbuilder/index.html#feature---chain-call)           |
//! | [Builder customization](#feature---builder-customization) | [👍](derivebuilder/index.html#feature---builder-customization) | [👎](typedbuilder/index.html#feature---builder-customization) |
//!
//! Legend:
//! * 👍: supported (may require configuration per struct)
//! * ☝: supported (but require field configuration)
//! * 🤏: partially supported (see details)
//! * ✋: not supported (but custom code possible)
//! * 👎: not supported (and can't add custom code)
//!
//! ## Expected Features
//!
//! ##### Feature - Builder function
//!
//! **`fn builder()`**: Being able to init Builder struct, without importing it
//!
//! ```
//! struct Foobar {}
//! # impl Foobar {
//! #   pub fn builder() {}
//! # }
//! Foobar::builder()
//! ```
//!
//! ##### Feature - Into field
//!
//! **`&str` input for `String` fields**: Being able to use directly static strings
//!
//! ```
//! struct Foobar {
//!     foo: String,
//! }
//! # impl Foobar {
//! #   pub fn set_foo(&self, foo: &str) {}
//! # }
//! # let foobar = Foobar { foo: "".into() };
//! foobar.set_foo("foobar")
//! ```
//!
//! ##### Feature - Option field
//!
//! **Direct input for `Option` fields**: No need to wrap values in `Some(_)`
//!
//! ```
//! struct Foobar {
//!     bar: Option<bool>,
//! }
//! # impl Foobar {
//! #   pub fn set_bar(&self, bar: bool) {}
//! # }
//! # let foobar = Foobar { bar: None };
//! foobar.set_bar(true)
//! ```
//!
//! ##### Feature - Default struct
//!
//! **`Default` support** (Builder only): Avoiding to setup full struct
//!
//! ```
//! #[derive(Default)]
//! struct Foobar {
//!     bar: bool,
//! }
//! # struct FoobarBuilder;
//! # impl Foobar {
//! #   pub fn builder() -> FoobarBuilder {
//! #     FoobarBuilder
//! #   }
//! # }
//! # impl FoobarBuilder {
//! #   pub fn build(&self) {}
//! # }
//! Foobar::builder().build()
//! ```
//!
//! ##### Feature - Collection field
//!
//! **Collection support**: Ease adding new items into a collection-based (`Vec` or `HashMap`) fields
//!
//! ```
//! # #[derive(Default)]
//! struct Foobar {
//!     foos: Vec<u8>,
//!     bars: HashMap<u8, u8>,
//! }
//! # use std::collections::HashMap;
//! # impl Foobar {
//! #   pub fn push_foo(&self, foo: u8) {}
//! #   pub fn push_bar(&self, bar_key: u8, bar_value: u8) {}
//! # }
//! # let foobar = Foobar::default();
//! foobar.push_foo(1);
//! foobar.push_bar(1, 1);
//! ```
//!
//! ##### Feature - Builder field
//!
//! **Collection builder support**: Avoid import and long call for sub-items *with* function
//!
//! ```
//! # use std::collections::HashMap;
//! # #[derive(Default)]
//! struct Foobar {
//!     foos: Vec<Foo>,
//!     bars: HashMap<u8, Bar>,
//! }
//! struct Foo {
//!     value: bool,
//! }
//! struct Bar {
//!     value: u8,
//! }
//! # impl Foobar {
//! #   pub fn push_foo_with_default<FN: FnOnce(&Foo)>(&self, f: FN) {}
//! #   pub fn push_bar_with_default<FN: FnOnce(&Bar)>(&self, f: FN) {}
//! #   pub fn push_foo_with_builder<FN: FnOnce(&Foo)>(&self, f: FN) {}
//! #   pub fn push_bar_with_builder<FN: FnOnce(&Bar)>(&self, f: FN) {}
//! # }
//! # impl Foo {
//! #    pub fn value(&self, value: bool) {}
//! # }
//! # impl Bar {
//! #   pub fn value(&self, value: u8) {}
//! # }
//! # let foobar = Foobar::default();
//! foobar.push_foo_with_default(|f| f.value(true));
//! foobar.push_bar_with_default(|b| b.value(1));
//! foobar.push_foo_with_builder(|fb| fb.value(true));
//! foobar.push_bar_with_builder(|bb| bb.value(1));
//! ```
//!
//! ##### Feature - Into Builder
//!
//! **`Into` for builder**: Ease builder usage as `Into` is used for convenient trait bound for inputs (including builder params):
//!
//! ```
//! struct Foobar {
//!     foo: Foo,
//! }
//! struct Foo;
//! # struct FoobarBuilder;
//! # struct FooBuilder;
//! # impl Foobar {
//! #   pub fn builder() -> FoobarBuilder {
//! #     FoobarBuilder {}
//! #   }
//! # }
//! # impl FoobarBuilder {
//! #   pub fn foo<F: Into<Foo>>(&self, foo: F) {}
//! # }
//! # impl Foo {
//! #   pub fn builder() -> FooBuilder { FooBuilder }
//! # }
//! # impl Into<Foo> for FooBuilder {
//! #   fn into(self) -> Foo { Foo }
//! # }
//! Foobar::builder().foo(Foo::builder())
//! ```
//!
//! ##### Feature - Chain call
//!
//! **Chain support**: Ease and beauty data structure initialization with *ownership* or *mutable borrow*
//!
//! ```
//! # #[derive(Default)]
//! struct Foobar {
//!     foo: bool,
//!     bar: u8,
//! }
//! # impl Foobar {
//! #    pub fn set_foo(&self, foo: bool) -> &Self { self }
//! #    pub fn set_bar(&self, bar: u8) -> &Self { self }
//! # }
//! # let foobar = Foobar::default();
//! foobar
//!     .set_foo(true)
//!     .set_bar(1);
//! ```
//!
//! ##### Feature - Builder customization
//!
//! **Builder customization**: Let's extend capability (including unsupported features)
//!
//! ```
//! struct Foobar {
//! }
//! # struct FoobarBuilder;
//! impl FoobarBuilder {
//!   fn extension(&mut self) -> &mut Self {
//!     println!("Extension");
//!     self
//!   }
//! }
//! ```
//!
//! ## Benchmarks
//!
//! Each crate is tested into its own module. If variants exist, they are tested into a sub-mobule.
//!
//! * [`blueprint`]: Manually generated codes with demo data structure and target feature implementation.
//!   * [`blueprint::setter`]
//!   * [`blueprint::builder`]
//! * [`derivebuilder`]: Using [`derive_builder 0.12.0` crate](https://crates.io/crates/derive_builder/0.12.0)
//! * [`typedbuilder`]: Using [`typed-builder 0.16.2` crate](https://crates.io/crates/typed-builder/0.16.2)
//!

#[macro_use]
extern crate derive_builder;

pub mod blueprint;
pub mod derivebuilder;
pub mod typedbuilder;
