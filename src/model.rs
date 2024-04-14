use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq)]
pub struct TimelineRecord {
    pub lat_e7: i32,
    pub lon_e7: i32,
    pub accuracy: Option<u32>,
    pub timestamp: DateTime<Utc>,
    pub activity: TimelineActivity,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimelineActivity {
    Unknown,
    Still,
    OnFoot,
    OnBike,
    InRoadVehicle,
    InRailVehicle,
}

impl TimelineRecord {
    pub fn lat(&self) -> f32 {
        self.lat_e7 as f32 / 1e7
    }
    pub fn lon(&self) -> f32 {
        self.lon_e7 as f32 / 1e7
    }
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn lat_and_lon() {
        let record = TimelineRecord {
            lat_e7: 525163702,
            lon_e7: 133779641,
            accuracy: Some(100),
            timestamp: Utc.with_ymd_and_hms(2023, 10, 10, 7, 59, 55).unwrap(),
            activity: TimelineActivity::Unknown,
        };

        assert_float_eq!(record.lat(), 52.5163702, r2nd <= f32::EPSILON);
        assert_float_eq!(record.lon(), 13.3779641, r2nd <= f32::EPSILON);
    }
}
