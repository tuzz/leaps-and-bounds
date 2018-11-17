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
