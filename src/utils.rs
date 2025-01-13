use std::path::Path;

pub fn get_cache_file_path(path: &str) -> &Path {
    Path::new(path)
}