use crate::errors::CustomError;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, Read};
use serde::{Serialize, Deserialize};
use std::path::Path;
use serde::de::Error;

pub struct CacheDB<K, V>
where
    K: Eq + Clone + ToString + Serialize + for<'de> Deserialize<'de>,
    V: Clone + ToString + Serialize + for<'de> Deserialize<'de>,
{
    cache: Vec<(K, V)>,
    capacity: usize,
    file_path: String,
}

impl<K, V> CacheDB<K, V>
where
    K: Eq + Clone + ToString + Serialize + for<'de> Deserialize<'de>,
    V: Clone + ToString + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new_persistent(capacity: usize, file_path: &str) -> Result<Self, CustomError> {
        let file_path_clone = file_path.to_string();
        let cache = Vec::with_capacity(capacity);

        let mut persistent_cache = CacheDB {
            cache,
            capacity,
            file_path: file_path.to_string(),
        };

        if Path::new(&file_path_clone).exists() {
            persistent_cache.load()?;
        }

        Ok(persistent_cache)
    }

    pub fn save(&self) -> Result<(), CustomError> {
        let file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
        {
            Ok(file) => file,
            Err(_) => return Err(CustomError::CacheDbSaveError),
        };

        let mut writer = BufWriter::new(file);

        for (key, value) in self.cache.iter() {
            let key_json = match serde_json::to_string(key) {
                Ok(json) => json,
                Err(_) => return Err(CustomError::SerializationError(serde_json::Error::custom(
                    "Failed to serialize key to JSON",
                ))),
            };
            let value_json = match serde_json::to_string(value) {
                Ok(json) => json,
                Err(_) => return Err(CustomError::SerializationError(serde_json::Error::custom(
                    "Failed to serialize value to JSON",
                ))),
            };
            let line = format!("{}={}\n", key_json, value_json);
            if let Err(_) = writer.write_all(line.as_bytes()) {
                return Err(CustomError::CacheDbSaveError);
            }
        }

        if let Err(_) = writer.flush() {
            return Err(CustomError::CacheDbSaveError);
        }

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), CustomError> {
        let file = match File::open(&self.file_path) {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(_) => return Err(CustomError::CacheDbLoadError),
        };

        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        if let Err(_) = reader.read_to_string(&mut contents) {
            return Err(CustomError::CacheDbLoadError);
        }

        self.cache.clear();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let key: K = match serde_json::from_str(parts[0]) {
                    Ok(key) => key,
                    Err(_) => return Err(CustomError::CacheDbLoadError),
                };
                let value: V = match serde_json::from_str(parts[1]) {
                    Ok(value) => value,
                    Err(_) => return Err(CustomError::CacheDbLoadError),
                };
                if let Err(e) = self.put(key, value) {
                    return Err(e)
                }
            } else {
                return Err(CustomError::CacheDbLoadError);
            }
        }

        Ok(())
    }

    pub fn put(&mut self, key: K, value: V) -> Result<(), CustomError> {
        if let Some(index) = self.cache.iter().position(|(k, _)| k == &key) {
            if let Some((_, val)) = self.cache.get_mut(index) {
                *val = value;
            }
            let element = self.cache.remove(index);
            self.cache.push(element);
            return self.save();
        }

        if self.cache.len() >= self.capacity {
            self.cache.remove(0);
        }

        self.cache.push((key, value));
        self.save()
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(index) = self.cache.iter().position(|(k, _)| k == key) {
            let element = self.cache.remove(index);
            self.cache.push(element);
            return Some(&self.cache.last().unwrap().1);
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Result<(), CustomError> {
        if let Some(index) = self.cache.iter().position(|(k, _)| k == key){
            self.cache.remove(index);
            self.save()?;
            Ok(())
        } else {
            Err(CustomError::NotFound)
        }
    }

    pub fn clear(&mut self) -> Result<(), CustomError> {
        self.cache.clear();
        self.save()
    }

    pub fn iter(&self) -> std::slice::Iter<(K, V)> {
        self.cache.iter()
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }
}