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
            for p in 0..threshold {
                self.disable(&(w, p));
            }
        }

        None
    }

    pub fn unprune(&mut self, wasted_symbols: usize, lower_bounds: &[usize], upper_bounds: &[usize]) -> usize {
        if wasted_symbols < lower_bounds.len() {
            return wasted_symbols;
        }

        let previous_waste = wasted_symbols - 1;

        let lower_bound = lower_bounds[previous_waste];
        let upper_bound = upper_bounds[previous_waste];

        for p in ((lower_bound + 1)..=upper_bound).rev() {
            for w in (0..=previous_waste).rev() {
                let allowed_waste = previous_waste - w;
                let max_permutations = upper_bounds[allowed_waste];

                if self.enable(&(w, p.saturating_sub(max_permutations))) {
                    return w;
                }
            }
        }

        wasted_symbols
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
