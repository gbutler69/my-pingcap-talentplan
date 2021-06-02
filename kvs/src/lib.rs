#![deny(missing_docs)]

//! kvs - a simple key-value store
//!
//! # Example
//! ```
//! use kvs::KvStore;
//!
//! let mut store = KvStore::<String, String>::new(std::path::Path::new("testdb")).unwrap();
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
    marker, mem,
    path::{self, Path},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod error;
pub use error::{Error, ErrorKind, Result};

/// Simple Key-Value Storage Type
pub struct KvStore<K, V> {
    index: HashMap<K, u64>,
    stale_count: u64,
    file_path: path::PathBuf,
    reader: io::BufReader<fs::File>,
    writer: io::BufWriter<fs::File>,
    stale_fraction_for_compaction: f64,
    min_records_before_compaction: u64,
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
    /// let store = KvStore::<String,String>::new(std::path::Path::new("testdb")).unwrap();
    /// ```
    pub fn new(path: &Path) -> Result<Self> {
        ensure_dir_exists(path);
        let db_path = use_existing_or_create_new_db_log_path(path)?;
        Self::init_self(&db_path, true)
    }
    /// open a disk-based, log-based storage at a path
    /// If the file exists it opens for reading and appending. If the file does not exist it creates it.
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let store = KvStore::<String,String>::open(std::path::Path::new("testdb")).unwrap();
    /// ```
    pub fn open(path: &path::Path) -> Result<Self> {
        ensure_dir_exists(path);
        let db_path = use_existing_or_create_new_db_log_path(path)?;
        let mut kv_store = Self::init_self(&db_path, false)?;
        kv_store.load_index()?;
        Ok(kv_store)
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
    /// let mut store = KvStore::<String,String>::new(std::path::Path::new("testdb")).unwrap();
    /// let _ = store.set("key1".into(),"value1".into());
    /// let _ = store.set("key1".into(),"value2".into());
    /// let value = store.get("key1".into()).unwrap();
    /// assert_eq!(value,Some("value2".into()));
    /// ```
    pub fn set(&mut self, key: K, value: V) -> Result<()> {
        let rec = self.build_output_record(&key, Some(value))?;
        let db_key = rec.db_key;
        self.write_record_to_db(rec)?;
        if self.index.insert(key, db_key).is_some() {
            self.stale_count += 1;
        };
        self.compact_if_stale_threshold_reached()?;
        Ok(())
    }
    /// get the value stored under the given key or None if no such key
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::<String,String>::new(std::path::Path::new("testdb")).unwrap();
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
    /// remove the value stored under the given key or no-op if the key does not exist
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::<String,String>::new(std::path::Path::new("testdb")).unwrap();
    /// let _ = store.set("key1".into(),"value1".into());
    /// let value = store.get("key1".into()).unwrap();
    /// assert_eq!(value,Some("value1".into()));
    /// let _ = store.remove("key1".into());
    /// let value = store.get("key1".into()).unwrap();
    /// assert_eq!(value,None);
    /// let _ = store.remove("key2".into());
    /// ```
    pub fn remove(&mut self, key: K) -> Result<()> {
        match self.index.contains_key(&key) {
            true => {
                let rec = self.build_output_record(&key, None)?;
                self.write_record_to_db(rec)?;
                self.index.remove(&key);
                self.stale_count += 1;
                self.compact_if_stale_threshold_reached()?;
                Ok(())
            }
            false => Err(Error::new(ErrorKind::KeyNotPresent)),
        }
    }

    fn init_self(db_path: &path::Path, do_truncate_on_open: bool) -> Result<Self> {
        let (reader, writer) = open_db_reader_and_writer(db_path, do_truncate_on_open)?;
        Ok(Self {
            index: HashMap::new(),
            stale_count: 0,
            file_path: db_path.to_owned(),
            reader,
            writer,
            stale_fraction_for_compaction: 0.25,
            min_records_before_compaction: 100,
            phantom_value: marker::PhantomData::default(),
        })
    }
    fn load_index(&mut self) -> Result<()> {
        while let Some(rec) = self.read_next_record()? {
            match rec {
                Record {
                    db_key,
                    key,
                    value: Some(_),
                } => {
                    if self.index.insert(key, db_key).is_some() {
                        self.stale_count += 1;
                    }
                }
                Record {
                    key, value: None, ..
                } => {
                    self.index.remove(&key);
                    self.stale_count += 1;
                }
            };
        }
        Ok(())
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
    fn read_next_record_value(&mut self) -> Result<Option<V>> {
        match self.read_next_record() {
            Ok(Some(rec)) => Ok(rec.value),
            _ => Err(Error::new(ErrorKind::IoError)),
        }
    }
    fn build_output_record(&mut self, key: &K, value: Option<V>) -> Result<Record<K, V>> {
        Ok(Record {
            db_key: self.writer.get_ref().stream_position()?,
            key: key.clone(),
            value,
        })
    }
    fn write_record_to_db(&mut self, rec: Record<K, V>) -> Result<()> {
        let writer = &mut self.writer;
        write_record_to_writer(rec, writer)
    }
    fn compact_if_stale_threshold_reached(&mut self) -> Result<()> {
        if self.index.len() as u64 >= self.min_records_before_compaction
            && self.stale_count as f64 / self.index.len() as f64
                >= self.stale_fraction_for_compaction
        {
            self.compact()?;
        }
        assert!(
            self.index.len() < usize::MAX && (self.index.len() as u64) < u64::MAX,
            "Maximum Database size reached - unable to continue"
        );
        Ok(())
    }
    fn compact(&mut self) -> Result<()> {
        let compact_path = make_next_db_log_path(self.file_path.clone());
        match self.copy_active_records_to_compaction_file_and_update_indexes(compact_path.clone()) {
            Err(err) => {
                self.remove_file(&compact_path)?;
                return Err(err);
            }
            Ok((_, _, _, orig_path)) => {
                self.finalize_compacted_filename()?;
                self.remove_file(&orig_path)?;
                self.stale_count = 0;
            }
        }
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn copy_active_records_to_compaction_file_and_update_indexes(
        &mut self,
        compact_file_path: path::PathBuf,
    ) -> Result<(
        io::BufReader<fs::File>,
        io::BufWriter<fs::File>,
        HashMap<K, u64>,
        path::PathBuf,
    )> {
        let (compacted_reader, mut compacted_writer) =
            open_db_reader_and_writer(&compact_file_path, true)?;
        let mut compacted_index = HashMap::new();
        self.reader.seek(io::SeekFrom::Start(0))?;
        while let Some(mut rec) = self.read_next_record()? {
            match self.index.get(&rec.key) {
                Some(current_db_key) if *current_db_key == rec.db_key => {
                    let (key, db_key) = (rec.key.clone(), compacted_writer.stream_position()?);
                    rec.db_key = db_key;
                    write_record_to_writer(rec, &mut compacted_writer)?;
                    compacted_index.insert(key, db_key);
                }
                _ => (),
            }
        }
        Ok(self.replace_reader_writer_index_file(
            compacted_reader,
            compacted_writer,
            compacted_index,
            compact_file_path,
        ))
    }
    #[allow(clippy::type_complexity)]
    fn replace_reader_writer_index_file(
        &mut self,
        mut reader: io::BufReader<fs::File>,
        mut writer: io::BufWriter<fs::File>,
        mut index: HashMap<K, u64>,
        mut file_path: path::PathBuf,
    ) -> (
        io::BufReader<fs::File>,
        io::BufWriter<fs::File>,
        HashMap<K, u64>,
        path::PathBuf,
    ) {
        mem::swap(&mut reader, &mut self.reader);
        mem::swap(&mut writer, &mut self.writer);
        mem::swap(&mut index, &mut self.index);
        mem::swap(&mut file_path, &mut self.file_path);
        (reader, writer, index, file_path)
    }
    fn remove_file(&self, compacted_path: &path::Path) -> Result<()> {
        fs::remove_file(compacted_path)?;
        Ok(())
    }
    fn finalize_compacted_filename(&mut self) -> Result<()> {
        let final_path = self.file_path.with_extension("log");
        fs::rename(&self.file_path, &final_path)?;
        self.file_path = final_path;
        Ok(())
    }
}

fn ensure_dir_exists(path: &Path) {
    if !path.exists() {
        let _ = fs::create_dir(path);
    }
    assert!(path.is_dir());
}
fn use_existing_or_create_new_db_log_path(path: &Path) -> Result<path::PathBuf> {
    let db_path = match latest_log_for_dir(path) {
        Ok(Some(path)) => path,
        Ok(None) => make_db_log_path(path),
        Err(err) => return Err(err),
    };
    Ok(db_path)
}
fn latest_log_for_dir(path: &path::Path) -> Result<Option<path::PathBuf>> {
    let mut max_modified = None;
    let mut existing_path = None;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if let (true, Some(filestem), Some(extension)) =
            (path.is_file(), path.file_stem(), path.extension())
        {
            if let (Some(filestem), Some(extension)) = (filestem.to_str(), extension.to_str()) {
                if filestem.starts_with("kvsdb-") && filestem.len() == 38 && extension == "log" {
                    let last_modified = entry.metadata()?.modified()?;
                    if max_modified.is_none() || last_modified > max_modified.unwrap() {
                        max_modified = Some(last_modified);
                        existing_path = Some(path);
                    }
                }
            }
        }
    }
    Ok(existing_path)
}
fn make_db_log_path(path: &Path) -> path::PathBuf {
    let uuid = uuid::Uuid::new_v4().to_simple();
    path.join(path::Path::new(&format!("kvsdb-{}.log", uuid)))
}
fn make_next_db_log_path(mut existing_path: path::PathBuf) -> path::PathBuf {
    existing_path.pop();
    make_db_log_path(&existing_path).with_extension("compact")
}
fn open_db_reader_and_writer(
    db_path: &path::Path,
    truncate: bool,
) -> Result<(io::BufReader<fs::File>, io::BufWriter<fs::File>)> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(truncate)
        .open(db_path)?;
    if !truncate {
        file.seek(io::SeekFrom::End(0))?;
    }
    Ok((
        io::BufReader::new(fs::OpenOptions::new().read(true).open(db_path)?),
        io::BufWriter::new(file),
    ))
}
fn write_record_to_writer<K, V>(
    rec: Record<K, V>,
    mut writer: &mut io::BufWriter<fs::File>,
) -> Result<()>
where
    K: Serialize + DeserializeOwned + Eq + PartialEq + hash::Hash + Clone,
    V: Serialize + DeserializeOwned + Clone,
{
    if serde_asn1_der::to_writer(&rec, &mut writer).is_err() {
        writer.seek(io::SeekFrom::Start(rec.db_key))?;
        writer.get_mut().set_len(rec.db_key)?;
        return Err(Error::new(ErrorKind::IoError));
    }
    Ok(writer.flush()?)
}

#[cfg(test)]
mod tests;
