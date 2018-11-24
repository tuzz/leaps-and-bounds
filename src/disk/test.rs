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
        subject("test-2", false);
        assert_eq!(Path::new(PATH).exists(), true);
    }
}

mod basename {
    use super::*;

    #[test]
    fn it_returns_a_name_based_on_the_number_of_wasted_symbols_and_permutations() {
        let subject = subject("test-3", false);
        let actual = subject.basename(3, 4);

        let name = "test-3/candidates-with-3-wasted-symbols-and-4-permutations.dat";
        let expected = format!("{}/{}", PATH, name);

        assert_eq!(actual, expected);
    }
}

mod filename_for_reading {
    use super::*;

    #[test]
    fn it_returns_none_if_no_file_exists() {
        let subject = subject("test-4", false);
        let filename = subject.filename_for_reading(3, 4);

        assert_eq!(filename, None);
    }

    #[test]
    fn it_returns_the_name_of_the_first_available_file() {
        let candidate = Candidate::seed(5);
        let bucket = VecDeque::from(vec![candidate]);

        let subject = subject("test-5", false);

        subject.write(&bucket, 3, 4); // 0
        subject.write(&bucket, 3, 4); // 1
        subject.write(&bucket, 3, 4); // 2

        let filename = subject.filename_for_reading(3, 4).unwrap();
        assert_eq!(&filename[70..], "-4-permutations.dat.0");

        let filename = subject.filename_for_reading(3, 4).unwrap();
        assert_eq!(&filename[70..], "-4-permutations.dat.0");

        subject.read(3, 4); // delete 0

        let filename = subject.filename_for_reading(3, 4).unwrap();
        assert_eq!(&filename[70..], "-4-permutations.dat.1");

        let filename = subject.filename_for_reading(3, 4).unwrap();
        assert_eq!(&filename[70..], "-4-permutations.dat.1");

        subject.read(3, 4); // delete 1

        let filename = subject.filename_for_reading(3, 4).unwrap();
        assert_eq!(&filename[70..], "-4-permutations.dat.2");
    }
}

mod filename_for_writing {
    use super::*;

    #[test]
    fn it_adds_a_suffix_to_the_basename() {
        let subject = subject("test-6", false);

        let filename = subject.filename_for_writing(3, 4);
        assert_eq!(&filename[70..], "-4-permutations.dat.0");
    }

    #[test]
    fn it_increments_the_index_each_time_a_file_is_written() {
        let candidate = Candidate::seed(5);
        let bucket = VecDeque::from(vec![candidate]);

        let subject = subject("test-7", false);
        let filename = subject.filename_for_writing(3, 4);
        assert_eq!(&filename[70..], "-4-permutations.dat.0");

        subject.write(&bucket, 3, 4);
        let filename = subject.filename_for_writing(3, 4);
        assert_eq!(&filename[70..], "-4-permutations.dat.1");

        subject.write(&bucket, 3, 4);
        let filename = subject.filename_for_writing(3, 4);
        assert_eq!(&filename[70..], "-4-permutations.dat.2");
    }
}

mod write {
    use super::*;

    #[test]
    fn it_writes_the_bucket_to_a_file() {
        let candidate = Candidate::seed(5);
        let bucket = VecDeque::from(vec![candidate]);

        let subject = subject("test-8", false);
        subject.write(&bucket, 3, 4);

        let filename = subject.filename_for_reading(3, 4).unwrap();
        assert_eq!(Path::new(&filename).exists(), true);
    }
}

mod read {
    use super::*;

    #[test]
    fn it_reads_the_bucket_from_a_file() {
        let candidate = Candidate::seed(5);
        let bucket = VecDeque::from(vec![candidate]);

        let subject = subject("test-9", false);
        subject.write(&bucket, 3, 4);

        let bucket_from_file = subject.read(3, 4);
        assert_eq!(bucket_from_file, Some(bucket));
    }

    #[test]
    fn it_deletes_the_file() {
        let candidate = Candidate::seed(5);
        let bucket = VecDeque::from(vec![candidate]);

        let subject = subject("test-10", false);
        subject.write(&bucket, 3, 4);

        let filename = subject.filename_for_reading(3, 4).unwrap();
        assert_eq!(Path::new(&filename).exists(), true);

        subject.read(3, 4);
        assert_eq!(Path::new(&filename).exists(), false);
    }
}

mod gzip_compression {
    use super::*;

    #[test]
    fn it_writes_a_smaller_file_to_disk() {
        let bucket: VecDeque<_> = (0..1000)
            .map(|_| Candidate::seed(5)).collect();

        let with_gzip = subject("test-11", true);
        let without_gzip = subject("test-12", false);

        with_gzip.write(&bucket, 3, 4);
        without_gzip.write(&bucket, 5, 6);

        let file1 = with_gzip.filename_for_reading(3, 4).unwrap();
        let file2 = without_gzip.filename_for_reading(5, 6).unwrap();

        let with_gzip_size = metadata(file1).unwrap().len();
        let without_gzip_size = metadata(file2).unwrap().len();

        assert_eq!(with_gzip_size > 0, true);
        assert_eq!(with_gzip_size < without_gzip_size, true);

        let compression_rate = without_gzip_size / with_gzip_size;
        assert_eq!(compression_rate > 200, true);
    }
}
