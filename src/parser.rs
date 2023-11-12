use std::io::Read;

use anyhow::{Result, Error, anyhow};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use struson::reader::{JsonStreamReader, JsonReader};

use crate::model::TimelineRecord;

#[derive(Deserialize, Debug, PartialEq)]
struct JsonRecord {
    #[serde(rename = "latitudeE7")] 
    latitude_e7: Option<i32>,
    #[serde(rename = "longitudeE7")] 
    longitude_e7: Option<i32>,
    timestamp: Option<DateTime<Utc>>,
    accuracy: Option<u32>,
    source: Option<String>,

    #[serde(default)]
    activity: Vec<JsonActivity>
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonActivity {
    #[serde(rename = "activity")] 
    guesses: Vec<JsonActivityGuess>,
    timestamp: DateTime<Utc>,
}

///
/// E.g.
/// {
///   "type": "STILL",
///   "confidence": 96
/// }
/// 
#[derive(Deserialize, Debug, PartialEq)]
struct JsonActivityGuess {
    r#type: String,
    confidence: u8
}

impl TryInto<TimelineRecord> for JsonRecord {
    type Error = Error;

    fn try_into(self) -> std::result::Result<TimelineRecord, Self::Error> {
        let record = if let (Some(latitude_e7), Some(longitude_e7), Some(timestamp)) = (self.latitude_e7, self.longitude_e7, self.timestamp) {
            TimelineRecord {
                lat_e7: latitude_e7,
                lon_e7: longitude_e7,
                timestamp,
            }
        } else {
            return Err(anyhow!("Mandatory attribute missing: {:?}", self));
        };

        Ok(record)
    }
}

pub(crate) fn read_timeline_record(json_reader: &mut JsonStreamReader<impl Read>) -> Result<TimelineRecord> {
    let record: JsonRecord = json_reader.deserialize_next()?;
    record.try_into()
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn parse_valid_record_1() -> Result<()> {
        let json = r#"{
            "latitudeE7": 525163702,
            "longitudeE7": 133779641,
            "accuracy": 14,
            "timestamp": "2023-10-10T07:59:55Z"
        }"#;
        let mut json_reader = JsonStreamReader::new(json.as_bytes());

        assert_eq!(
            read_timeline_record(&mut json_reader).unwrap(),
            TimelineRecord {
                lat_e7: 525163702,
                lon_e7: 133779641,
                timestamp: Utc.with_ymd_and_hms(2023, 10, 10, 7, 59, 55).unwrap()
            }
        );

        Ok(())
    }

    #[test]
    fn parse_valid_record_2() -> Result<()> {
        let json = r#"{
            "latitudeE7": -785542265,
            "longitudeE7": -1732126463,
            "accuracy": 14,
            "timestamp": "2023-10-10T07:59:55Z"
        }"#;
        let mut json_reader = JsonStreamReader::new(json.as_bytes());

        assert_eq!(
            read_timeline_record(&mut json_reader).unwrap(),
            TimelineRecord {
                lat_e7: -785542265,
                lon_e7: -1732126463,
                timestamp: Utc.with_ymd_and_hms(2023, 10, 10, 7, 59, 55).unwrap()
            }
        );

        Ok(())
    }
}