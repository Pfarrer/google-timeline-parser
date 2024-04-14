use std::fs::File;

use google_timeline_parser::read_timeline_records_from_archive;
use zip::ZipArchive;

#[test]
fn read_timeline_records_from_archive_with_test_archive() {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let archive_file =
        File::open(format!("{}/tests/data/takeout-german.zip", project_root)).unwrap();
    let mut archive = ZipArchive::new(archive_file).unwrap();
    let mut records_iter = read_timeline_records_from_archive(&mut archive).unwrap();

    assert!(records_iter.next().is_some());
    assert!(records_iter.next().is_some());
    assert!(records_iter.next().is_none());
}
