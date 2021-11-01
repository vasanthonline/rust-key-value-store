use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};
use crate::{Result};

pub struct KvStore {
    path: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>
}

impl KvStore {

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        serde_json::to_writer(&mut self.writer, &cmd)?;
        Ok(())
    }

    pub fn get(&self, key: String) {
        
    }

    pub fn remove(&mut self, key: String) {
        
    }

    pub fn open(&self, path: PathBuf) -> Result<KvStore> {
        let file = File::create(&path)?;
        let reader = BufReader::new(file);

        let file_writer = File::open(&path)?;
        let writer = BufWriter::new(file_writer);

        Ok(KvStore {
            path,
            reader,
            writer
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}