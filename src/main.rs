extern crate bit_set;
extern crate bucket_queue;

mod bounds;
mod candidate;
mod frontier;

use self::bounds::Bounds;
use self::candidate::Candidate;
use self::frontier::Frontier;

use std::process::exit;

fn main() {
    let n = 4;

    let candidate = Candidate::seed(n);
    let mut frontier = Frontier::new();
    let mut bounds = Bounds::new(n);

    frontier.add(candidate, n);

    while let Some(mut wasted_symbols) = frontier.min_waste() {
        wasted_symbols = frontier.unprune(
            wasted_symbols,
            &bounds.lower_bounds,
            &bounds.upper_bounds,
        );

        let candidate = frontier.next().unwrap();
        let permutations = candidate.number_of_permutations();

        if bounds.update(wasted_symbols, permutations) {
            let threshold = bounds.thresholds[wasted_symbols];
            frontier.prune(wasted_symbols, threshold, true);

            println!("{:?}", bounds.lower_bounds);
        }

        let upper_bound = bounds.upper(wasted_symbols);
        for child in candidate.expand(upper_bound, n) {
            frontier.add(child, n);
        }

        if bounds.found_for_superpermutation() {
            exit(0);
        }
    }
}
