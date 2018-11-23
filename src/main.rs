extern crate bit_set;
extern crate bucket_queue;

mod bounds;
mod candidate;
mod frontier;

use self::bounds::Bounds;
use self::candidate::Candidate;
use self::frontier::Frontier;

use std::process::exit;
use std::io::{prelude::*, stdin, stdout};

fn main() {
    print!("This tool will try to find the length of the shortest ");
    print!("superpermutation on n symbols. Please enter n: ");
    stdout().flush().ok().expect("Failed to flush stdout");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input.");
    let n: usize = input.trim().parse().expect("Failed to parse integer.");

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
        }

        let upper_bound = bounds.upper(wasted_symbols);
        for child in candidate.expand(upper_bound, n) {
            frontier.add(child, n);
        }

        if bounds.found_for_superpermutation() {
            let factorial = Bounds::factorial(n);
            let waste = bounds.lower_bounds.len() - 1;
            let length = factorial + waste + n - 1;

            println!("{} wasted characters: at most {} permutations", waste, factorial);
            println!("\n-----\nDONE!\n-----\n");
            print!("Minimal superpermutations on {} symbols have {} ", n, waste);
            println!("wasted characters and a length of {}.", length);

            exit(0);
        }
    }
}
