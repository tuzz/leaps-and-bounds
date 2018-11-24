use super::candidate::Candidate;

use std::collections::VecDeque;
use std::path::Path;
use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::{BufWriter, BufReader};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use bincode::{serialize_into, deserialize_from};

struct Disk {
    path: String,
    gzip: bool,
}

impl Disk {
    pub fn new(path: String, gzip: bool) -> Self {
        remove_dir_all(&path).expect(&format!("Failed to remove {}", path));
        create_dir_all(&path).expect(&format!("Failed to create {}", path));

        Self { path, gzip }
    }

    pub fn read(&self, wasted_symbols: usize, permutations: usize) -> VecDeque<Candidate> {
        let filename = self.filename(wasted_symbols, permutations);
        let file = File::open(&filename).expect(&format!("Failed to open {}", filename));

        let mut reader = BufReader::new(file);

        if self.gzip {
            let mut decoder = ZlibDecoder::new(reader);
            deserialize_from(&mut decoder).unwrap()
        } else {
            deserialize_from(&mut reader).unwrap()
        }
    }

    pub fn write(&self, bucket: &VecDeque<Candidate>, wasted_symbols: usize, permutations: usize) {
        let filename = self.filename(wasted_symbols, permutations);
        let file = File::create(&filename).expect(&format!("Failed to create {}", filename));

        let mut writer = BufWriter::new(file);

        if self.gzip {
            let mut encoder = ZlibEncoder::new(writer, Compression::default());
            serialize_into(&mut encoder, &bucket).unwrap();
        } else {
            serialize_into(&mut writer, &bucket).unwrap();
        }
    }

    pub fn filename(&self, wasted_symbols: usize, permutations: usize) -> String {
        format!(
            "{}/candidates-with-{}-wasted-symbols-and-{}-permutations",
            self.path,
            wasted_symbols,
            permutations
        )
    }
}

#[cfg(test)]
mod test;
