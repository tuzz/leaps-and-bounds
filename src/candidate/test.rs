use super::*;
use std::usize::MAX;

type Subject = Candidate;

const N: usize = 5;
const F: bool = false;

mod seed {
    use super::*;

    #[test]
    fn it_has_seen_the_first_permutation() {
        let subject = Subject::seed(N);

        assert_eq!(subject.permutations_seen.contains(0), true);
        assert_eq!(subject.permutations_seen.contains(1), false);
    }

    #[test]
    fn it_contains_the_tail_of_the_01234_permutation() {
        let subject = Subject::seed(N);

        assert_eq!(subject.tail_of_string, &[1, 2, 3, 4]);
    }

    #[test]
    fn it_sets_wasted_symbols_to_zero() {
        let subject = Subject::seed(N);

        assert_eq!(subject.wasted_symbols, 0);
    }
}

mod expand {
    use super::*;

    #[test]
    fn it_expands_all_candidates_except_for_the_last_symbol_of_the_tail() {
        let subject = Subject::seed(N);
        let candidates: Vec<Subject> = subject.expand(MAX, N).collect();

        assert_eq!(candidates.len(), 4);

        assert_eq!(candidates[0].permutations_seen.len(), 2);
        assert_eq!(candidates[0].tail_of_string, &[2, 3, 4, 0]);
        assert_eq!(candidates[0].wasted_symbols, 0);

        assert_eq!(candidates[1].permutations_seen.len(), 1);
        assert_eq!(candidates[1].tail_of_string, &[2, 3, 4, 1]);
        assert_eq!(candidates[1].wasted_symbols, 1);

        assert_eq!(candidates[2].permutations_seen.len(), 1);
        assert_eq!(candidates[2].tail_of_string, &[3, 4, 2]);
        assert_eq!(candidates[2].wasted_symbols, 1);

        assert_eq!(candidates[3].permutations_seen.len(), 1);
        assert_eq!(candidates[3].tail_of_string, &[4, 3]);
        assert_eq!(candidates[3].wasted_symbols, 1);
    }
}

mod expand_one {
    use super::*;

    mod when_the_next_symbol_adds_a_permutation {
        use super::*;

        #[test]
        fn it_has_seen_the_new_permutation() {
            let subject = Subject::seed(N);
            let candidate = subject.expand_one(0, F, N);

            let lehmer = Lehmer::from_permutation(vec![1, 2, 3, 4, 0]);
            let decimal = lehmer.to_decimal() as usize;

            assert_eq!(candidate.permutations_seen.contains(decimal), true);
            assert_eq!(candidate.permutations_seen.len(), 2);
        }

        #[test]
        fn it_builds_a_tail_from_the_end_of_the_permutation() {
            let subject = Subject::seed(N);
            let candidate = subject.expand_one(0, F, N);

            assert_eq!(candidate.tail_of_string, &[2, 3, 4, 0]);
        }

        #[test]
        fn it_has_no_additional_wasted_symbols() {
            let subject = Subject::seed(N);
            let candidate = subject.expand_one(0, F, N);

            assert_eq!(candidate.wasted_symbols, 0);
        }
    }

    mod when_the_next_symbol_does_not_add_a_permutation {
        use super::*;

        #[test]
        fn it_has_not_seen_any_new_permutations() {
            let subject = Subject::seed(N);
            let candidate = subject.expand_one(3, F, N);

            assert_eq!(candidate.permutations_seen.len(), 1);
        }

        #[test]
        fn it_builds_a_tail_after_the_repeated_symbol_in_the_previous_tail() {
            let subject = Subject::seed(N);

            let candidate = subject.expand_one(1, F, N);
            assert_eq!(candidate.tail_of_string, &[2, 3, 4, 1]);

            let candidate = subject.expand_one(2, F, N);
            assert_eq!(candidate.tail_of_string, &[3, 4, 2]);

            let candidate = subject.expand_one(3, F, N);
            assert_eq!(candidate.tail_of_string, &[4, 3]);

            let candidate = subject.expand_one(4, F, N);
            assert_eq!(candidate.tail_of_string, &[4]);
        }

        #[test]
        fn it_has_one_additional_wasted_symbol() {
            let subject = Subject::seed(N);

            let candidate = subject.expand_one(3, F, N);
            assert_eq!(candidate.wasted_symbols, 1);

            let candidate = candidate.expand_one(3, F, N);
            assert_eq!(candidate.wasted_symbols, 2);
        }

        mod and_the_permutation_has_been_seen_before {
            use super::*;

            #[test]
            fn it_has_one_additional_wasted_symbol() {
                let subject = Subject::seed(N);

                // Waste a symbol after the first permutation.
                let candidate = subject.expand_one(3, F, N);
                assert_eq!(candidate.wasted_symbols, 1);

                let candidate = candidate.expand_one(0, F, N);
                let candidate = candidate.expand_one(1, F, N);
                let candidate = candidate.expand_one(2, F, N);
                let candidate = candidate.expand_one(3, F, N);
                assert_eq!(candidate.wasted_symbols, 4);

                let candidate = candidate.expand_one(4, F, N);
                assert_eq!(candidate.wasted_symbols, 5);
            }

