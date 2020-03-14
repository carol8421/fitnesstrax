use dimensioned::si::{Meter, Second};
use emseries::{DateTimeTz, Recordable};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ActivityType {
    Cycling,
    Rowing,
    Running,
    Swimming,
    Walking,
}

pub fn activity_types() -> Vec<ActivityType> {
    vec![
        ActivityType::Cycling,
        ActivityType::Rowing,
        ActivityType::Running,
        ActivityType::Swimming,
        ActivityType::Walking,
    ]
}

impl TryFrom<&str> for ActivityType {
    type Error = &'static str;

    fn try_from(inp: &str) -> Result<ActivityType, Self::Error> {
        match inp {
            "Cycling" => Ok(ActivityType::Cycling),
            "Rowing" => Ok(ActivityType::Rowing),
            "Running" => Ok(ActivityType::Running),
            "Swimming" => Ok(ActivityType::Swimming),
            "Walking" => Ok(ActivityType::Walking),
            _ => Err("invalid activity string"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeDistanceRecord {
    #[serde(rename = "date")]
    pub timestamp: DateTimeTz,
    pub activity: ActivityType,
    pub distance: Option<Meter<f64>>,
    pub duration: Option<Second<f64>>,
    pub comments: Option<String>,
}

impl TimeDistanceRecord {
    pub fn new(
        timestamp: DateTimeTz,
        activity: ActivityType,
        distance: Option<Meter<f64>>,
        duration: Option<Second<f64>>,
        comments: Option<String>,
    ) -> TimeDistanceRecord {
        TimeDistanceRecord {
            timestamp,
            activity,
            distance,
            duration,
            comments,
        }
    }
}

impl Recordable for TimeDistanceRecord {
    fn timestamp(&self) -> DateTimeTz {
        self.timestamp.clone()
    }

    fn tags(&self) -> Vec<String> {
        match self.activity {
            ActivityType::Cycling => vec![String::from("Cycling")],
            ActivityType::Rowing => vec![String::from("Rowing")],
            ActivityType::Running => vec![String::from("Running")],
            ActivityType::Swimming => vec![String::from("Swimming")],
            ActivityType::Walking => vec![String::from("Walking")],
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ActivityType, TimeDistanceRecord};
    use dimensioned::si::{M, S};

    #[test]
    pub fn deserialize_time_distance() {
        let cycling_track_str = "{\"distance\":12200,\"date\":\"2017-10-28T19:27:00Z\",\"activity\":\"Cycling\",\"comments\":null,\"duration\":3120}";
        let cycle_track: TimeDistanceRecord = serde_json::from_str(cycling_track_str).unwrap();
        assert_eq!(cycle_track.activity, ActivityType::Cycling);
        assert_eq!(cycle_track.distance, Some(12200. * M));
        assert_eq!(cycle_track.duration, Some(3120. * S));

        let running_track_str = "{\"distance\":3630,\"date\":\"2018-11-12T18:30:00Z\",\"activity\":\"Running\",\"comments\":null,\"duration\":1800}";
        let running_track: Result<TimeDistanceRecord, serde_json::Error> =
            serde_json::from_str(running_track_str);
        match running_track {
            Ok(track) => {
                assert_eq!(track.activity, ActivityType::Running);
                assert_eq!(track.distance, Some(3630. * M));
                assert_eq!(track.duration, Some(1800. * S));
            }
            Err(err) => panic!(err),
        }
    }
}
