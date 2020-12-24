use blake3::Hash;
use std::path::{Path, PathBuf};
pub fn get_directory_from_hash(hash: Hash) -> PathBuf {
    let first_chars = hash.to_hex().to_string();
    get_directory_from_string(first_chars)
}

pub fn get_directory_from_string(string: String) -> PathBuf {
    let s: String = string.chars().take(2).collect();
    let mut path = PathBuf::new();
    path.push(s);
    path
}

pub fn get_file_path(storage_path: &str, hash: String) -> PathBuf {

        let directory_name = get_directory_from_string(hash.clone());
        let prefix = Path::new(storage_path);
        let directory_path = prefix.join(directory_name);

        let mut file_path = directory_path;
        file_path.push(hash);
        file_path
}
pub fn get_file_path_from_hash(storage_path: &str, hash: Hash) -> PathBuf {
        let directory_name = get_directory_from_hash(hash);

        let prefix = Path::new(storage_path);
        let directory_path = prefix.join(directory_name);

        let mut file_path = directory_path;
        file_path.push(hash.to_hex().to_string());
        file_path
}