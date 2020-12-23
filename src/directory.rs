use blake3::Hash;
use std::path::PathBuf;
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
