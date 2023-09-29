//! [`typed-builder 0.16.2` crate](https://crates.io/crates/typed-builder/0.16.2) demo.
//!
//! ## Summary
//!
//! Pros:
//!
//! Cons:
//! * `default` setting must be set on every field (including `Option<T>`)
//!
//! | Feature | [`typed-builder`](self) |
//! | --- | --- |
//! | [`fn builder()`](#feature---builder-function) | 👍 |
//! | [`Into` field](#feature---into-field) | 🤏 |
//! | [`Option` field](#feature---option-field) | ☝ |
//! | [`Default` struct](#feature---default-struct) | 👎 |
//! | [Collection field](#feature---collection-field) | 👎 |
//! | [`Builder` field](#feature---builder-field) | 👎 |
//! | [`Into` builder](#feature---into-builder) | 👎 |
//! | [Chain call](#feature---chain-call) | 👍 |
//! | [Builder customization](#feature---builder-customization) | 👎 |
//!
//! ## Example
//!
//! ```
//! # use rust_benchmark_setter::typedbuilder::*;
//! # use std::collections::HashMap;
//! let actual = Root::builder()
//!     .number(1)
//!     .boolean(true)
//!     .string("foo")
//!     .opt_string("bar")
//!     .opt_item(Item::builder()
//!         .number(2)
//!         .build()
//!     )
//!     .listitems([
//!         Item::builder()
//!           .number(3)
//!           .build(),
//!     ])
//!     .mapitems([
//!        (
//!          "foobar".into(),
//!           Item::builder()
//!             .number(4)
//!             .build(),
//!        ),
//!     ])
//!     .build();
//!
//! let expected = Root {
//!     number: 1,
//!     boolean: true,
//!     string: "foo".to_owned(),
//!     opt_string: Some("bar".to_owned()),
//!     opt_item: Some(Item {
//!         number: 2,
//!     }),
//!     listitems: vec![
//!         Item {
//!             number: 3,
//!         }
//!     ],
//!     mapitems: HashMap::from([
//!         ("foobar".to_owned(), Item {
//!             number: 4,
//!         })
//!     ]),
//! };
//!
//! //assert_eq!(expected, actual);
//! ```
//!
//! ## Details
//!
//! ##### Feature - Builder function
//!
//! 👍
//!
//! ```
//! # use typed_builder::TypedBuilder;
//! #[derive(TypedBuilder)]
//! struct Root {
//! }
//! Root::builder();
//! ```
//!
//! ##### Feature - Into field
//!
//! 🤏
//!
//! Not available for collection fields.
//!
//! ```
//! # use typed_builder::TypedBuilder;
//! # #[derive(Debug,PartialEq)]
//! #[derive(TypedBuilder)]
//! #[builder(field_defaults(setter(into)))]
//! struct Root {
//!   foo: String,
//!   bar: Option<u8>,
//! }
//! # let root =
//! Root::builder()
//!     .foo("foobar")
//!     .bar(1)
//!     .build();
//! # assert_eq!(
//! #     Root {
//! #       foo: "foobar".to_owned(),
//! #       bar: Some(1),
//! #     },
//! #     root,
//! # );
//! ```
//!
//! ##### Feature - Option field
//!
//! ☝
//!
//! If not all fields are `Option<T>`, must be specified per field.
//!
//! ```
//! # use typed_builder::TypedBuilder;
//! # #[derive(Debug,PartialEq)]
//! #[derive(TypedBuilder)]
//! #[builder(field_defaults(setter(strip_option)))]
//! struct Root {
//!   foo: Option<String>,
//!   bar: Option<u8>,
//! }
//! # let root =
//! Root::builder()
//!     .foo("foobar".to_owned())
//!     .bar(1)
//!     .build();
//! # assert_eq!(
//! #     Root {
//! #       foo: Some("foobar".to_owned()),
//! #       bar: Some(1),
//! #     },
//! #     root,
//! # );
//! ```
//!
//! ##### Feature - Default struct
//!
//! 👎
//!
//! `Default` struct implementation is not supported.
//!
//! ##### Feature - Collection field
//!
//! 👎
//!
//! No option to enable adding a item to collection-like field. More, builders can't be extended with custom code.
//!
//! ##### Feature - Builder field
//!
//! 👎
//!
//! No option to consume a builder to set a field. More, builders can't be extended with custom code.
//!
//! ##### Feature - Into Builder
//!
//! 👎
//!
//! No option to add `Into` support for builders. More, builders can't be extended with custom code.
//!
//! ##### Feature - Chain call
//!
//! 👍
//!
//! ```
//! # use typed_builder::TypedBuilder;
//! # #[derive(Debug,PartialEq)]
//! #[derive(TypedBuilder)]
//! struct Root {
//!   foo: u8,
//!   bar: u8,
//! }
//! # let root =
//! Root::builder()
//!   .foo(1)
//!   .bar(2)
//!   .build();
//! # assert_eq!(
//! #     Root {
//! #       foo: 1,
//! #       bar: 2,
//! #     },
//! #     root,
//! # );
//! ```
//!
//! ##### Feature - Builder customization
//!
//! 👎
//!
//! Can't create custom `impl` block for generated builders
//!

use std::collections::HashMap;
use typed_builder::TypedBuilder;

/// Root data structure
#[derive(Clone,Debug,PartialEq)]
#[derive(TypedBuilder)]
#[builder(field_defaults(default,setter(into)))]
pub struct Root {
    pub number: u8,
    pub boolean: bool,
    pub string: String,
    #[builder(setter(strip_option))]
    pub opt_string: Option<String>,
    #[builder(setter(strip_option))]
    pub opt_item: Option<Item>,
    pub listitems: Vec<Item>,
    pub mapitems: HashMap<String, Item>,
}

/// Sub-item for Root data structure
#[derive(Clone,Debug,PartialEq)]
#[derive(TypedBuilder)]
pub struct Item {
    #[builder(default)]
    pub number: u8,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn usage() {
        let actual = Root::builder()
            .number(1)
            .boolean(true)
            .string("foo")
            .opt_string("bar")
            .opt_item(
                Item::builder()
                    .number(2)
                    .build()
            )
            .listitems([
                Item::builder()
                    .number(3)
                    .build(),
            ])
            .mapitems([
                (
                    "foobar".to_owned(),
                    Item::builder()
                        .number(4)
                        .build(),
                ),
            ])
            .build();
        let expected = Root {
            number: 1,
            boolean: true,
            string: "foo".to_owned(),
            opt_string: Some("bar".to_owned()),
            opt_item: Some(Item {
                number: 2,
            }),
            listitems: vec![
                Item {
                    number: 3,
                }
            ],
            mapitems: HashMap::from([
                ("foobar".to_owned(), Item {
                    number: 4,
                })
            ]),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn default() {
        let actual = Root::builder()
            .build();

        let expected = Root {
            number: 0,
            boolean: false,
            string: "".to_owned(),
            opt_string: None,
            opt_item: None,
            listitems: vec![],
            mapitems: HashMap::from([]),
        };
        assert_eq!(expected, actual);
    }
}
