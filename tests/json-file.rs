use google_timeline_parser::read_timeline_records;

#[test]
fn parse_multiple_locations_from_string() {
    let json = r#"{
  "locations": [{
    "latitudeE7": 525163702,
    "longitudeE7": 133779641,
    "timestamp": "2023-10-10T07:59:55Z"
  },{
    "latitudeE7": 488583736,
    "longitudeE7": 22919064,
    "timestamp": "2023-10-10T07:59:55Z"
  }]
}"#;
    let mut records_iter = read_timeline_records(json.as_bytes()).unwrap();

    assert!(records_iter.next().is_some());
    assert!(records_iter.next().is_some());
    assert!(records_iter.next().is_none());
}

#[test]
fn recovers_after_broken_item() {
    let json = r#"{
  "locations": [{}, {
    "latitudeE7": 488583736,
    "longitudeE7": 22919064,
    "timestamp": "2023-10-10T07:59:55Z"
  }]
}"#;

    let mut records_iter = read_timeline_records(json.as_bytes()).unwrap();
    assert!(records_iter.next().is_some());
    assert!(records_iter.next().is_none());
}
