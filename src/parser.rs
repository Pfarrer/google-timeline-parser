use std::io::Read;

use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use struson::reader::{JsonReader, JsonStreamReader};

use crate::model::{TimelineActivity, TimelineRecord};

#[derive(Deserialize, Debug, PartialEq)]
struct JsonRecord {
    #[serde(rename = "latitudeE7")]
    latitude_e7: Option<i32>,
    #[serde(rename = "longitudeE7")]
    longitude_e7: Option<i32>,
    timestamp: Option<DateTime<Utc>>,
    accuracy: Option<u32>,

    #[serde(default)]
    activity: Vec<JsonActivity>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonActivity {
    #[serde(rename = "activity")]
    guesses: Vec<JsonActivityGuess>,
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
    confidence: u8,
}

impl TryInto<TimelineRecord> for JsonRecord {
    type Error = Error;

    fn try_into(self) -> std::result::Result<TimelineRecord, Self::Error> {
        match (self.latitude_e7, self.longitude_e7, self.timestamp) {
            (Some(latitude_e7), Some(longitude_e7), Some(timestamp)) => Ok(TimelineRecord {
                lat_e7: latitude_e7,
                lon_e7: longitude_e7,
                accuracy: self.accuracy,
                timestamp,
                activity: self.activity.into(),
            }),
            _ => Err(anyhow!("Mandatory attribute missing: {:?}", self)),
        }
    }
}

impl Into<TimelineActivity> for Vec<JsonActivity> {
    fn into(self) -> TimelineActivity {
        self.last()
            .and_then(|act| {
                act.guesses
                    .iter()
                    .filter(|guess| guess.confidence > 90)
                    .filter_map(|guess| match guess.r#type.as_str() {
                        "STILL" => Some(TimelineActivity::Still),
                        "WALKING" | "ON_FOOT" => Some(TimelineActivity::OnFoot),
                        "ON_BICYCLE" => Some(TimelineActivity::OnBike),
                        "IN_ROAD_VEHICLE" => Some(TimelineActivity::InRoadVehicle),
                        "IN_RAIL_VEHICLE" => Some(TimelineActivity::InRoadVehicle),
                        "UNKNOWN" | "IN_VEHICLE" | "EXITING_VEHICLE" | "TILTING" => None,
                        a => panic!("Unmapped activity type: {}", a),
                    })
                    .next()
            })
            .unwrap_or(TimelineActivity::Unknown)
    }
}

pub(crate) fn read_timeline_record(
    json_reader: &mut JsonStreamReader<impl Read>,
) -> Result<TimelineRecord> {
    let record: JsonRecord = json_reader.deserialize_next()?;
    record.try_into()
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn parse_minimal_valid_record() -> Result<()> {
        let json = r#"{
            "latitudeE7": 525163702,
            "longitudeE7": 133779641,
            "timestamp": "2023-10-10T07:59:55Z"
        }"#;
        let mut json_reader = JsonStreamReader::new(json.as_bytes());

        assert_eq!(
            read_timeline_record(&mut json_reader).unwrap(),
            TimelineRecord {
                lat_e7: 525163702,
                lon_e7: 133779641,
                accuracy: None,
                activity: TimelineActivity::Unknown,
                timestamp: Utc.with_ymd_and_hms(2023, 10, 10, 7, 59, 55).unwrap()
            }
        );

        Ok(())
    }

    #[test]
    fn parse_valid_record_1() -> Result<()> {
        let json = r#"{
            "latitudeE7": 525163702,
            "longitudeE7": 133779641,
            "accuracy": 14,
            "activity": [{
                "activity": [{
                  "type": "WALKING",
                  "confidence": 41
                }],
                "timestamp": "2023-10-10T07:59:55Z"
            }],
            "timestamp": "2023-10-10T07:59:55Z"
        }"#;
        let mut json_reader = JsonStreamReader::new(json.as_bytes());

        assert_eq!(
            read_timeline_record(&mut json_reader).unwrap(),
            TimelineRecord {
                lat_e7: 525163702,
                lon_e7: 133779641,
                accuracy: Some(14),
                activity: TimelineActivity::Unknown,
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
            "timestamp": "2023-10-10T07:59:55Z",
            "accuracy": 100,
            "activity": [
                {
                    "activity": [
                        {
                            "type": "STILL",
                            "confidence": 96
                        }, {
                            "type": "ON_FOOT",
                            "confidence": 2
                        }, {
                            "type": "WALKING",
                            "confidence": 2
                        }
                    ],
                    "timestamp": "2023-10-10T08:43:58.954Z"
                },
                {
                    "activity": [
                        {
                            "type": "STILL",
                            "confidence": 100
                        }
                    ],
                    "timestamp": "2023-10-10T08:46:02.957Z"
                }
            ]
        }"#;
        let mut json_reader = JsonStreamReader::new(json.as_bytes());

        assert_eq!(
            read_timeline_record(&mut json_reader).unwrap(),
            TimelineRecord {
                lat_e7: -785542265,
                lon_e7: -1732126463,
                accuracy: Some(100),
                activity: TimelineActivity::Still,
                timestamp: Utc.with_ymd_and_hms(2023, 10, 10, 7, 59, 55).unwrap()
            }
        );

        Ok(())
    }
}
