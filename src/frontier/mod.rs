use super::candidate::Candidate;

use ::bucket_queue::*;
use std::collections::VecDeque;

pub struct Frontier {
    priority_queue: BucketQueue<BucketQueue<VecDeque<Candidate>>>,
}

impl Frontier {
    pub fn new() -> Self {
        let queue = BucketQueue::<BucketQueue<VecDeque<Candidate>>>::new();

        Frontier { priority_queue: queue }
    }

    pub fn add(&mut self, candidate: Candidate, n: usize) {
        let permutations = candidate.permutations_seen.len();

        self.priority_queue
            .bucket_for_adding(candidate.total_waste(n))
            .enqueue(candidate, permutations);
    }

    pub fn next(&mut self) -> Option<Candidate> {
        let waste = self.min_waste()?;
        let bucket = self.priority_queue.bucket_for_removing(waste)?;

        bucket.dequeue_max()
    }

    pub fn prune(&mut self, wasted_symbols: usize, threshold: usize, eager: bool) -> Option<()> {
        let max = match eager {
            true => self.max_waste()?,
            false => wasted_symbols,
        };

        for w in wasted_symbols..=max {
            self.prune_one(w, threshold);
        }

        None
    }

    pub fn prune_one(&mut self, wasted_symbols: usize, threshold: usize) -> Option<()> {
        let mut bucket = self.priority_queue.bucket(wasted_symbols);
        let min = bucket.min_priority()?;

        for p in min..threshold {
            bucket.prune(p);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.priority_queue.len()
    }

    pub fn len_for_waste(&self, wasted_symbols: usize) -> usize {
        let bucket = self.priority_queue.bucket_for_peeking(wasted_symbols);
        bucket.map_or(0, |b| b.len())
    }

    pub fn min_waste(&self) -> Option<usize> {
        self.priority_queue.min_priority()
    }

    pub fn max_waste(&self) -> Option<usize> {
        self.priority_queue.max_priority()
    }
}

#[cfg(test)]
mod test;
