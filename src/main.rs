use blake3::{Hash, Hasher};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tempfile::NamedTempFile;

fn main() {
    let stdin = io::stdin();
    let mut temp_file = NamedTempFile::new().unwrap();
    let start = Instant::now();
    let prefix = Path::new("./tmp");

    //calculate the hash while also streaming STDIN to a temporary file
    let hash = {
        let mut locked = stdin.lock();

        let hasher = process_stream(&mut locked, &mut temp_file);

        hasher.finalize()
    };
    //flush writes to disk
    temp_file.flush().unwrap();

    //determine where the temporary file should ultimately live
    let directory_name = get_directory_from_hash(hash);

    let directory_path = prefix.join(directory_name);
    fs::create_dir(directory_path.clone()).unwrap();

    let mut file_path = directory_path.clone();
    file_path.push(hash.to_hex().to_string());

    //rename the temporary file

    fs::rename(temp_file.path(), file_path.clone()).unwrap();

    log_info(start, hash);
}

fn log_info(start: Instant, hash: Hash) {
    println!("time elapsed in {:?}", start.elapsed());
    println!("hash value is: {:?}", hash);
}
fn process_stream<T: Read>(read: &mut T, temp_file: &mut NamedTempFile) -> Hasher {
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = read.read(&mut buffer[..]).unwrap();
        if bytes_read > 0 {
            hasher.update(&buffer[..bytes_read]);
            temp_file.write(&buffer[..bytes_read]).unwrap();
        } else {
            break;
        }
    }
    return hasher;
}

fn get_directory_from_hash(hash: Hash) -> PathBuf {
    let first_chars = hash.to_hex().to_string();
    let str: String = first_chars.chars().take(2).collect();
    let mut path = PathBuf::new();
    path.push(str);
    path
}
