//! Blueprint for hand-written fluent setters.
//! 
//! ```rust
//! # use rust_benchmark_setter::blueprint::setter::{Root,Item};
//! # use std::collections::HashMap;
//! let actual = Root::default()
//!     .with(|r| { r
//!         .set_number(1)
//!         .set_boolean(true)
//!         .set_string("foo")
//!         .set_opt_string("bar")
//!         .set_opt_item_with_default(|i| { i.set_number(2); })
//!         .push_listitem_with_default(|i| { i.set_number(3); })
//!         .push_mapitem_with_default("foobar", |i| { i.set_number(4); })
//!         ;
//!     });
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

impl Root {
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::Root;
    /// let root = Root::default().with(|r| r.number = 1);
    /// 
    /// assert_eq!(1, root.number);
    /// ```
    pub fn with<FN>(mut self, f: FN) -> Self where FN: FnOnce(&mut Self) {
        f(&mut self);
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::Root;
    /// let mut root = Root::default();
    /// root.set_number(1);
    /// 
    /// assert_eq!(1, root.number);
    /// ```
    pub fn set_number<N: Into<u8>>(&mut self, number: N) -> &mut Self {
        self.number = number.into();
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::Root;
    /// let mut root = Root::default();
    /// root.set_boolean(true);
    /// 
    /// assert_eq!(true, root.boolean);
    /// ```
    pub fn set_boolean<B: Into<bool>>(&mut self, boolean: B) -> &mut Self {
        self.boolean = boolean.into();
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::Root;
    /// let mut root = Root::default();
    /// root.set_string("foobar");
    /// 
    /// assert_eq!("foobar", root.string);
    /// ```
    pub fn set_string<S: Into<String>>(&mut self, string: S) -> &mut Self {
        self.string = string.into();
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::Root;
    /// let mut root = Root::default();
    /// root.set_opt_string("foobar");
    /// 
    /// assert_eq!(Some("foobar"), root.opt_string.as_deref());
    /// ```
    pub fn set_opt_string<S: Into<String>>(&mut self, string: S) -> &mut Self {
        self.opt_string = Some(string.into());
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// let mut root = Root::default();
    /// root.set_opt_item(Item::default());
    /// 
    /// assert_eq!(Some(Item::default()), root.opt_item);
    /// ```
    pub fn set_opt_item<I: Into<Item>>(&mut self, item: I) -> &mut Self {
        self.opt_item = Some(item.into());
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// let mut root = Root::default();
    /// root.set_opt_item_with(|| Item::default());
    /// 
    /// assert_eq!(Some(Item::default()), root.opt_item);
    /// ```
    pub fn set_opt_item_with<FN,I>(&mut self, item: FN) -> &mut Self where FN: FnOnce()->I, I: Into<Item> {
        self.set_opt_item(item())
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// let mut root = Root::default();
    /// root.set_opt_item_with_default(|i| i.number = 1);
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
    pub fn set_opt_item_with_default<FN>(&mut self, f: FN) -> &mut Self where FN: FnOnce(&mut Item) {
        let mut item = Item::default();
        f(&mut item);
        self.set_opt_item(item)
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// let mut root = Root::default();
    /// root.push_listitem(Item::default());
    /// 
    /// assert_eq!(vec![Item::default()], root.listitems);
    /// ```
    pub fn push_listitem<I: Into<Item>>(&mut self, item: I) -> &mut Self {
        self.listitems.push(item.into());
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// let mut root = Root::default();
    /// root.push_listitem_with(|| Item::default());
    /// 
    /// assert_eq!(vec![Item::default()], root.listitems);
    /// ```
    pub fn push_listitem_with<FN,I>(&mut self, item: FN) -> &mut Self where FN: FnOnce()->I, I: Into<Item> {
        self.push_listitem(item())
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// let mut root = Root::default();
    /// root.push_listitem_with_default(|i| {});
    /// 
    /// assert_eq!(vec![Item::default()], root.listitems);
    /// ```
    pub fn push_listitem_with_default<FN>(&mut self, f: FN) -> &mut Self where FN: FnOnce(&mut Item) {
        let mut item = Item::default();
        f(&mut item);
        self.push_listitem(item)
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// # use std::collections::HashMap;
    /// let mut root = Root::default();
    /// root.push_mapitem("foobar", Item::default());
    /// 
    /// assert_eq!(
    ///     HashMap::from([
    ///        ("foobar".to_owned(), Item::default()),
    ///     ]),
    ///     root.mapitems,
    /// );
    /// ```
    pub fn push_mapitem<S:Into<String>, I: Into<Item>>(&mut self, key: S, value: I) -> &mut Self {
        self.mapitems.insert(key.into(), value.into());
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// # use std::collections::HashMap;
    /// let mut root = Root::default();
    /// root.push_mapitem_with("foobar", || Item::default());
    /// 
    /// assert_eq!(
    ///     HashMap::from([
    ///        ("foobar".to_owned(), Item::default()),
    ///     ]),
    ///     root.mapitems,
    /// );
    /// ```
    pub fn push_mapitem_with<S,FN,I>(&mut self, key: S, value: FN) -> &mut Self where S: Into<String>, FN: FnOnce()->I, I: Into<Item> {
        self.push_mapitem(key, value())
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// # use std::collections::HashMap;
    /// let mut root = Root::default();
    /// root.push_mapitem_with_default("foobar", |i| i.number = 1);
    /// 
    /// assert_eq!(
    ///     HashMap::from([
    ///         ("foobar".to_owned(), {
    ///             let mut item = Item::default();
    ///             item.number = 1;
    ///             item
    ///         }),
    ///     ]),
    ///     root.mapitems,
    /// );
    /// ```
    pub fn push_mapitem_with_default<S,FN>(&mut self, key: S, f: FN) -> &mut Self where S: Into<String>, FN: FnOnce(&mut Item) {
        let mut item = Item::default();
        f(&mut item);
        self.push_mapitem(key, item)
    }
}

impl Item {
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::Item;
    /// let item = Item::default().with(|i| i.number = 1);
    /// 
    /// assert_eq!(1, item.number);
    /// ```
    pub fn with<FN>(mut self, f: FN) -> Self where FN: FnOnce(&mut Self) {
        f(&mut self);
        self
    }

    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::Item;
    /// let mut item = Item::default();
    /// item.set_number(1);
    /// 
    /// assert_eq!(1, item.number);
    /// ```
    pub fn set_number<N: Into<u8>>(&mut self, number: N) -> &mut Self {
        self.number = number.into();
        self
    }
}
