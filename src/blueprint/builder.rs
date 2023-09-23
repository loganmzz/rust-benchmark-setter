//! Blueprint for hand-written builders.
//!
//! ```rust
//! # use rust_benchmark_setter::blueprint::builder::{Root,Item};
//! # use std::collections::HashMap;
//! let actual = Root::builder()
//!     .number(1)
//!     .boolean(true)
//!     .string("foo")
//!     .opt_string("bar")
//!     .opt_item_with(|i| i.number(2))
//!     .listitem_with(|i| i.number(3))
//!     .mapitem_with("foobar", |i| i.number(4))
//!     .build();
//!
//! let expected = Root {
//!     number: 1,
//!     boolean: true,
//!     string: "foo".to_owned(),
//!     opt_string: Some("bar".to_owned()),
//!     opt_item: Some(Item {
//!         number: 2,
//!         boolean: false,
//!         string: "".to_owned(),
//!         opt_string: None,
//!     }),
//!     listitems: vec![
//!         Item {
//!             number: 3,
//!             boolean: false,
//!             string: "".to_owned(),
//!             opt_string: None,
//!         }
//!     ],
//!     mapitems: HashMap::from([
//!         ("foobar".to_owned(), Item {
//!             number: 4,
//!             boolean: false,
//!             string: "".to_owned(),
//!             opt_string: None,
//!         })
//!     ]),
//! };
//!
//! assert_eq!(expected, actual);
//! ```

use std::collections::HashMap;

pub trait TakeWithOption<T> {
    fn take_with<FN: FnMut(T)>(&mut self, f: FN);
}

impl<T> TakeWithOption<T> for Option<T> {
    fn take_with<FN: FnOnce(T)>(&mut self, f: FN) {
        if let Some(value) = self.take() {
            f(value);
        }
    }
}

/// Root data structure
#[derive(Debug,Default,PartialEq)]
pub struct Root {
    pub number: u8,
    pub boolean: bool,
    pub string: String,
    pub opt_string: Option<String>,
    pub opt_item: Option<Item>,
    pub listitems: Vec<Item>,
    pub mapitems: HashMap<String, Item>,
}

/// Sub-item for Root data structure
#[derive(Debug,Default,PartialEq)]
pub struct Item {
    pub number: u8,
    pub boolean: bool,
    pub string: String,
    pub opt_string: Option<String>,
}

#[derive(Default)]
pub struct RootBuilder {
    number: Option<u8>,
    boolean: Option<bool>,
    string: Option<String>,
    opt_string: Option<Option<String>>,
    opt_item: Option<Option<Item>>,
    listitems: Option<Vec<Item>>,
    mapitems: Option<HashMap<String, Item>>,
}

