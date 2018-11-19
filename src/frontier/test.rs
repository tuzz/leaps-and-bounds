use super::*;
use std::usize::MAX;

type Subject = Frontier;

const N: usize = 5;
const F: bool = false;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_new_frontier() {
        let subject = Subject::new();
        assert_eq!(subject.len(), 0);
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
        let mut queue = subject.priority_queue;

        assert_eq!(queue.min_priority(), Some(total_waste));
        assert_eq!(queue.min_bucket().min_priority(), Some(permutations));
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

mod len {
    use super::*;

    #[test]
    fn it_returns_how_many_candidates_there_are_in_total() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        for c in candidate.expand(MAX, N) {
            subject.add(c, N);
        }

        assert_eq!(subject.len(), 4);
    }
}

mod len_for_waste {
    use super::*;

    #[test]
    fn it_returns_how_many_candidates_there_are_for_the_given_number_of_wasted_symbols() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed(N);

        for c in candidate.expand(MAX, N) {
            subject.add(c, N);
        }

        assert_eq!(subject.len_for_waste(0), 1);
        assert_eq!(subject.len_for_waste(1), 1);
        assert_eq!(subject.len_for_waste(2), 1);
        assert_eq!(subject.len_for_waste(3), 1);

        assert_eq!(subject.len_for_waste(4), 0);
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
