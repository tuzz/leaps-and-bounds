use super::*;
use std::usize::MAX;
use bit_set::BitSet;

type Subject = Frontier;

const N: usize = 5;
const F: bool = false;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_new_frontier() {
        let subject = Subject::new();

        assert_eq!(subject.enabled_queue.len(), 0);
        assert_eq!(subject.disabled_queue.len(), 0);
        assert_eq!(subject.disabled.len(), 0);
    }
}

mod add {
    use super::*;

    #[test]
    fn it_adds_a_candidate_to_the_frontier() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        subject.add(candidate, N);
        assert_eq!(subject.len(), 1);

        let candidate = subject.next().unwrap();

        assert_eq!(candidate.permutations_seen.len(), 1);
        assert_eq!(candidate.tail_of_string, &[1, 2, 3, 4]);
        assert_eq!(candidate.wasted_symbols, 0);
    }

    #[test]
    fn it_queues_the_candidate_based_on_total_waste_and_number_of_permutations() {
        let mut subject = Subject::new();

        let seed = Candidate::seed(N);
        let candidate = seed.expand(MAX, N).last().unwrap();

        let total_waste = candidate.total_waste(N);
        let permutations = candidate.number_of_permutations();

        subject.add(candidate, N);
        let mut queue = subject.enabled_queue;

        assert_eq!(queue.min_priority(), Some(total_waste));
        assert_eq!(queue.min_bucket().min_priority(), Some(permutations));
    }

    mod when_the_bucket_is_disabled {
        use super::*;

        #[test]
        fn it_adds_the_candidate_to_the_disabled_queue() {
            let mut subject = Subject::new();
            let seed = Candidate::seed(N);

            let candidate = seed.expand(MAX, N).last().unwrap();

            let total_waste = candidate.total_waste(N);
            let permutations = candidate.number_of_permutations();

            subject.disable(&(total_waste, permutations));
            subject.add(candidate, N);

            assert_eq!(subject.enabled_queue.len(), 0);
            assert_eq!(subject.disabled_queue.len(), 1);
        }
    }
}

mod prune {
    use super::*;

    #[test]
    fn it_removes_candidates_with_the_given_waste_that_do_not_meet_the_threshold() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        for c in candidate.expand(MAX, N) {
            subject.add(c, N);
        }

        assert_eq!(subject.len(), 4);

        // Current state of Frontier:
        // 012340: 0 waste, 2 permutations
        // 012341: 1 waste, 1 permutation
        // 012342: 2 waste, 1 permutation
        // 012343: 3 waste, 1 permutation

        subject.prune(2, 0, F); // does nothing (above threshold)
        assert_eq!(subject.len(), 4);

        subject.prune(2, 1, F); // does nothing (equal to threshold)
        assert_eq!(subject.len(), 4);

        subject.prune(2, 2, F); // prunes 012342
        assert_eq!(subject.len(), 3);

        subject.prune(2, 3, F); // does nothing (already pruned)
        assert_eq!(subject.len(), 3);

        // Check the right candidate was pruned:
        assert_eq!(subject.next().unwrap().tail_of_string, &[2, 3, 4, 0]);
        assert_eq!(subject.next().unwrap().tail_of_string, &[2, 3, 4, 1]);
        assert_eq!(subject.next().unwrap().tail_of_string, &[4, 3]);
        assert_eq!(subject.next(), None);
    }

    mod when_pruning_eagerly {
        use super::*;

        #[test]
        fn it_also_prunes_candidates_that_have_more_wasted_symbols() {
            let mut subject = Subject::new();
            let candidate = Candidate::seed(N);

            for c in candidate.expand(MAX, N) {
                subject.add(c, N);
            }

            assert_eq!(subject.len(), 4);

            // Current state of Frontier:
            // 012340: 0 waste, 2 permutations
            // 012341: 1 waste, 1 permutation
            // 012342: 2 waste, 1 permutation
            // 012343: 3 waste, 1 permutation

            subject.prune(2, 0, true); // does nothing (above threshold)
            assert_eq!(subject.len(), 4);

            subject.prune(2, 1, true); // does nothing (equal to threshold)
            assert_eq!(subject.len(), 4);

            subject.prune(2, 2, true); // prunes 012342 _and_ 012343
            assert_eq!(subject.len(), 2);

            subject.prune(2, 3, true); // does nothing (already pruned)
            assert_eq!(subject.len(), 2);

            // Check the right candidate was pruned:
            assert_eq!(subject.next().unwrap().tail_of_string, &[2, 3, 4, 0]);
            assert_eq!(subject.next().unwrap().tail_of_string, &[2, 3, 4, 1]);
            assert_eq!(subject.next(), None);
        }
    }
}

