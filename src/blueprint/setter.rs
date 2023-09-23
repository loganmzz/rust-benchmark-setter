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
        self.listitems.push(item().into());
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// let mut root = Root::default();
    /// root.push_listitem_with_default(|i| i);
    /// 
    /// assert_eq!(vec![Item::default()], root.listitems);
    /// ```
    pub fn push_listitem_with_default<FN>(&mut self, upd: FN) -> &mut Self where FN: FnOnce(&mut Item)->&mut Item {
        let mut item = Item::default();
        upd(&mut item);
        self.listitems.push(item);
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
        self.mapitems.insert(key.into(), value().into());
        self
    }
    
    /// ```rust
    /// # use rust_benchmark_setter::blueprint::setter::{Root,Item};
    /// # use std::collections::HashMap;
    /// let mut root = Root::default();
    /// root.push_mapitem_with_default("foobar", |i| i);
    /// 
    /// assert_eq!(
    ///     HashMap::from([
    ///        ("foobar".to_owned(), Item::default()),
    ///     ]),
    ///     root.mapitems,
    /// );
    /// ```
    pub fn push_mapitem_with_default<S,FN>(&mut self, key: S, upd: FN) -> &mut Self where S: Into<String>, FN: FnOnce(&mut Item)->&mut Item {
        let mut item = Item::default();
        upd(&mut item);
        self.mapitems.insert(key.into(), item);
        self
    }
}

