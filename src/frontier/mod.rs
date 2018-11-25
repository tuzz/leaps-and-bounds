use super::candidate::Candidate;
use super::disk::Disk;

use ::bucket_queue::*;

use std::collections::VecDeque;
use std::collections::HashSet;
use rayon::prelude::*;

type PriorityQueue = BucketQueue<BucketQueue<VecDeque<Candidate>>>;
type BucketID = (usize, usize);

pub struct Frontier {
    enabled_queue: PriorityQueue,
    disabled_queue: PriorityQueue,
    disabled: HashSet<BucketID>,
    disk: Disk,
}

impl Frontier {
    pub fn new() -> Self {
        Frontier {
            enabled_queue: PriorityQueue::new(),
            disabled_queue: PriorityQueue::new(),
            disabled: HashSet::new(),
            disk: Disk::new("queue".to_string(), false), // TODO: configure
        }
    }

    pub fn add(&mut self, candidate: Candidate, n: usize) {
        let wasted_symbols = candidate.total_waste(n);
        let permutations = candidate.permutations_seen.len();

        self.queue_for(&(wasted_symbols, permutations))
            .bucket_for_adding(wasted_symbols)
            .enqueue(candidate, permutations);

        self.offload_buckets_to_disk();
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
                let remainder = p.saturating_sub(max_permutations);

                if self.enable(&(w, remainder)) {
                    println!("  unpruning {:02}, {:03} ... queue: {}", w, remainder, self.len());
                    return w;
                }
            }
        }

        wasted_symbols
    }

    pub fn len(&self) -> usize {
        self.enabled_queue.len() + self.disabled_queue.len()
    }

    pub fn min_waste(&self) -> Option<usize> {
        self.enabled_queue.min_priority()
    }

    pub fn max_waste(&self) -> Option<usize> {
        self.enabled_queue.max_priority()
    }

    fn enable(&mut self, bucket_id: &BucketID) -> bool {
        if !self.disabled.contains(bucket_id) {
            return false;
        }

        // Swap in buckets from memory:
        if Self::bucket_len(&self.disabled_queue, bucket_id) > 0 {
            Self::swap(&mut self.disabled_queue, &mut self.enabled_queue, bucket_id);
            return true;
        }

        // Swap in buckets from disk:
        if self.onload_from_disk(bucket_id) {
            return true;
        }

        // The bucket is completely empty, disable it:
        self.disabled.remove(bucket_id);
        false
    }

    fn disable(&mut self, bucket_id: &BucketID) -> bool {
        if self.disabled.insert(*bucket_id) {
            Self::swap(&mut self.enabled_queue, &mut self.disabled_queue, bucket_id).is_some()
        } else {
            false
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

    fn queue_for(&mut self, bucket_id: &BucketID) -> &mut PriorityQueue {
        if self.disabled.contains(bucket_id) {
            &mut self.disabled_queue
        } else {
            &mut self.enabled_queue
        }
    }

    fn onload_from_disk(&mut self, bucket_id: &BucketID) -> bool {
        let bucket = match self.disk.read(bucket_id.0, bucket_id.1) {
            None => return false,
            Some(bucket) => bucket,
        };

        if Self::bucket_len(&self.enabled_queue, bucket_id) > 0 {
            panic!("about to overwrite data");
        }

        self.enabled_queue
            .bucket(bucket_id.0)
            .replace(bucket_id.1, Some(bucket));

        true
    }

    fn offload_buckets_to_disk(&mut self) {
        if self.len() < 50_000_000 { // TODO: configure
            return;
        }

        println!("running low on memory, offloading to disk");
        let queue = &mut self.disabled_queue;

        let waste_min = queue.min_priority().unwrap();
        let waste_max = queue.max_priority().unwrap();

        let mut jobs = vec![];

        for w in waste_min..=waste_max {
            let mut waste_bucket = self.disabled_queue.bucket(w);

            let perm_min = match waste_bucket.min_priority() {
                None => continue,
                Some(p) => p,
            };

            let perm_max = waste_bucket.max_priority().unwrap();

            print!("  {:02}, {:03}..{:03} | ", w, perm_min, perm_max);
            for p in perm_min..=perm_max {
                let bucket = match waste_bucket.replace(p, None) {
                    None => continue,
                    Some(b) => b,
                };

                print!("{} ", bucket.len());
                jobs.push((bucket, w, p));
            }

            println!();
        }

        jobs.into_par_iter().for_each(|job| {
            self.disk.write(&job.0, job.1, job.2);
        });

        println!("done, continuing with search");
    }

    fn bucket_len(queue: &PriorityQueue, bucket_id: &BucketID) -> usize {
        match queue.bucket_for_peeking(bucket_id.0) {
            None => 0,
            Some(b) => match b.bucket_for_peeking(bucket_id.1) {
                None => 0,
                Some(b) => b.len(),
            },
        }
    }
}

#[cfg(test)]
mod test;
