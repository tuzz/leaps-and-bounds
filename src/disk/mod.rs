use super::candidate::Candidate;

use std::collections::VecDeque;
use std::path::Path;
use std::fs::{File, create_dir_all, remove_dir_all, remove_file};
use std::io::{BufWriter, BufReader};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use bincode::{serialize_into, deserialize_from};
use glob::glob;

struct Disk {
    path: String,
    gzip: bool,
}

impl Disk {
    pub fn new(path: String, gzip: bool) -> Self {
        let _ = remove_dir_all(&path);
        create_dir_all(&path).expect(&format!("Failed to create {}", path));

        Self { path, gzip }
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
        let filename = self.basename(wasted_symbols, permutations);
        let indexes = self.file_indexes(&filename);
        let first_index = indexes.iter().min()?;

        Some(format!("{}.{}", filename, first_index))
    }

    pub fn filename_for_writing(&self, wasted_symbols: usize, permutations: usize) -> String {
        let basename = self.basename(wasted_symbols, permutations);
        let indexes = self.file_indexes(&basename);
        let next_index = indexes.iter().max().map_or(0, |i| i + 1);

        format!("{}.{}", basename, next_index)
    }

    fn file_indexes(&self, basename: &String) -> Vec<usize> {
        let pattern = format!("{}*", basename);
        let glob = glob(&pattern).expect(&format!("Failed to glob {}", pattern));

        glob
            .map(|result| result.unwrap())
            .map(|pathbuf| pathbuf.into_os_string())
            .map(|osstr| osstr.into_string().unwrap())
            .map(|string| string.rsplit(".").map(|s| s.to_string()).collect())
            .map(|splits: Vec<String>| splits.first().cloned().unwrap())
            .map(|suffix| suffix.parse::<usize>().unwrap())
            .collect()
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
