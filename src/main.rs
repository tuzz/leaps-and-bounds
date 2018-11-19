extern crate bit_set;
extern crate bucket_queue;

mod bounds;
mod candidate;
mod frontier;

use self::bounds::Bounds;
use self::candidate::Candidate;
use self::frontier::Frontier;

fn main() {
    let n = 4;

    let candidate = Candidate::seed(n);
    let mut frontier = Frontier::new();
    let mut bounds = Bounds::new(n);

    frontier.add(candidate, n);

    while let Some(wasted_symbols) = frontier.min_waste() {
        let next_candidate = frontier.next().unwrap();
        let permutations = next_candidate.number_of_permutations();

        if bounds.update(wasted_symbols, permutations) {
            let threshold = bounds.thresholds[wasted_symbols];
            frontier.prune(wasted_symbols, threshold, true);

            println!("{:?}", bounds.lower_bounds);
        }

        let upper_bound = bounds.upper(wasted_symbols);
        for candidate in next_candidate.expand(upper_bound, n) {
            frontier.add(candidate, n);
        }

        if bounds.found_for_superpermutation() {
            break;
        }
    }
}
