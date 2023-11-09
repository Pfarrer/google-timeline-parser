use std::io::Read;

use anyhow::Result;
pub use model::{TimelineRecord, TimelineRecordsIter};
use parser::read_timeline_record;
use struson::{
    json_path,
    reader::{JsonReader, JsonStreamReader},
};

mod model;
mod parser;

///
/// Examples:
/// 
/// ```rust
/// use google_timeline_parser::read_timeline_records;
/// 
/// # fn main() -> anyhow::Result<()> {
/// let json = r#"{
///   "locations": [{
///     "latitudeE7": 525163702,
///     "longitudeE7": 133779641,
///     "accuracy": 14,
///     "timestamp": "2023-10-10T07:59:55Z"
///   }]
/// }"#;
/// 
/// let mut records_iter = read_timeline_records(json.as_bytes())?;
/// assert!(records_iter.next().is_some());
/// assert!(records_iter.next().is_none());
/// # Ok(())
/// # }
/// ```
pub fn read_timeline_records(reader: impl Read) -> Result<TimelineRecordsIter<impl Read>> {
    let mut json_reader = JsonStreamReader::new(reader);
    json_reader.seek_to(&json_path!["locations"])?;
    json_reader.begin_array()?;

    Ok(TimelineRecordsIter { json_reader })
}

impl<R: Read> Iterator for TimelineRecordsIter<R> {
    type Item = TimelineRecord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.json_reader.has_next().unwrap_or(false) {
            let result = read_timeline_record(&mut self.json_reader);
            if let Ok(record) = result {
                return Some(record)
            }
            else {
                return self.next();
            }
        } else {
            return None;
        }
    }
}
