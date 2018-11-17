use super::*;

type Subject = Candidate;

mod seed {
    use super::*;

    #[test]
    fn it_has_seen_the_first_permutation() {
        let subject = Subject::seed(5);

        assert_eq!(subject.permutations_seen.contains(0), true);
        assert_eq!(subject.permutations_seen.contains(1), false);
    }

    #[test]
    fn it_contains_the_tail_of_the_01234_permutation() {
        let subject = Subject::seed(5);

        assert_eq!(subject.tail_of_string, &[1, 2, 3, 4]);
    }

    #[test]
    fn it_sets_wasted_symbols_to_zero() {
        let subject = Subject::seed(5);

        assert_eq!(subject.wasted_symbols, 0);
    }
}

mod expand_one {
    use super::*;

    mod when_the_next_symbol_adds_a_permutation {
        use super::*;

        #[test]
        fn it_has_seen_the_new_permutation() {
            let subject = Subject::seed(5);
            let candidate = subject.expand_one(0, 5);

            let lehmer = Lehmer::from_permutation(vec![1, 2, 3, 4, 0]);
            let decimal = lehmer.to_decimal() as usize;

            assert_eq!(candidate.permutations_seen.contains(decimal), true);
            assert_eq!(candidate.permutations_seen.len(), 2);
        }

        #[test]
        fn it_builds_a_tail_from_the_end_of_the_permutation() {
            let subject = Subject::seed(5);
            let candidate = subject.expand_one(0, 5);

            assert_eq!(candidate.tail_of_string, &[2, 3, 4, 0]);
        }

        #[test]
        fn it_has_no_additional_wasted_symbols() {
            let subject = Subject::seed(5);
            let candidate = subject.expand_one(0, 5);

            assert_eq!(candidate.wasted_symbols, 0);
        }
    }

    mod when_the_next_symbol_does_not_add_a_permutation {
        use super::*;

        #[test]
        fn it_has_not_seen_any_new_permutations() {
            let subject = Subject::seed(5);
            let candidate = subject.expand_one(3, 5);

            assert_eq!(candidate.permutations_seen.len(), 1);
        }

        #[test]
        fn it_builds_a_tail_after_the_repeated_symbol_in_the_previous_tail() {
            let subject = Subject::seed(5);

            let candidate = subject.expand_one(1, 5);
            assert_eq!(candidate.tail_of_string, &[2, 3, 4, 1]);

            let candidate = subject.expand_one(2, 5);
            assert_eq!(candidate.tail_of_string, &[3, 4, 2]);

            let candidate = subject.expand_one(3, 5);
            assert_eq!(candidate.tail_of_string, &[4, 3]);

            let candidate = subject.expand_one(4, 5);
            assert_eq!(candidate.tail_of_string, &[4]);
        }

        #[test]
        fn it_has_one_additional_wasted_symbol() {
            let subject = Subject::seed(5);

            let candidate = subject.expand_one(3, 5);
            assert_eq!(candidate.wasted_symbols, 1);

            let candidate = candidate.expand_one(3, 5);
            assert_eq!(candidate.wasted_symbols, 2);
        }
    }

    mod for_a_more_complicated_example {
        use super::*;

        #[test]
        fn it() {
            let n = 5;

            let subject = Subject::seed(n);
            assert_eq!(subject.permutations_seen.len(), 1);
            assert_eq!(subject.tail_of_string, &[1, 2, 3, 4]);
            assert_eq!(subject.wasted_symbols, 0);

            let depth_1 = subject.expand_one(0, n);
            assert_eq!(depth_1.permutations_seen.len(), 2);
            assert_eq!(depth_1.tail_of_string, &[2, 3, 4, 0]);
            assert_eq!(depth_1.wasted_symbols, 0);

            let depth_2 = depth_1.expand_one(1, n);
            assert_eq!(depth_2.permutations_seen.len(), 3);
            assert_eq!(depth_2.tail_of_string, &[3, 4, 0, 1]);
            assert_eq!(depth_2.wasted_symbols, 0);

            let depth_3 = depth_2.expand_one(4, n);
            assert_eq!(depth_3.permutations_seen.len(), 3);
            assert_eq!(depth_3.tail_of_string, &[0, 1, 4]);
            assert_eq!(depth_3.wasted_symbols, 1);

            let depth_4 = depth_3.expand_one(3, n);
            assert_eq!(depth_4.permutations_seen.len(), 3);
            assert_eq!(depth_4.tail_of_string, &[0, 1, 4, 3]);
            assert_eq!(depth_4.wasted_symbols, 2);

            let depth_5 = depth_4.expand_one(0, n);
            assert_eq!(depth_5.permutations_seen.len(), 4);
            assert_eq!(depth_5.tail_of_string, &[1, 4, 3, 0]);
            assert_eq!(depth_5.wasted_symbols, 2);
        }
    }
}
