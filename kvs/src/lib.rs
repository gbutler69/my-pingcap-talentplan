#![deny(missing_docs)]

//! kvs - a simple key-value store
//!
//! # Example
//! ```
//! use kvs::KvStore;
//!
//! let mut store = KvStore::<String, String>::new(std::path::Path::new("tests.kvsdb")).unwrap();
//!
//! let _ = store.set(String::from("key1"), String::from("value1"));
//! let value1 = store.get(String::from("key1")).unwrap();
//! assert_eq!(value1, Some("value1".into()));
//!
//! let value2 = store.get(String::from("key2")).unwrap();
//! assert!(value2.is_none());
//!
//! let _ = store.remove(String::from("key1"));
//! let value1 = store.get(String::from("key1")).unwrap();
//! assert_eq!(value1, None);
//! ```
//!

use std::{
    collections::HashMap,
    fs, hash,
    io::{self, Seek, Write},
    marker,
    path::{self, Path},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod error;
pub use error::{Error, ErrorKind, Result};

/// Simple Key-Value Storage Type
pub struct KvStore<K, V> {
    index: HashMap<K, u64>,
    reader: io::BufReader<fs::File>,
    writer: io::BufWriter<fs::File>,
    phantom_value: marker::PhantomData<V>,
}

/// Key-Value Storage Record
#[derive(Debug, Serialize, Deserialize)]
pub struct Record<K, V> {
    db_key: u64,
    key: K,
    value: Option<V>,
}

impl<K, V> KvStore<K, V>
where
    K: Serialize + DeserializeOwned + Eq + PartialEq + hash::Hash + Clone,
    V: Serialize + DeserializeOwned + Clone,
{
    /// create a new empty Key-Value storage instance
    /// If the file exists already, it truncates it to empty. In any case, the file is opened for reading/writing.
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let store = KvStore::<String,String>::new(std::path::Path::new("tests.kvsdb")).unwrap();
    /// ```
    pub fn new(path: &Path) -> Result<Self> {
        let writer = io::BufWriter::new(
            fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)?,
        );
        let reader = io::BufReader::new(fs::OpenOptions::new().read(true).open(path)?);
        Ok(Self {
            index: HashMap::new(),
            reader,
            writer,
            phantom_value: marker::PhantomData::default(),
        })
    }
    /// open a disk-based, log-based storage at a path
    /// If the file exists it opens for reading and appending. If the file does not exist it creates it.
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let store = KvStore::<String,String>::open(std::path::Path::new("tests.kvsdb")).unwrap();
    /// ```
    pub fn open(path: &path::Path) -> Result<Self> {
        let writer = io::BufWriter::new(
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?,
        );
        let reader = io::BufReader::new(fs::OpenOptions::new().read(true).open(path)?);
        let index = HashMap::new();
        let mut kv_store = Self {
            index,
            reader,
            writer,
            phantom_value: marker::PhantomData::default(),
        };
        while let Some(rec) = kv_store.read_next_record()? {
            let _ = match rec {
                Record {
                    db_key,
                    key,
                    value: Some(_),
                } => kv_store.index.insert(key, db_key),
                Record {
                    key, value: None, ..
                } => kv_store.index.remove(&key),
            };
        }
        Ok(kv_store)
    }
    fn read_next_record(&mut self) -> Result<Option<Record<K, V>>> {
        let vec = &mut Vec::new();
        let read_value =
            serde_asn1_der::from_reader(&mut self.reader, serde_asn1_der::VecBacking(vec));
        match read_value {
            Ok(rec) => Ok(Some(rec)),
            Err(serde_asn1_der::SerdeAsn1DerError::Asn1DerError(_)) => Ok(None),
            Err(_) => Err(Error::new(ErrorKind::IoError)),
        }
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
    /// let mut store = KvStore::<String,String>::new(std::path::Path::new("tests.kvsdb")).unwrap();
    /// let _ = store.set("key1".into(),"value1".into());
    /// let _ = store.set("key1".into(),"value2".into());
    /// let value = store.get("key1".into()).unwrap();
    /// assert_eq!(value,Some("value2".into()));
    /// ```
    pub fn set(&mut self, key: K, value: V) -> Result<()> {
        let rec = Record {
            db_key: self.writer.get_ref().stream_position()?,
            key: key.clone(),
            value: Some(value),
        };
        if serde_asn1_der::to_writer(&rec, &mut self.writer).is_err() {
            self.writer.seek(io::SeekFrom::Start(rec.db_key))?;
            return Err(Error::new(ErrorKind::IoError));
        }
        let _ = self.writer.flush();
        self.index.insert(key, rec.db_key);
        Ok(())
    }
    /// get the value stored under the given key or None if no such key
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::<String,String>::new(std::path::Path::new("tests.kvsdb")).unwrap();
    /// let _ = store.set("key1".into(),"value1".into());
    /// let value = store.get("key1".into()).unwrap();
    /// assert_eq!(value,Some("value1".into()));
    /// let value = store.get("key2".into()).unwrap();
    /// assert_eq!(value,None);
    /// ```
    pub fn get(&mut self, key: K) -> Result<Option<V>> {
        let db_key = match self.index.get(&key) {
            Some(&db_key) => db_key,
            None => return Ok(None),
        };
        let _ = self.reader.seek(io::SeekFrom::Start(db_key))?;
        self.read_next_record_value()
    }
    fn read_next_record_value(&mut self) -> Result<Option<V>> {
        let vec = &mut Vec::new();
        let read_value: serde_asn1_der::Result<Record<K, V>> =
            serde_asn1_der::from_reader(&mut self.reader, serde_asn1_der::VecBacking(vec));
        match read_value {
            Ok(rec) => Ok(rec.value),
            Err(_) => Err(Error::new(ErrorKind::IoError)),
        }
    }
    /// remove the value stored under the given key or no-op if the key does not exist
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::<String,String>::new(std::path::Path::new("tests.kvsdb")).unwrap();
    /// let _ = store.set("key1".into(),"value1".into());
    /// let value = store.get("key1".into()).unwrap();
    /// assert_eq!(value,Some("value1".into()));
    /// let _ = store.remove("key1".into());
    /// let value = store.get("key1".into()).unwrap();
    /// assert_eq!(value,None);
    /// let _ = store.remove("key2".into());
    /// ```
    pub fn remove(&mut self, key: K) -> Result<()> {
        let rec = Record::<K, V> {
            db_key: self.writer.get_ref().stream_position()?,
            key: key.clone(),
            value: None,
        };
        if serde_asn1_der::to_writer(&rec, &mut self.writer).is_err() {
            self.writer.seek(io::SeekFrom::Start(rec.db_key))?;
            return Err(Error::new(ErrorKind::IoError));
        }
        let _ = self.writer.flush();
        self.index.remove(&key);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
