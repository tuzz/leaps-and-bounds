mod serialize;

use bit_set::BitSet;
use lehmer::Lehmer;
use std::iter::once;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(serialize_with="serialize::serialize", deserialize_with="serialize::deserialize")]
    pub permutations_seen: BitSet,
    pub tail_of_string: Vec<u8>,
    pub wasted_symbols: u16,
}

impl Candidate {
    pub fn seed(n: usize) -> Self {
        let max_value = Lehmer::max_value(n) as usize;
        let mut seen = BitSet::with_capacity(max_value);

        seen.insert(0);

        Candidate {
            permutations_seen: seen,
            tail_of_string: (1..n as u8).collect(),
            wasted_symbols: 0,
        }
    }

    pub fn expand(self, upper_bound: usize, n: usize) -> impl Iterator<Item=Self> {
        let last_symbol = *self.tail_of_string.last().unwrap();
        let at_upper_bound = self.number_of_permutations() == upper_bound;

        (0..n as u8)
            .filter(move |&s| s != last_symbol)
            .map(move |s| self.expand_one(s, at_upper_bound, n))
    }

    pub fn number_of_permutations(&self) -> usize {
        self.permutations_seen.len()
    }

    pub fn future_waste(&self, n: usize) -> usize {
        n - self.tail_of_string.len() - 1
    }

    pub fn total_waste(&self, n: usize) -> usize {
        self.wasted_symbols as usize + self.future_waste(n)
    }

    fn expand_one(&self, symbol: u8, at_upper_bound: bool, n: usize) -> Self {
        let tail_of_string = self.build_tail(symbol, n);

        if Self::less_than_full(&self.tail_of_string, n) {
            return self.candidate_with_wasted_symbol(tail_of_string, 1);
        }

        if Self::less_than_full(&tail_of_string, n) {
            return self.candidate_with_wasted_symbol(tail_of_string, 1);
        }

        if self.tail_starts_with(symbol) {
            return self.candidate_with_wasted_symbol(tail_of_string, 1);
        }

        if at_upper_bound {
            return self.candidate_with_wasted_symbol(tail_of_string, 1);
        }

        let id = Self::permutation_id(&self.tail_of_string, symbol);

        if self.permutations_seen.contains(id) {
            let penalty = match self.seen_next_tail_as_well(&tail_of_string, n) {
                false => 1,
                true => 2,
            };

            return self.candidate_with_wasted_symbol(tail_of_string, penalty);
        }

        self.candidate_with_new_permutation(tail_of_string, id)
    }

    fn candidate_with_wasted_symbol(&self, tail_of_string: Vec<u8>, penalty: usize) -> Self {
        Candidate {
            permutations_seen: self.permutations_seen.clone(),
            tail_of_string: tail_of_string,
            wasted_symbols: self.wasted_symbols + penalty as u16,
        }
    }

    fn candidate_with_new_permutation(&self, tail_of_string: Vec<u8>, id: usize) -> Self {
        let mut permutations_seen = self.permutations_seen.clone();
        permutations_seen.insert(id);

        let wasted_symbols = self.wasted_symbols;
        Candidate { permutations_seen, tail_of_string, wasted_symbols }
    }

    fn less_than_full(tail_of_string: &[u8], n: usize) -> bool {
        tail_of_string.len() < n - 1
    }

    fn tail_starts_with(&self, symbol: u8) -> bool {
        symbol == *self.tail_of_string.first().unwrap()
    }

    // TODO: update Lehmer crate to accept a slice or iterator of usize
    fn permutation_id(tail_of_string: &[u8], symbol: u8) -> usize {
        let permutation = tail_of_string
            .iter()
            .map(|&e| e)
            .chain(once(symbol))
            .collect();

        Lehmer::from_permutation(permutation).to_decimal() as usize
    }

    fn build_tail(&self, symbol: u8, n: usize) -> Vec<u8> {
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

    fn seen_next_tail_as_well(&self, tail_of_string: &[u8], n: usize) -> bool {
        let mut symbols_in_tail = vec![false; n];

        for &symbol in tail_of_string {
            symbols_in_tail[symbol as usize] = true;
        }

        for (symbol, &present) in symbols_in_tail.iter().enumerate() {
            if !present {
                let id = Self::permutation_id(tail_of_string, symbol as u8);
                return self.permutations_seen.contains(id);
            }
        }

        false
    }

    fn append(slice: &[u8], symbol: u8) -> Vec<u8> {
        slice.iter().map(|&e| e).chain(once(symbol)).collect()
    }
}

#[cfg(test)]
mod test;
