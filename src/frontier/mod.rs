use super::candidate::Candidate;

use ::bucket_queue::*;

use std::collections::VecDeque;
use std::collections::HashSet;

type PriorityQueue = BucketQueue<BucketQueue<VecDeque<Candidate>>>;
type BucketID = (usize, usize);

pub struct Frontier {
    enabled_queue: PriorityQueue,
    disabled_queue: PriorityQueue,
    disabled: HashSet<BucketID>,
}

impl Frontier {
    pub fn new() -> Self {
        Frontier {
            enabled_queue: PriorityQueue::new(),
            disabled_queue: PriorityQueue::new(),
            disabled: HashSet::new(),
        }
    }

    pub fn add(&mut self, candidate: Candidate, n: usize) {
        let wasted_symbols = candidate.total_waste(n);
        let permutations = candidate.permutations_seen.len();

        self.queue_for(&(wasted_symbols, permutations))
            .bucket_for_adding(wasted_symbols)
            .enqueue(candidate, permutations);
    }

    pub fn next(&mut self) -> Option<Candidate> {
        let waste = self.min_waste()?;
        let bucket = self.enabled_queue.bucket_for_removing(waste)?;

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
        let mut bucket = self.enabled_queue.bucket(wasted_symbols);
        let min = bucket.min_priority()?;

        for p in min..threshold {
            self.disable(&(wasted_symbols, p));
        }

        None
    }

    pub fn len(&self) -> usize {
        self.enabled_queue.len()
    }

    pub fn min_waste(&self) -> Option<usize> {
        self.enabled_queue.min_priority()
    }

    pub fn max_waste(&self) -> Option<usize> {
        self.enabled_queue.max_priority()
    }

    fn enable(&mut self, bucket_id: &BucketID) -> bool {
        if self.disabled.remove(bucket_id) {
            Self::swap(&mut self.disabled_queue, &mut self.enabled_queue, bucket_id).is_some()
        } else {
            false
        }
    }

    fn disable(&mut self, bucket_id: &BucketID) -> bool {
        if self.disabled.insert(*bucket_id) {
            Self::swap(&mut self.enabled_queue, &mut self.disabled_queue, bucket_id).is_some()
        } else {
            false
        }
    }

    fn queue_for(&mut self, bucket_id: &BucketID) -> &mut PriorityQueue {
        if self.disabled.contains(bucket_id) {
            &mut self.disabled_queue
        } else {
            &mut self.enabled_queue
        }
    }

    fn swap(from: &mut PriorityQueue, to: &mut PriorityQueue, bucket_id: &BucketID) -> Option<()> {
        let bucket_0 = from.bucket_for_peeking(bucket_id.0)?;
        let bucket_1 = bucket_0.bucket_for_peeking(bucket_id.1)?;

        if bucket_1.is_empty() {
            return None;
        }

        let contents = from.bucket(bucket_id.0).replace(bucket_id.1, None);
        to.bucket(bucket_id.0).replace(bucket_id.1, contents);

        Some(())
    }
}

#[cfg(test)]
mod test;
