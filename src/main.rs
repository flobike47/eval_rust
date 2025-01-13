mod errors;
mod persistent;
mod utils;

use persistent::CacheDB;
use crate::errors::CustomError;

fn main() -> Result<(), CustomError>{

    let mut cache = match CacheDB::new_persistent(3, "cache.txt") {
        Ok(cache) => cache,
        Err(e) => return Err(e),
    };

    if let Err(e) = cache.put("1".to_string(), "rouge".to_string()) {
        return Err(e);
    }
    if let Err(e) = cache.put("banane".to_string(), "jaune".to_string()) {
        return Err(e);
    }

    println!("Valeur pour 1: {:?}", cache.get(&"1".to_string()));
    println!("Valeur pour banane: {:?}", cache.get(&"banane".to_string()));

    if let Err(e) = cache.remove(&"1".to_string()) {
        return Err(e);
    }

    for (key, value) in cache.iter() {
        println!("{} = {}", key, value);
    }


    Ok(())
}