use super::*;
use std::fs::metadata;

type Subject = Disk;

const PATH: &'static str = "/tmp/superpermutation-test";

fn subject(test_id: &'static str, gzip: bool) -> Subject {
    let path = format!("{}/{}", PATH, test_id);
    Subject::new(path, gzip)
}

mod new {
    use super::*;

    #[test]
    fn it_builds_the_struct_with_the_path() {
        let subject = subject("test-1", false);
        assert_eq!(subject.path, "/tmp/superpermutation-test/test-1");
    }

    #[test]
    fn it_creates_a_directory_at_the_path() {
        let subject = subject("test-2", false);
        assert_eq!(Path::new(PATH).exists(), true);
    }
}

mod filename {
    use super::*;

    #[test]
    fn it_returns_a_filename_based_on_the_number_of_wasted_symbols_and_permutations() {
        let subject = subject("test-3", false);
        let actual = subject.filename(3, 4);

        let basename = "test-3/candidates-with-3-wasted-symbols-and-4-permutations";
        let expected = format!("{}/{}", PATH, basename);

        assert_eq!(actual, expected);
    }
}

mod write {
    use super::*;

    #[test]
    fn it_writes_the_bucket_to_a_file() {
        let candidate = Candidate::seed(5);
        let bucket = VecDeque::from(vec![candidate]);

        let subject = subject("test-4", false);
        subject.write(&bucket, 3, 4);

        let filename = subject.filename(3, 4);;
        assert_eq!(Path::new(&filename).exists(), true);
    }
}

mod read {
    use super::*;

    #[test]
    fn it_reads_the_bucket_from_a_file() {
        let candidate = Candidate::seed(5);
        let bucket = VecDeque::from(vec![candidate]);

        let subject = subject("test-5", false);
        subject.write(&bucket, 3, 4);

        let bucket_from_file = subject.read(3, 4);
        assert_eq!(bucket_from_file, bucket);
    }
}

mod gzip_compression {
    use super::*;

    #[test]
    fn it_writes_a_smaller_file_to_disk() {
        let bucket: VecDeque<_> = (0..1000)
            .map(|_| Candidate::seed(5)).collect();

        let with_gzip = subject("test-6", true);
        let without_gzip = subject("test-7", false);

        with_gzip.write(&bucket, 3, 4);
        without_gzip.write(&bucket, 5, 6);

        let file1 = with_gzip.filename(3, 4);
        let file2 = without_gzip.filename(5, 6);

        let with_gzip_size = metadata(file1).unwrap().len();
        let without_gzip_size = metadata(file2).unwrap().len();

        assert_eq!(with_gzip_size > 0, true);
        assert_eq!(with_gzip_size < without_gzip_size, true);

        let compression_rate = without_gzip_size / with_gzip_size;
        assert_eq!(compression_rate > 200, true);
    }
}
