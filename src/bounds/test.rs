use super::*;

type Subject = Bounds;
const N: usize = 5;

mod new {
    use super::*;

    #[test]
    fn it_sets_the_first_lower_bound_to_zero_and_upper_bound_to_factorial_of_n() {
        let subject = Subject::new(N);

        assert_eq!(subject.lower_bounds, &[0]);
        assert_eq!(subject.upper_bounds, &[120]);
    }

    #[test]
    fn it_sets_the_first_threshold_to_zero() {
        let subject = Subject::new(N);

        assert_eq!(subject.thresholds, &[0]);
    }
}

mod update {
    use super::*;

    #[test]
    fn it_increases_the_lower_bound_for_the_given_index() {
        let mut subject = Subject::new(N);

        subject.update(0, 1);
        assert_eq!(subject.lower_bounds, &[1]);

        subject.update(0, 2);
        assert_eq!(subject.lower_bounds, &[2]);
    }

    #[test]
    fn it_returns_true_if_the_bounds_updated() {
        let mut subject = Subject::new(N);

        assert_eq!(subject.update(0, 0), false);
        assert_eq!(subject.update(0, 1), true);
        assert_eq!(subject.update(0, 1), false);
        assert_eq!(subject.update(0, 2), true);
        assert_eq!(subject.update(0, 3), true);
    }

    mod when_the_index_is_larger_than_the_array {
        use super::*;

        #[test]
        fn it_increases_the_length_of_the_array() {
            let mut subject = Subject::new(N);

            subject.update(1, 0);
            assert_eq!(subject.lower_bounds, &[0, 0]);

            subject.update(2, 0);
            assert_eq!(subject.lower_bounds, &[0, 0, 0]);
        }

        #[test]
        fn it_sets_the_lower_bound() {
            let mut subject = Subject::new(N);

            subject.update(1, 5);
            assert_eq!(subject.lower_bounds, &[0, 5]);

            subject.update(2, 7);
            assert_eq!(subject.lower_bounds, &[0, 5, 7]);
        }

        #[test]
        fn it_reuses_the_previous_lower_bound_if_it_is_larger() {
            let mut subject = Subject::new(N);

            subject.update(0, 5);
            assert_eq!(subject.lower_bounds, &[5]);

            subject.update(1, 3);
            assert_eq!(subject.lower_bounds, &[5, 5]);

            subject.update(1, 6);
            assert_eq!(subject.lower_bounds, &[5, 6]);

            subject.update(4, 4);
            assert_eq!(subject.lower_bounds, &[5, 6, 6, 6, 6]);
        }

        #[test]
        fn it_returns_true() {
            let mut subject = Subject::new(N);

            subject.update(0, 5);

            assert_eq!(subject.update(1, 3), true);
            assert_eq!(subject.update(2, 3), true);
        }

        #[test]
        fn it_fixes_the_upper_bound_for_the_previous_indexes() {
            let mut subject = Subject::new(N);

            subject.update(0, 5);
            assert_eq!(subject.upper_bounds[0], 120);

            subject.update(1, 3);
            assert_eq!(subject.upper_bounds[0], 5);

            subject.update(3, 3);
            assert_eq!(subject.upper_bounds[..3], [5, 5, 5]);
        }

        #[test]
        fn it_sets_the_upper_bound_to_its_lower_bound_plus_the_first_upper_bound() {
            let mut subject = Subject::new(N);

            subject.update(0, 5);
            assert_eq!(subject.lower_bounds, &[5]);
            assert_eq!(subject.upper_bounds, &[120]);

            subject.update(1, 3);
            assert_eq!(subject.lower_bounds, &[5, 5]);
            assert_eq!(subject.upper_bounds, &[5, 10]);

            subject.update(3, 7);
            assert_eq!(subject.lower_bounds, &[5, 5, 7, 7]);
            assert_eq!(subject.upper_bounds, &[5, 5, 7, 12]);
        }

        #[test]
        fn it_sets_the_threshold_to_the_lower_bound_minus_first_upper_bound() {
            let mut subject = Subject::new(N);

            subject.update(0, 5);
            assert_eq!(subject.thresholds, &[0]);

            subject.update(1, 7);
            assert_eq!(subject.thresholds, &[0, 2]);

            subject.update(1, 8);
            assert_eq!(subject.thresholds, &[0, 3]);

            subject.update(3, 9);
            assert_eq!(subject.lower_bounds, &[5, 8, 9, 9]);
            assert_eq!(subject.upper_bounds, &[5, 8, 9, 14]);
            assert_eq!(subject.thresholds,   &[0, 3, 4, 4]);
        }
    }
}

mod upper {
    use super::*;

    #[test]
    fn it_returns_the_upper_bound_for_the_index_or_max() {
        let mut subject = Subject::new(N);

        subject.update(0, 5);
        subject.update(2, 5);

        assert_eq!(subject.upper_bounds, &[5, 5, 10]);

        assert_eq!(subject.upper(0), 5);
        assert_eq!(subject.upper(1), 5);
        assert_eq!(subject.upper(2), 10);
        assert_eq!(subject.upper(3), 120);
    }
}

mod found_for_superpermutation {
    use super::*;

    #[test]
    fn it_returns_true_if_the_last_lower_bound_is_equal_to_factorial_n() {
        let mut subject = Subject::new(N);
        assert_eq!(subject.found_for_superpermutation(), false);

        subject.update(0, 119);
        assert_eq!(subject.found_for_superpermutation(), false);

        subject.update(0, 120);
        assert_eq!(subject.found_for_superpermutation(), true);
    }
}
