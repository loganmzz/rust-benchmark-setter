//! # Rust - Builder/Setter crate benchmarking
//!
//! Rust project benchmarking builder/setter generation crates
//!
//! While writing data structure with several fields and sub-types, I was looking for a macro able to ease playing with them.
//! So I wanted to write and share a comparison of available crates, regarding either fluent setters or even builders.
//!
//! ## Expected Features
//!
//! * **`&str` input for `String` fields**: Being able to use directly static strings
//!
//! ```rust
//! struct Foobar {
//!     foo: String,
//! }
//!
//! foobar.set_foo("foobar")
//! ```
//!
//! * **Direct input for `Option` fields**: No need to wrap values in `Some(_)`
//!
//! ```rust
//! struct Foobar {
//!     bar: Option<bool>,
//! }
//!
//! foo.set_bar(true)
//! ```
//!
//! * **`Default` support** (Builder only): Avoiding to setup full struct
//!
//! ```rust
//! #[derive(Default)]
//! struct Foobar {
//!     bar: bool,
//! }
//!
//! Foobar::builder().build()
//! ```
//!
//! * **Collection support**: Ease adding new items into a collection-based (`Vec` or `HashMap`) fields
//!
//! ```rust
//! struct Foobar {
//!     foos: Vec<u8>,
//!     bars: HashMap<u8, u8>,
//! }
//!
//! foobar.push_foo(1);
//! foobar.push_bar(1, 1);
//! ```
//!
//! * **Collection builder support**: Avoid import and long call for sub-items *with* function
//!
//! ```rust
//! #[derive(Default)]
//! struct Foobar {
//!     foos: Vec<Foo>
//!     bars: HashMap<u8, Bar>,
//! }
//! #[derive(Default)]
//! struct Foo {
//!     value: bool,
//! }
//! #[derive(Default)]
//! struct Bar {
//!     value: u8,
//! }
//!
//! foobar.push_foo_with_default(|f| f.value(true));
//! foobar.push_bar_with_default(|b| b.value(1));
//!
//! foobar.push_foo_with_builder(|fb| fb.value(true));
//! foobar.push_bar_with_builder(|bb| bb.value(1));
//! ```
//!
//! * **Chain support**: Ease and beauty data structure initialization with *ownership* or *mutable borrow*
//!
//! ```rust
//! struct Foobar {
//!     foo: bool,
//!     bar: u8,
//! }
//!
//! foobar
//!     .set_foo(true)
//!     .set_bar(1);
//! ```
//!
//! ## Benchmarks
//!
//! Each crate is tested into its own module. If variants exist, they are tested into a sub-mobule.
//!
//! * [`blueprint`]: Manually generated codes with demo data structure and target feature implementation.
//!   * [`blueprint::setter`]
//!   * [`blueprint::builder`]

pub mod blueprint;