mod unprune {
    use super::*;

    #[test]
    fn it_unprunes_buckets_based_on_the_bounds_for_the_current_wasted_symbols() {
        let mut subject = Subject::new();

        add_pruned_candidate(&mut subject, 1, 3);
        add_pruned_candidate(&mut subject, 1, 4);
        add_pruned_candidate(&mut subject, 1, 5);
        add_pruned_candidate(&mut subject, 1, 6);
        add_pruned_candidate(&mut subject, 1, 7);

        add_pruned_candidate(&mut subject, 2, 7);
        add_pruned_candidate(&mut subject, 2, 8);
        add_pruned_candidate(&mut subject, 2, 9);
        add_pruned_candidate(&mut subject, 2, 10);
        add_pruned_candidate(&mut subject, 2, 11);

        assert_eq!(subject.len(), 0);

        // The bounds we're exploring for waste 3 are 16..=18:
        let lower_bounds = vec![4, 8, 12, 16];
        let upper_bounds = vec![4, 8, 12, 18];

        // We've just moved on to the next number of wasted symbols:
        let previous_waste = 3;
        let wasted_symbols = previous_waste + 1;

        // Waste 2 is allowed 1 more wasted symbol which can add 8 permutations.
        // To improve on the lower bound of 16, we'd need to see 9 permutations.
        // We should unprune waste 2 candidates with permutations between 9..=10

        // Waste 1 is allowed 2 more wasted symbols which can add 12 permutations.
        // To improve on the lower bound of 16, we'd need to see 5 permutations.
        // We should unprune waste 1 candidates with permutations between 5..=6

        subject.unprune(wasted_symbols, &lower_bounds, &upper_bounds);
        assert_eq!(subject.len(), 1);
        assert_eq!(is_pruned(&subject, 2, 10), false);

        subject.unprune(wasted_symbols, &lower_bounds, &upper_bounds);
        assert_eq!(subject.len(), 2);
        assert_eq!(is_pruned(&subject, 1, 6), false);

        subject.unprune(wasted_symbols, &lower_bounds, &upper_bounds);
        assert_eq!(subject.len(), 3);
        assert_eq!(is_pruned(&subject, 2, 9), false);

        subject.unprune(wasted_symbols, &lower_bounds, &upper_bounds);
        assert_eq!(subject.len(), 4);
        assert_eq!(is_pruned(&subject, 1, 5), false);

        subject.unprune(wasted_symbols, &lower_bounds, &upper_bounds);
        assert_eq!(subject.len(), 4);
    }

    #[test]
    fn it_returns_the_number_of_wasted_symbols_for_the_bucket_that_was_unpruned() {
        let mut subject = Subject::new();

        add_pruned_candidate(&mut subject, 1, 6);
        add_pruned_candidate(&mut subject, 2, 10);

        let lower_bounds = vec![4, 8, 12, 16];
        let upper_bounds = vec![4, 8, 12, 18];

        assert_eq!(subject.unprune(4, &lower_bounds, &upper_bounds), 2);
        assert_eq!(subject.unprune(4, &lower_bounds, &upper_bounds), 1);
    }

    #[test]
    fn it_returns_the_original_wasted_symbols_once_all_buckets_have_been_unpruned() {
        let mut subject = Subject::new();

        add_pruned_candidate(&mut subject, 1, 6);
        add_pruned_candidate(&mut subject, 2, 10);

        let lower_bounds = vec![4, 8, 12, 16];
        let upper_bounds = vec![4, 8, 12, 18];

        subject.unprune(4, &lower_bounds, &upper_bounds);
        subject.unprune(4, &lower_bounds, &upper_bounds);

        assert_eq!(subject.unprune(4, &lower_bounds, &upper_bounds), 4);
        assert_eq!(subject.unprune(4, &lower_bounds, &upper_bounds), 4);
    }

    fn add_pruned_candidate(frontier: &mut Frontier, wasted_symbols: usize, permutations: usize) {
        let mut permutations_seen = BitSet::new();

        for i in 0..permutations {
            permutations_seen.insert(i);
        }

        let candidate = Candidate {
            permutations_seen,
            tail_of_string: vec![0, 1, 2, 3],
            wasted_symbols
        };

        frontier.add(candidate, N);
        frontier.disable(&(wasted_symbols, permutations));
    }

    fn is_pruned(frontier: &Frontier, wasted_symbols: usize, permutations: usize) -> bool {
        frontier.disabled.contains(&(wasted_symbols, permutations))
    }
}

mod next {
    use super::*;

