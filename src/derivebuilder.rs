//! [`derive_builder` crate](https://crates.io/crates/derive_builder) demo.
//!
//! ## Summary
//!
//! Pros:
//! * Generate lot of boilerplate code
//! * Several customization
//!
//! Cons:
//! * No static builder init from target struct
//! * Few defaults. You must activate several expected feature (`into`, `strip_option`, `each`, ...)
//! * `build()` methods return a `Result`
//! * No support for nested data builders
//! * No implemention of `Into` for builders
//! * No support of `Into` for `HashMap` entries
//!
//! | Feature | [`derive-builder`](self) |
//! | --- | --- |
//! | [`fn builder()`](#feature---builder-function) | ❌ |
//! | [`Into` field](#feature---into-field) | ⚠ |
//! | [`Option` field](#feature---option-field) | ❗ |
//! | [`Default` struct](#feature---default-struct) | ❗ |
//! | [Collection field](#feature---collection-field) | ❗ |
//! | [`Builder` field](#feature---builder-field) | ❌ |
//! | [`Into` builder](#feature---into-builder) | ❌ |
//! | [Chain call](#feature---chain-call) | ✔ |
//!
//! ## Example
//!
//! ```
//! # use rust_benchmark_setter::derivebuilder::*;
//! # use std::collections::HashMap;
//! let actual = RootBuilder::default()
//!     .number(1)
//!     .boolean(true)
//!     .string("foo")
//!     .opt_string("bar")
//!     .opt_item(ItemBuilder::default()
//!         .number(2)
//!         .build()
//!         .expect("Can't build opt_item value")
//!     )
//!     .listitem(ItemBuilder::default()
//!         .number(3)
//!         .build()
//!         .expect("Can't build listitem value")
//!     )
//!     .mapitem((
//!         "foobar".into(),
//!         ItemBuilder::default()
//!             .number(4)
//!             .build()
//!             .expect("Can't build mapitem value for foobar"),
//!     ))
//!     .build()
//!     .expect("Can't build root");
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
//! assert_eq!(expected, actual);
//! ```
//!
//! ## Details
//!
//! ##### Feature - Builder function
//!
//! ❌
//!
//! No static builder init from target struct:
//!
//! ```
//! # struct Root;
//! # #[derive(Default)]
//! # struct RootBuilder;
//! impl Root {
//!     pub fn builder() -> RootBuilder {
//!         RootBuilder::default()
//!     }
//! }
//! ```
//!
//! ##### Feature - Into field
//!
//! ⚠
//!
//! No support of `Into` for `HashMap` entries
//!
//! ```
//! # #[macro_use]
//! # extern crate derive_builder;
//! # use std::collections::HashMap;
//! #[derive(Builder,Default)]
//! struct Root {
//!   #[builder(setter(each(name="listitem",into)))]
//!   listitems: Vec<Item>,
//!   #[builder(setter(each(name="mapitem",into)))]
//!   mapitems: HashMap<String, Item>,
//! }
//! #[derive(Clone)]
//! struct Item;
//!
//! impl RootBuilder {
//!     /// Missing function
//!     fn mapitem_into<K: Into<String>,V: Into<Item>>(&mut self, key: K, value: V) -> &mut Self {
//!         self.mapitems
//!             .get_or_insert_with(Default::default)
//!             .extend(
//!                 Some((
//!                     key.into(),
//!                     value.into(),
//!                 ))
//!             );
//!         self
//!     }
//! }
//! # pub fn main() {}
//! ```
//!
//! ##### Feature - Option field
//!
//! ❗
//!
//! `setter(strip_option)` setting must be set on every structs:
//!
//! ```
//! # #[macro_use]
//! # extern crate derive_builder;
//! #[derive(Builder)]
//! # #[derive(Debug,PartialEq)]
//! #[builder(setter(strip_option))]
//! struct Root {
//!    item: Option<Item>,
//! }
//! #[derive(Builder,Clone)]
//! # #[derive(Debug,PartialEq)]
//! #[builder(setter(strip_option))]
//! struct Item {
//!    value: Option<u8>,
//! }
//! # pub fn main() {
//! #   let root =
//! RootBuilder::default()
//!     .item(ItemBuilder::default()
//!         .value(1)
//!         .build()
//!         .expect("item error")
//!     )
//!     .build()
//!     .expect("root error");
//! #   assert_eq!(
//! #     Root {
//! #       item: Some(Item {
//! #         value: Some(1),
//! #       }),
//! #     },
//! #     root,
//! #   );
//! # }
//! ```
//!
//! ##### Feature - Default struct
//!
//! ❗
//!
//! `default` setting must be set on every structs:
//!
//! ```
//! # #[macro_use]
//! # extern crate derive_builder;
//! #[derive(Builder,Default)]
//! # #[derive(Debug,PartialEq)]
//! #[builder(default)]
//! struct Root {
//!    item: Item,
//! }
//! #[derive(Builder,Clone,Default)]
//! # #[derive(Debug,PartialEq)]
//! #[builder(default)]
//! struct Item {
//!    value: u8,
//! }
//! # pub fn main() {
//! #   let root =
//! RootBuilder::default()
//!     .build()
//!     .expect("root error");
//! #   assert_eq!(
//! #     Root {
//! #       item: Item {
//! #         value: 0,
//! #       },
//! #     },
//! #     root,
//! #   );
//! # }
//! ```
//!
//! ##### Feature - Collection field
//!
//! ❗
//!
//! `setter(each(...))` setting must be set on every collection field:
//!
//! ```
//! # #[macro_use]
//! # extern crate derive_builder;
//! #[derive(Builder)]
//! # #[derive(Debug,PartialEq)]
//! struct Root {
//!    #[builder(setter(each(name="foo")))]
//!    foos: Vec<u8>,
//! }
//! # pub fn main() {
//! #   let root =
//! RootBuilder::default()
//!     .foo(1)
//!     .foo(2)
//!     .build()
//!     .expect("root error");
//! #   assert_eq!(
//! #     Root {
//! #       foos: vec![1, 2],
//! #     },
//! #     root,
//! #   );
//! # }
//! ```
//!
//! ##### Feature - Builder field
//!
//! ❌
//!
//! No support for nested data builders
//! ```
//! # struct RootBuilder;
//! # #[derive(Default)]
//! # struct ItemBuilder;
//! # struct ItemBuilderError;
//! # impl RootBuilder {
//! #   pub fn listitem(&mut self, _: ()) ->&mut Self { self }
//! # }
//! # impl ItemBuilder {
//! #   pub fn build(&self) -> Result<(),ItemBuilderError> { Ok(()) }
//! # }
//! impl RootBuilder {
//!     pub fn listitem_with<FN: FnOnce(&mut ItemBuilder)>(&mut self, f: FN) -> Result<&mut Self,ItemBuilderError> {
//!         let mut builder = ItemBuilder::default();
//!         f(&mut builder);
//!         Ok(self.listitem(builder.build()?))
//!     }
//! }
//! ```
//!
//! ##### Feature - Into Builder
//!
//! ❌
//!
//! No implemention of `Into` for builders
//! ```
//! # struct Root;
//! # struct RootBuilder;
//! # impl RootBuilder {
//! #   pub fn build(&mut self) -> Result<Root, ()> { Ok(Root) }
//! # }
//! impl Into<Root> for &mut RootBuilder {
//!     fn into(self) -> Root {
//!         self.build().expect("unable to build Root")
//!     }
//! }
//! ```
//!
//! ##### Feature - Chain call
//!
//! ✔
//!

