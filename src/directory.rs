use blake3::Hash;
use std::path::PathBuf;
pub fn get_directory_from_hash(hash: Hash) -> PathBuf {
    let first_chars = hash.to_hex().to_string();
    let str: String = first_chars.chars().take(2).collect();
    let mut path = PathBuf::new();
    path.push(str);
    path
}