    #[test]
    fn it_returns_the_candidates_ordered_by_waste_asc_then_number_of_permutations_desc() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        for c in candidate.expand(MAX, N) {
            subject.add(c, N);
        }

        let candidate = subject.next().unwrap();
        assert_eq!(candidate.total_waste(N), 0);
        assert_eq!(candidate.number_of_permutations(), 2);

        let candidate = subject.next().unwrap();
        assert_eq!(candidate.total_waste(N), 1);
        assert_eq!(candidate.number_of_permutations(), 1);

        let candidate = subject.next().unwrap();
        assert_eq!(candidate.total_waste(N), 2);
        assert_eq!(candidate.number_of_permutations(), 1);

        let candidate = subject.next().unwrap();
        assert_eq!(candidate.total_waste(N), 3);
        assert_eq!(candidate.number_of_permutations(), 1);

        assert_eq!(subject.next(), None);
    }

    #[test]
    fn it_does_not_return_candidates_from_the_disabled_queue() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        for c in candidate.expand(MAX, N) {
            subject.add(c, N);
        }

        subject.disable(&(1, 1));
        subject.disable(&(2, 1));

        let candidate = subject.next().unwrap();
        assert_eq!(candidate.total_waste(N), 0);
        assert_eq!(candidate.number_of_permutations(), 2);

        // (1, 1) and (2, 1) are not returned

        let candidate = subject.next().unwrap();
        assert_eq!(candidate.total_waste(N), 3);
        assert_eq!(candidate.number_of_permutations(), 1);

        assert_eq!(subject.next(), None);
    }
}

mod min_waste {
    use super::*;

    #[test]
    fn it_returns_the_minimum_number_of_wasted_symbols_for_candidates_in_the_frontier() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        for c in candidate.expand(MAX, N) {
            subject.add(c, N);
        }

        assert_eq!(subject.min_waste(), Some(0));

        let _ = subject.next();
        assert_eq!(subject.min_waste(), Some(1));

        let _ = subject.next();
        assert_eq!(subject.min_waste(), Some(2));
    }
}

mod max_waste {
    use super::*;

    #[test]
    fn it_returns_the_maximum_number_of_wasted_symbols_for_candidates_in_the_frontier() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        for c in candidate.expand(MAX, N) {
            subject.add(c, N);
        }

        assert_eq!(subject.max_waste(), Some(3));
    }
}

mod enable_and_disable {
    use super::*;

    #[test]
    fn it_adds_or_removes_the_bucket_id_from_the_disabled_hash_set() {
        let mut subject = Subject::new();
        let bucket_id = (2, 3);

        subject.disable(&bucket_id);
        assert_eq!(subject.disabled.contains(&bucket_id), true);

        subject.enable(&bucket_id);
        assert_eq!(subject.disabled.contains(&bucket_id), false);
    }

    #[test]
    fn it_moves_the_bucket_between_the_enabled_and_disabled_queues() {
        let mut subject = Subject::new();

        let seed = Candidate::seed(N);
        let candidate = seed.expand(MAX, N).last().unwrap();

        let total_waste = candidate.total_waste(N);
        let permutations = candidate.number_of_permutations();

        let bucket_id = (total_waste, permutations);

        subject.add(candidate, N);

        assert_eq!(subject.enabled_queue.len(), 1);
        assert_eq!(subject.disabled_queue.len(), 0);

        subject.disable(&bucket_id);

        assert_eq!(subject.enabled_queue.len(), 0);
        assert_eq!(subject.disabled_queue.len(), 1);

        subject.enable(&bucket_id);

        assert_eq!(subject.enabled_queue.len(), 1);
        assert_eq!(subject.disabled_queue.len(), 0);
    }

    #[test]
    fn it_returns_true_if_the_bucket_that_was_moved_contained_something() {
        let mut subject = Subject::new();

        let seed = Candidate::seed(N);
        let candidate = seed.expand(MAX, N).last().unwrap();

        let total_waste = candidate.total_waste(N);
        let permutations = candidate.number_of_permutations();

        let bucket_id = (total_waste, permutations);

        assert_eq!(subject.enable(&bucket_id), false);
        assert_eq!(subject.disable(&bucket_id), false);

        subject.add(candidate, N);

        assert_eq!(subject.disable(&bucket_id), false);
        assert_eq!(subject.disable(&bucket_id), false);

        assert_eq!(subject.enable(&bucket_id), true);
        assert_eq!(subject.enable(&bucket_id), false);

        subject.next();
        assert_eq!(subject.len(), 0);

        assert_eq!(subject.disable(&bucket_id), false);
        assert_eq!(subject.enable(&bucket_id), false);
    }
}