use std::collections::HashMap;

/// Root data structure
#[derive(Builder,Clone,Debug,Default,PartialEq)]
#[builder(default,setter(into,strip_option))]
pub struct Root {
    pub number: u8,
    pub boolean: bool,
    pub string: String,
    pub opt_string: Option<String>,
    pub opt_item: Option<Item>,
    #[builder(setter(each(name="listitem",into)))]
    pub listitems: Vec<Item>,
    #[builder(setter(each(name="mapitem",into)))]
    pub mapitems: HashMap<String, Item>,
}

/// Sub-item for Root data structure
#[derive(Builder,Clone,Debug,Default,PartialEq)]
#[builder(default,setter(into,strip_option))]
pub struct Item {
    pub number: u8,
}

impl Root {
    pub fn builder() -> RootBuilder {
        RootBuilder::default()
    }
}

impl Item {
    pub fn builder() -> ItemBuilder {
        ItemBuilder::default()
    }
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
                .opt_item(Item::builder().number(2).build().expect("Unable to build opt_item"))
                .listitem(Item::builder().number(3).build().expect("Unable to build listitem"))
                .mapitem(("foobar".into(), Item::builder().number(4).build().expect("Unable to build mapitem")))
                .build().expect("Unable to build root");

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
    pub fn support_default() {
        let actual = Root::builder().build().expect("Failed to default");

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
