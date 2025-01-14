use crate::errors::CustomError;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, Read};
use serde::{Serialize, Deserialize};
use std::path::Path;
use serde::de::Error;

/// Cache qui stock les données dans un fichier.
///
/// # Exemples
///
/// ```
/// use eval_rust::CacheDB;
/// use eval_rust::errors::CustomError;
///
/// # fn main() -> Result<(), CustomError> {
/// // Crée un nouveau cache avec une capacité de 5 éléments et le chemin du fichier "cache.txt".
/// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
///
/// // Ajouter des éléments au cache.
/// cache.put("pomme".to_string(), 1)?;
/// cache.put("banane".to_string(), 2)?;
/// cache.put("orange".to_string(), 3)?;
///
/// // Récupre la valeur associée à une clé.
/// if let Some(valeur) = cache.get(&"pomme".to_string()) {
///     println!("La valeur de pomme est: {}", valeur);
/// }
///
/// // Supprime un élément du cache.
/// cache.remove(&"banane".to_string())?;
///
/// // Itère sur les éléments du cache
/// for (key, value) in cache.iter() {
///     println!("Clé: {}, Valeur: {}", key, value);
/// }
///
/// // Vide le cache.
/// cache.clear()?;
/// # Ok(())
/// # }
/// ```
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
    /// Crée un nouveau cache avec la capacité et le chemin du fichier spécifiés.
    ///
    /// Si le fichier existe, le cache est chargé à partir de celui-ci. Sinon, un nouveau cache vide est créé.
    ///
    /// # Arguments
    ///
    /// * `capacity` - La capacité maximale du cache.
    /// * `file_path` - Le chemin du fichier où le cache sera stocké.
    ///
    /// # Retour
    ///
    /// Retourne un `Result` ou une erreur `CustomError` si une erreur s'est produite.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let cache = CacheDB::<String, String>::new_persistent(10, "cache.txt")?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Sauvegarde le cache dans le fichier.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si le cache a été sauvegardé avec succès, ou une erreur `CustomError` si une erreur s'est produite.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.put("pomme".to_string(), 1)?;
    /// cache.save()?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Charge le cache à partir du fichier.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si le cache a été chargé avec succès, ou une erreur `CustomError` si une erreur s'est produite.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.load()?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Insère une paire clé-valeur dans le cache.
    ///
    /// Si la clé existe déjà, la valeur associée est mise à jour.
    /// Si le cache est plein, l'élément le moins récemment utilisé est supprimé.
    ///
    /// # Arguments
    ///
    /// * `key` - La clé à insérer.
    /// * `value` - La valeur à associer à la clé.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si l'insertion a réussi, ou une erreur `CustomError` si une erreur s'est produite.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.put("pomme".to_string(), 1)?;
    /// cache.put("banane".to_string(), 2)?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Récupère la valeur associée à une clé dans le cache.
    ///
    /// Si la clé est trouvée, la valeur correspondante est retournée et l'élément est marqué comme récemment utilisé.
    ///
    /// # Arguments
    ///
    /// * `key` - La clé à rechercher.
    ///
    /// # Retour
    ///
    /// Retourne `Some(&V)` contenant une référence à la valeur associée à la clé si elle est trouvée, ou `None` si la clé n'est pas dans le cache.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.put("pomme".to_string(), 1)?;
    ///
    /// if let Some(valeur) = cache.get(&"pomme".to_string()) {
    ///     println!("La valeur de pomme est: {}", valeur);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(index) = self.cache.iter().position(|(k, _)| k == key) {
            let element = self.cache.remove(index);
            self.cache.push(element);
            return Some(&self.cache.last().unwrap().1);
        }
        None
    }

    /// Supprime l'élément associé à une clé du cache.
    ///
    /// # Arguments
    ///
    /// * `key` - La clé à supprimer.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si la clé a été trouvée et supprimée, ou une erreur `CustomError` si une erreur s'est produite.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.put("pomme".to_string(), 1)?;
    /// cache.remove(&"pomme".to_string())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove(&mut self, key: &K) -> Result<(), CustomError> {
        if let Some(index) = self.cache.iter().position(|(k, _)| k == key){
            self.cache.remove(index);
            self.save()?;
            Ok(())
        } else {
            Err(CustomError::NotFound)
        }
    }

    /// Vide le cache.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si le cache a été vidé avec succès, ou une erreur `CustomError` si une erreur s'est produite.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.put("pomme".to_string(), 1)?;
    /// cache.clear()?;
    /// assert_eq!(cache.len(), 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear(&mut self) -> Result<(), CustomError> {
        self.cache.clear();
        self.save()
    }

    /// Retourne un itérateur sur les éléments du cache.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.put("pomme".to_string(), 1)?;
    /// cache.put("banane".to_string(), 2)?;
    ///
    /// for (key, value) in cache.iter() {
    ///     println!("Clé: {}, Valeur: {}", key, value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn iter(&self) -> std::slice::Iter<(K, V)> {
        self.cache.iter()
    }

    /// Retourne le nombre d'éléments dans le cache.
    ///
    /// # Exemples
    ///
    /// ```
    /// use eval_rust::CacheDB;
    /// use eval_rust::errors::CustomError;
    ///
    /// # fn main() -> Result<(), CustomError> {
    /// let mut cache = CacheDB::<String, i32>::new_persistent(5, "cache.txt")?;
    /// cache.put("pomme".to_string(), 1)?;
    /// assert_eq!(cache.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.cache.len()
    }
}