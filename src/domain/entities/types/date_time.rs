use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct DateTime(pub SystemTime);


impl Default for DateTime {
    fn default() -> Self {
        DateTime(SystemTime::now())
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration_since_epoch = self.0.duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        let seconds = duration_since_epoch.as_secs();
        serializer.serialize_u64(seconds)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = u64::deserialize(deserializer)?;
        let system_time = UNIX_EPOCH + Duration::from_secs(seconds);
        Ok(DateTime(system_time))
    }
}