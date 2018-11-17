use super::*;

struct Candidate {
    permutations_seen: BitSet,
    tail_of_string: Vec<usize>,
    wasted_symbols: usize,
}

impl Candidate {
    fn seed(n: usize) -> Self {
        let max_value = Lehmer::max_value(n) as usize;
        let mut seen = BitSet::with_capacity(max_value);

        seen.insert(0);

        Candidate {
            permutations_seen: seen,
            tail_of_string: (1..n).collect(),
            wasted_symbols: 0,
        }
    }
}

#[cfg(test)]
mod test;
