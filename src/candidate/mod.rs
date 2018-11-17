use super::*;

use std::iter::once;

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

    fn expand_one(&self, symbol: usize, n: usize) -> Self {
        let tail_of_string = self.build_tail(symbol, n);

        if Self::less_than_full(&self.tail_of_string, n) {
            return self.candidate_with_wasted_symbol(tail_of_string);
        }

        if Self::less_than_full(&tail_of_string, n) {
            return self.candidate_with_wasted_symbol(tail_of_string);
        }

        let id = self.permutation_id(symbol);

        if self.permutations_seen.contains(id) {
            return self.candidate_with_wasted_symbol(tail_of_string);
        }

        self.candidate_with_new_permutation(tail_of_string, id)
    }

    fn candidate_with_wasted_symbol(&self, tail_of_string: Vec<usize>) -> Self {
        Candidate {
            permutations_seen: self.permutations_seen.clone(),
            tail_of_string: tail_of_string,
            wasted_symbols: self.wasted_symbols + 1,
        }
    }

    fn candidate_with_new_permutation(&self, tail_of_string: Vec<usize>, id: usize) -> Self {
        let mut permutations_seen = self.permutations_seen.clone();
        permutations_seen.insert(id);

        let wasted_symbols = self.wasted_symbols;
        Candidate { permutations_seen, tail_of_string, wasted_symbols }
    }

    fn less_than_full(tail_of_string: &Vec<usize>, n: usize) -> bool {
        tail_of_string.len() < n - 1
    }

    // TODO: update Lehmer crate to accept a slice or iterator of usize
    fn permutation_id(&self, symbol: usize) -> usize {
        let permutation = self.tail_of_string
            .iter()
            .map(|&e| e as u8)
            .chain(once(symbol as u8))
            .collect();

        Lehmer::from_permutation(permutation).to_decimal() as usize
    }

    fn build_tail(&self, symbol: usize, n: usize) -> Vec<usize> {
        let head = &self.tail_of_string;

        let index = match head.iter().position(|&e| e == symbol) {
            Some(index) => index + 1,
            None => match Self::less_than_full(head, n) {
                true => 0,
                false => 1,
            }
        };

        Self::append(&head[index..], symbol)
    }

    fn append(slice: &[usize], symbol: usize) -> Vec<usize> {
        slice.iter().map(|&e| e).chain(once(symbol)).collect()
    }
}

#[cfg(test)]
mod test;
