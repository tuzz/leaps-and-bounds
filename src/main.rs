extern crate bit_set;
extern crate bucket_queue;
extern crate rayon;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
extern crate bincode;

mod bounds;
mod candidate;
mod disk;
mod frontier;
mod ui;

use self::bounds::Bounds;
use self::candidate::Candidate;
use self::frontier::Frontier;
use self::ui::UI;

use std::process::exit;

fn main() {
    UI::print_introduction();
    let n = UI::ask_for_n();
    let memory = UI::ask_for_memory();
    let gzip = UI::ask_for_gzip();
    let verbose = UI::ask_for_verbose();
    UI::print_running();

    let candidate = Candidate::seed(n);
    let mut frontier = Frontier::new(memory, gzip, verbose, n);
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
            let waste = bounds.lower_bounds.len() - 1;
            let factorial = Bounds::factorial(n);
            let length = n - 1 + factorial + waste;

            println!("{} wasted symbols: at most {} permutations", waste, factorial);
            println!();
            println!("--->>> Done!");
            println!();
            println!("A maximum of {} wasted symbols can fit all {}! = {} permutations.", waste, n, factorial);
            println!("The shortest superpermutation contains {} + {} + {} = {} symbols.", n - 1, factorial, waste, length);
            println!();

            exit(0);
        }
    }
}
