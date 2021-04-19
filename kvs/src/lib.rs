#![deny(missing_docs)]

//! kvs - a simple key-value store
//!
//! # Example
//! ```
//! use kvs::KvStore;
//!
//! let mut store = KvStore::new();
//!
//! store.set( String::from("key1"), String::from("value1") );
//! let value1 = store.get(String::from("key1"));
//! assert_eq!(value1,Some("value1".into()));
//!
//! let value2 = store.get(String::from("key2"));
//! assert_eq!(value2,None);
//!
//! store.remove(String::from("key1"));
//! let value1 = store.get(String::from("key1"));
//! assert_eq!(value1,None);
//! ```
//!

use std::{collections::HashMap, path};

mod error;

pub use error::{Error, ErrorKind, Result};

/// Simple Key-Value Storage Type
pub struct KvStore {
    storage: HashMap<String, String>,
}

impl KvStore {
    /// create a new empty Key-Value storage instance
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let store: KvStore = KvStore::new();
    /// ```
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
    /// open a disk-based, log-based storage at a path
    ///
    /// # Example
    /// ```
    ///
    /// ```
    pub fn open(location: &path::Path) -> Result<Self> {
        todo!()
    }
    /// set a key to a value in the Key-Value Storage instance
    ///
    /// If the key is already set to a value this overwrites the
    /// value under the key with the new value
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store: KvStore = KvStore::new();
    /// store.set("key1".into(),"value1".into());
    /// store.set("key1".into(),"value2".into());
    /// let value = store.get("key1".into());
    /// assert_eq!(value,Some("value2".into()));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.storage.insert(key, value);
        Ok(())
    }
    /// get the value stored under the given key or None if no such key
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store: KvStore = KvStore::new();
    /// store.set("key1".into(),"value1".into());
    /// let value = store.get("key1".into());
    /// assert_eq!(value,Some("value1".into()));
    /// let value = store.get("key2".into());
    /// assert_eq!(value,None);
    /// ```
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.storage.get(&key).map(String::clone))
    }
    /// remove the value stored under the given key or no-op if the key does not exist
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store: KvStore = KvStore::new();
    /// store.set("key1".into(),"value1".into());
    /// let value = store.get("key1".into());
    /// assert_eq!(value,Some("value1".into()));
    /// store.remove("key1".into());
    /// let value = store.get("key1".into());
    /// assert_eq!(value,None);
    /// store.remove("key2".into());
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.storage.remove(&key);
        Ok(())
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