            mod and_the_only_choice_of_next_symbol_has_also_been_seen_before {
                use super::*;

                #[test]
                fn it_has_two_additional_wasted_symbols() {
                    let subject = Subject::seed(N);

                    // No symbol is wasted here...

                    let candidate = subject.expand_one(0, F, N);
                    let candidate = candidate.expand_one(1, F, N);
                    let candidate = candidate.expand_one(2, F, N);
                    let candidate = candidate.expand_one(3, F, N);
                    assert_eq!(candidate.wasted_symbols, 0);

                    // ... and the only choice after 4 (0) is already taken.
                    let candidate = candidate.expand_one(4, F, N);

                    // So we penalise by an extra symbol of waste:
                    assert_eq!(candidate.wasted_symbols, 2);
                }
            }
        }
    }

    mod when_the_candidate_is_at_the_upper_bound_for_wasted_symbols {
        use super::*;

        #[test]
        fn it_short_circuits_to_return_a_candidate_with_a_wasted_symbol() {
            let subject = Subject::seed(N);
            let at_upper_bound = true;

            let candidate = subject.expand_one(0, at_upper_bound, N);

            assert_eq!(candidate.permutations_seen.len(), 1);
            assert_eq!(candidate.wasted_symbols, 1);
        }
    }

    mod for_a_more_complicated_example {
        use super::*;

        #[test]
        fn it_expands_candidates_correctly() {
            let subject = Subject::seed(N);
            assert_eq!(subject.permutations_seen.len(), 1);
            assert_eq!(subject.tail_of_string, &[1, 2, 3, 4]);
            assert_eq!(subject.wasted_symbols, 0);

            let depth_1 = subject.expand_one(1, F, N);
            assert_eq!(depth_1.permutations_seen.len(), 1);
            assert_eq!(depth_1.tail_of_string, &[2, 3, 4, 1]);
            assert_eq!(depth_1.wasted_symbols, 1);

            let depth_2 = depth_1.expand_one(0, F, N);
            assert_eq!(depth_2.permutations_seen.len(), 2);
            assert_eq!(depth_2.tail_of_string, &[3, 4, 1, 0]);
            assert_eq!(depth_2.wasted_symbols, 1);

            let depth_3 = depth_2.expand_one(4, F, N);
            assert_eq!(depth_3.permutations_seen.len(), 2);
            assert_eq!(depth_3.tail_of_string, &[1, 0, 4]);
            assert_eq!(depth_3.wasted_symbols, 2);

            let depth_4 = depth_3.expand_one(3, F, N);
            assert_eq!(depth_4.permutations_seen.len(), 2);
            assert_eq!(depth_4.tail_of_string, &[1, 0, 4, 3]);
            assert_eq!(depth_4.wasted_symbols, 3);

            let depth_5 = depth_4.expand_one(2, F, N);
            assert_eq!(depth_5.permutations_seen.len(), 3);
            assert_eq!(depth_5.tail_of_string, &[0, 4, 3, 2]);
            assert_eq!(depth_5.wasted_symbols, 3);
        }
    }
}

mod future_waste {
    use super::*;

    #[test]
    fn it_returns_how_many_additional_symbols_will_be_wasted_before_we_can_see_a_new_permutation() {
        let subject = Subject::seed(N);             //     01234
        assert_eq!(subject.future_waste(N), 0);     //       |
                                                    //       v
        let depth_1 = subject.expand_one(3, F, N);  //    012343ww   (2 wasted)
        assert_eq!(depth_1.future_waste(N), 2);     //       |
                                                    //       v
        let depth_2 = depth_1.expand_one(0, F, N);  //    0123430w
        assert_eq!(depth_2.future_waste(N), 1);     //       |
                                                    //       v
        let depth_3 = depth_2.expand_one(1, F, N);  //    01234301
        assert_eq!(depth_3.future_waste(N), 0);     //       |
                                                    //       v
        let depth_4 = depth_3.expand_one(1, F, N);  //  012343011www
        assert_eq!(depth_4.future_waste(N), 3);
    }
}

mod total_waste {
    use super::*;

    #[test]
    fn it_returns_the_total_number_of_wasted_symbols_there_will_be_before_we_see_a_new_permutation() {
        let subject = Subject::seed(N);             //     01234
        assert_eq!(subject.total_waste(N), 0);      //       |
                                                    //       v
        let depth_1 = subject.expand_one(3, F, N);  //    01234[3ww]   (3 wasted in total)
        assert_eq!(depth_1.total_waste(N), 3);      //       |
                                                    //       v
        let depth_2 = depth_1.expand_one(0, F, N);  //    01234[30w]
        assert_eq!(depth_2.total_waste(N), 3);      //       |
                                                    //       v
        let depth_3 = depth_2.expand_one(1, F, N);  //    01234[301]
        assert_eq!(depth_3.total_waste(N), 3);      //       |
                                                    //       v
        let depth_4 = depth_3.expand_one(1, F, N);  //  01234[3011www]
        assert_eq!(depth_4.total_waste(N), 7);
    }
}
