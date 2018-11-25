use super::candidate::Candidate;

use std::collections::VecDeque;
use std::fs::{File, create_dir_all, remove_dir_all, remove_file};
use std::io::{BufWriter, BufReader};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use bincode::{serialize_into, deserialize_from};

struct Disk {
    path: String,
    gzip: bool,
    index: Vec<Vec<(usize, usize)>>,
}

impl Disk {
    pub fn new(path: String, gzip: bool) -> Self {
        let _ = remove_dir_all(&path);
        create_dir_all(&path).expect(&format!("Failed to create {}", path));

        Self { path, gzip, index: vec![] }
    }

    pub fn read(&mut self, wasted_symbols: usize, permutations: usize) -> Option<VecDeque<Candidate>> {
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

    pub fn write(&mut self, bucket: &VecDeque<Candidate>, wasted_symbols: usize, permutations: usize) {
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

    pub fn filename_for_reading(&mut self, wasted_symbols: usize, permutations: usize) -> Option<String> {
        let basename = self.basename(wasted_symbols, permutations);
        let index = self.index_to_read_from(wasted_symbols, permutations)?;

        Some(format!("{}.{}", basename, index))
    }

    pub fn filename_for_writing(&mut self, wasted_symbols: usize, permutations: usize) -> String {
        let basename = self.basename(wasted_symbols, permutations);
        let index = self.index_to_write_to(wasted_symbols, permutations);

        format!("{}.{}", basename, index)
    }

    pub fn index_to_read_from(&mut self, wasted_symbols: usize, permutations: usize) -> Option<usize> {
        let index = self.index.get(wasted_symbols)?.get(permutations)?;

        if index.0 <= index.1 {
            let current = self.index[wasted_symbols][permutations].0;
            self.index[wasted_symbols][permutations].0 = current + 1;

            Some(current)
        } else {
            None
        }
    }

    pub fn index_to_write_to(&mut self, wasted_symbols: usize, permutations: usize) -> usize {
        if self.index.len() < wasted_symbols {
            self.index.resize(wasted_symbols + 1, vec![]);
        }

        let nested = &mut self.index[wasted_symbols];

        if nested.len() <= permutations {
            nested.resize(permutations + 1, (0, 0));
            0
        } else {
            (*nested)[permutations].1 += 1;
            (*nested)[permutations].1
        }
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
