use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::fs::File as FsFile;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::Metadata;

use rocket::Data;
use crypto::digest::Digest;
use crypto::sha1::Sha1;

use errors::*;

fn hash_and_write_file<R, W>(mut reader: R, mut writer: W) -> Result<String> where R: Read, W: Write {
    const BUFF_SIZE: usize = 4096;
    let mut buffer = [0; BUFF_SIZE];
    let mut hasher = Sha1::new();
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(readed) => {
                hasher.input(&buffer[0..readed]);
                writer.write(&buffer[0..readed])?;
            },
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {},
            Err(err) => return Err(err.into())
        }
    }

    writer.flush()?;
    let hash = hasher.result_str();
    Ok(hash)
}

pub fn save_file(data: Data, path: PathBuf) -> Result<(String, Metadata)> {
    let destination_path = Path::new("upload/").join(path.clone());
    create_file_parent_dir(destination_path.clone())?;
    let destination_file = FsFile::create(destination_path.clone())?;
    let writer = BufWriter::new(destination_file);
    let hash = hash_and_write_file(data.open(), writer)?;
    let destination_file = FsFile::open(destination_path.clone())?;
    destination_file.sync_all()?;
    let metadata = destination_file.metadata()?;
    Ok((hash, metadata))
}

fn create_file_parent_dir(path: PathBuf) -> Result<()> {
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}