#[derive(Default)]
pub struct ItemBuilder {
    number: Option<u8>,
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

impl Into<Root> for &mut RootBuilder {
    fn into(self) -> Root {
        self.build()
    }
}

impl RootBuilder {
    pub fn build(&mut self) -> Root {
        let mut root = Root::default();
        self.number.take_with(|number| root.number = number);
        self.boolean.take_with(|boolean| root.boolean = boolean);
        self.string.take_with(|string| root.string = string);
        self.opt_string.take_with(|opt_string| root.opt_string = opt_string);
        self.opt_item.take_with(|opt_item| root.opt_item = opt_item);
        self.listitems.take_with(|listitems| root.listitems = listitems);
        self.mapitems.take_with(|mapitems| root.mapitems = mapitems);
        root
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::Root;
    /// let root = Root::builder()
    ///     .number(1)
    ///     .build();
    ///
    /// assert_eq!(1, root.number);
    /// ```
    pub fn number<N: Into<u8>>(&mut self, number: N) -> &mut Self {
        self.number = Some(number.into());
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::Root;
    /// let root = Root::builder()
    ///     .boolean(true)
    ///     .build();
    ///
    /// assert_eq!(true, root.boolean);
    /// ```
    pub fn boolean<N: Into<bool>>(&mut self, boolean: N) -> &mut Self {
        self.boolean = Some(boolean.into());
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::Root;
    /// let root = Root::builder()
    ///     .string("foobar")
    ///     .build();
    ///
    /// assert_eq!("foobar", root.string);
    /// ```
    pub fn string<N: Into<String>>(&mut self, string: N) -> &mut Self {
        self.string = Some(string.into());
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::Root;
    /// let root = Root::builder()
    ///     .opt_string("foobar")
    ///     .build();
    ///
    /// assert_eq!(Some("foobar"), root.opt_string.as_deref());
    /// ```
    pub fn opt_string<N: Into<String>>(&mut self, string: N) -> &mut Self {
        self.opt_string = Some(Some(string.into()));
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::{Root,Item};
    /// let root = Root::builder()
    ///     .opt_item({
    ///         let mut item = Item::default();
    ///         item.number = 1;
    ///         item
    ///     })
    ///     .build();
    ///
    /// assert_eq!(
    ///     Some({
    ///         let mut item = Item::default();
    ///         item.number = 1;
    ///         item
    ///     }),
    ///     root.opt_item,
    /// );
    /// ```
    pub fn opt_item<N: Into<Item>>(&mut self, item: N) -> &mut Self {
        self.opt_item = Some(Some(item.into()));
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::{Root,Item};
    /// let root = Root::builder()
    ///     .opt_item_with(|item|
    ///         item.number(1)
    ///     )
    ///     .build();
    ///
    /// assert_eq!(
    ///     Some({
    ///         let mut item = Item::default();
    ///         item.number = 1;
    ///         item
    ///     }),
    ///     root.opt_item,
    /// );
    /// ```
    pub fn opt_item_with<FN>(&mut self, item: FN) -> &mut Self where FN: FnOnce(&mut ItemBuilder)->&mut ItemBuilder {
        self.opt_item(item(&mut Item::builder()).build())
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::{Root,Item};
    /// let mut root = Root::builder()
    ///     .listitem({
    ///         let mut item = Item::default();
    ///         item.number = 1;
    ///         item
    ///     })
    ///     .build();
    ///
    /// assert_eq!(
    ///     vec![{
    ///         let mut item = Item::default();
    ///         item.number = 1;
    ///         item
    ///     }],
    ///     root.listitems,
    /// );
    /// ```
    pub fn listitem<I: Into<Item>>(&mut self, item: I) -> &mut Self {
        self.listitems.get_or_insert_with(|| Vec::new()).push(item.into());
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::{Root,Item};
    /// let mut root = Root::builder()
    ///     .listitem_with(|i| i.number(1))
    ///     .build();
    ///
    /// assert_eq!(
    ///     vec![{
    ///         let mut item = Item::default();
    ///         item.number = 1;
    ///         item
    ///     }],
    ///     root.listitems,
    /// );
    /// ```
    pub fn listitem_with<FN>(&mut self, item: FN) -> &mut Self where FN: FnOnce(&mut ItemBuilder)->&mut ItemBuilder {
        self.listitem(item(&mut Item::builder()).build())
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::{Root,Item};
    /// # use std::collections::HashMap;
    /// let mut root = Root::builder()
    ///     .mapitem("foobar", {
    ///         let mut item = Item::default();
    ///         item.number = 1;
    ///         item
    ///     })
    ///     .build();
    ///
    /// assert_eq!(
    ///     HashMap::from([
    ///         (
    ///             "foobar".to_owned(),
    ///             {
    ///                 let mut item = Item::default();
    ///                 item.number = 1;
    ///                 item
    ///             },
    ///         ),
    ///     ]),
    ///     root.mapitems,
    /// );
    /// ```
    pub fn mapitem<S: Into<String>, I: Into<Item>>(&mut self, key: S, item: I) -> &mut Self {
        self.mapitems.get_or_insert_with(|| HashMap::new()).insert(key.into(), item.into());
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::{Root,Item};
    /// # use std::collections::HashMap;
    /// let mut root = Root::builder()
    ///     .mapitem_with("foobar", |i| i.number(1))
    ///     .build();
    ///
    /// assert_eq!(
    ///     HashMap::from([
    ///         (
    ///             "foobar".to_owned(),
    ///             {
    ///                 let mut item = Item::default();
    ///                 item.number = 1;
    ///                 item
    ///             },
    ///         ),
    ///     ]),
    ///     root.mapitems,
    /// );
    /// ```
    pub fn mapitem_with<S,I>(&mut self, key: S, value: I) -> &mut Self where S: Into<String>, I: FnOnce(&mut ItemBuilder)->&mut ItemBuilder {
        self.mapitem(key, value(&mut Item::builder()).build())
    }
}

impl ItemBuilder {
    pub fn build(&mut self) -> Item {
        let mut item = Item::default();
        self.number.take_with(|number| item.number = number);
        item
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::builder::Item;
    /// let item = Item::builder()
    ///     .number(1)
    ///     .build();
    ///
    /// assert_eq!(1, item.number);
    /// ```
    pub fn number<N: Into<u8>>(&mut self, number: N) -> &mut Self {
        self.number = Some(number.into());
        self
    }
}
