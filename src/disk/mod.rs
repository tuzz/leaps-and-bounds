use super::candidate::Candidate;

use std::collections::VecDeque;
use std::fs::{File, create_dir_all, remove_dir_all, remove_file};
use std::io::{BufWriter, BufReader};
use std::sync::{Arc, Mutex};

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use bincode::{serialize_into, deserialize_from};

pub struct Disk {
    path: String,
    gzip: bool,
    index: Arc<Mutex<Vec<Vec<Option<(usize, usize)>>>>>,
}

impl Disk {
    pub fn new(path: String, gzip: bool) -> Self {
        let _ = remove_dir_all(&path);
        create_dir_all(&path).expect(&format!("Failed to create {}", path));

        let index = Arc::new(Mutex::new(vec![]));
        Self { path, gzip, index }
    }

    pub fn read(&self, wasted_symbols: usize, permutations: usize) -> Option<VecDeque<Candidate>> {
        let filename = self.filename_for_reading(wasted_symbols, permutations)?;
        let file = File::open(&filename).expect(&format!("Failed to open {}", filename));

        let mut reader = BufReader::new(file);

        let candidates = if self.gzip {
            let mut decoder = ZlibDecoder::new(reader);
            deserialize_from(&mut decoder).unwrap()
        } else {
            deserialize_from(&mut reader).unwrap()
        };

        remove_file(&filename).expect(&format!("Failed to remove {}", filename));

        Some(candidates)
    }

    pub fn write(&self, bucket: &VecDeque<Candidate>, wasted_symbols: usize, permutations: usize) {
        let filename = self.filename_for_writing(wasted_symbols, permutations);
        let file = File::create(&filename).expect(&format!("Failed to create {}", filename));

        let mut writer = BufWriter::new(file);

        if self.gzip {
            let mut encoder = ZlibEncoder::new(writer, Compression::default());
            serialize_into(&mut encoder, &bucket).unwrap();
        } else {
            serialize_into(&mut writer, &bucket).unwrap();
        }
    }

    pub fn filename_for_reading(&self, wasted_symbols: usize, permutations: usize) -> Option<String> {
        let basename = self.basename(wasted_symbols, permutations);
        let index = self.index_to_read_from(wasted_symbols, permutations)?;

        Some(format!("{}.{}", basename, index))
    }

    pub fn filename_for_writing(&self, wasted_symbols: usize, permutations: usize) -> String {
        let basename = self.basename(wasted_symbols, permutations);
        let index = self.index_to_write_to(wasted_symbols, permutations);

        format!("{}.{}", basename, index)
    }

    pub fn index_to_read_from(&self, wasted_symbols: usize, permutations: usize) -> Option<usize> {
        let mut index_mut = self.index.lock().unwrap();
        let (min, max) = (*index_mut.get(wasted_symbols)?.get(permutations)?)?;

        if min <= max {
            index_mut[wasted_symbols][permutations] = Some((min + 1, max));

            Some(min)
        } else {
            None
        }
    }

    pub fn index_to_write_to(&self, wasted_symbols: usize, permutations: usize) -> usize {
        let mut index = self.index.lock().unwrap();
        if index.len() <= wasted_symbols {
            index.resize(wasted_symbols + 1, vec![]);
        }

        let nested = &mut index[wasted_symbols];
        if nested.len() <= permutations {
            nested.resize(permutations + 1, None);
        }

        let tuple = match (*nested)[permutations] {
            None => (0, 0),
            Some((min, max)) => (min, max + 1),
        };

        (*nested)[permutations] = Some(tuple);
        tuple.1
    }

    pub fn basename(&self, wasted_symbols: usize, permutations: usize) -> String {
        let gzip_component = match self.gzip {
            true => ".gz",
            false => "",
        };

        format!(
            "{}/candidates-with-{}-wasted-symbols-and-{}-permutations.dat{}",
            self.path,
            wasted_symbols,
            permutations,
            gzip_component,
        )
    }
}

#[cfg(test)]
mod test;
