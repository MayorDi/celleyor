use std::{io::Read, path::PathBuf};

pub fn load_bytes_from_file(path: &str) -> std::io::Result<Vec<u8>> {
    let path = PathBuf::from(path);
    let mut file = std::fs::File::open(path)?;

    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    Ok(buf)
}
