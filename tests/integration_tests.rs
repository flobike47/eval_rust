use eval_rust::CacheDB;
use eval_rust::CustomError;
use std::fs;

#[test]
fn test_cache_put_get_remove() {
    let file_path = "test_cache_put_get_remove.txt";
    let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");

    // Test put et get avec des clés String
    assert!(cache.put("1".to_string(), "un".to_string()).is_ok());
    assert!(cache.put("2".to_string(), "deux".to_string()).is_ok());
    assert_eq!(cache.get(&"1".to_string()), Some(&"un".to_string()));
    assert_eq!(cache.get(&"2".to_string()), Some(&"deux".to_string()));

    // Test put et get avec des clés String
    assert!(cache.put("pomme".to_string(), "rouge".to_string()).is_ok());
    assert_eq!(cache.get(&"pomme".to_string()), Some(&"rouge".to_string()));

    // Test remove
    assert!(cache.remove(&"1".to_string()).is_ok());
    assert_eq!(cache.get(&"1".to_string()), None);

    // Test put avec remplacement
    assert!(cache.put("2".to_string(), "deux-bis".to_string()).is_ok());
    assert_eq!(cache.get(&"2".to_string()), Some(&"deux-bis".to_string()));

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_cache_capacity() {
    let file_path = "test_cache_capacity.txt";
    let mut cache: CacheDB<String, String> = CacheDB::new_persistent(2, file_path).expect("Erreur lors de la création du cache");

    // Test de la capacité avec des clés String
    assert!(cache.put("1".to_string(), "un".to_string()).is_ok());
    assert!(cache.put("2".to_string(), "deux".to_string()).is_ok());
    assert!(cache.put("3".to_string(), "trois".to_string()).is_ok()); // Devrait enlever l'élément le plus ancien (1)

    assert_eq!(cache.get(&"1".to_string()), None); // 1 a été enlevé
    assert_eq!(cache.get(&"2".to_string()), Some(&"deux".to_string()));
    assert_eq!(cache.get(&"3".to_string()), Some(&"trois".to_string()));

    // Test de la capacité avec des clés String
    assert!(cache.put("a".to_string(), "apple".to_string()).is_ok());
    assert!(cache.put("b".to_string(), "banana".to_string()).is_ok());
    assert!(cache.put("c".to_string(), "cherry".to_string()).is_ok()); // Devrait enlever l'élément le plus ancien ("a")

    assert_eq!(cache.get(&"a".to_string()), None); // "a" a été enlevé
    assert_eq!(cache.get(&"b".to_string()), Some(&"banana".to_string()));
    assert_eq!(cache.get(&"c".to_string()), Some(&"cherry".to_string()));

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_cache_persistence() {
    let file_path = "test_cache_persistence.txt";

    // Crée et remplit le cache
    {
        let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");
        assert!(cache.put("1".to_string(), "un".to_string()).is_ok());
        assert!(cache.put("pomme".to_string(), "rouge".to_string()).is_ok());
    }

    // Recharge le cache à partir du fichier
    {
        let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");

        // Vérifie que les données sont bien chargées
        assert_eq!(cache.get(&"1".to_string()), Some(&"un".to_string()));
        assert_eq!(cache.get(&"pomme".to_string()), Some(&"rouge".to_string()));

        // Ajoute un nouvel élément
        assert!(cache.put("2".to_string(), "deux".to_string()).is_ok());
    }

    // Recharge le cache à nouveau
    {
        let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");

        // Vérifie que le nouvel élément est bien présent et que l'ancien est toujour la
        assert_eq!(cache.get(&"1".to_string()), Some(&"un".to_string()));
        assert_eq!(cache.get(&"pomme".to_string()), Some(&"rouge".to_string()));
        assert_eq!(cache.get(&"2".to_string()), Some(&"deux".to_string()));
    }

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_cache_clear() {
    let file_path = "test_cache_clear.txt";
    let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");

    assert!(cache.put("1".to_string(), "un".to_string()).is_ok());
    assert!(cache.put("pomme".to_string(), "rouge".to_string()).is_ok());

    assert!(cache.clear().is_ok());
    assert_eq!(cache.len(), 0);

    // Vérifie que le cache est vide après le chargement
    let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");
    assert_eq!(cache.len(), 0);

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_cache_empty_file() {
    let file_path = "test_cache_empty_file.txt";

    // Crée un fichier vide
    fs::write(file_path, "").unwrap();

    // Vérifie que le chargement d'un fichier vide ne pose pas de problème
    let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");
    assert_eq!(cache.len(), 0);

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_cache_invalid_file() {
    let file_path = "test_cache_invalid_file.txt";

    fs::write(file_path, "invalid data").unwrap();

    let result = CacheDB::<String, String>::new_persistent(3, file_path); // On crée un nouveau cache
    assert!(matches!(result, Err(CustomError::CacheDbLoadError)));

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_cache_iter() {
    let file_path = "test_cache_iter.txt";
    let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");

    assert!(cache.put("1".to_string(), "un".to_string()).is_ok());
    assert!(cache.put("pomme".to_string(), "rouge".to_string()).is_ok());

    let mut count = 0;
    for (key, value) in cache.iter() {
        if *key == String::from("1") {
            assert_eq!(*value, String::from("un"));
        } else if *key == String::from("pomme") {
            assert_eq!(*value, String::from("rouge"));
        } else {
            panic!("Clé inattendue: {}", key);
        }
        count += 1;
    }
    assert_eq!(count, 2);

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_cache_len() {
    let file_path = "test_cache_len.txt";
    let mut cache: CacheDB<String, String> = CacheDB::new_persistent(3, file_path).expect("Erreur lors de la création du cache");

    assert_eq!(cache.len(), 0);
    assert!(cache.put("1".to_string(), "un".to_string()).is_ok());
    assert_eq!(cache.len(), 1);
    assert!(cache.put("pomme".to_string(), "rouge".to_string()).is_ok());
    assert_eq!(cache.len(), 2);
    assert!(cache.remove(&"1".to_string()).is_ok());
    assert_eq!(cache.len(), 1);
    assert!(cache.clear().is_ok());
    assert_eq!(cache.len(), 0);

    fs::remove_file(file_path).unwrap();
}