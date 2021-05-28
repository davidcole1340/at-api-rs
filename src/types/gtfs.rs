//! Standardised GTFS types which are returned from the Auckland Transport API.

use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub id: String,
    pub trip_update: Option<TripUpdate>,
    pub vehicle: Option<VehiclePosition>,
    #[serde(default)]
    pub is_deleted: bool,
    // pub alert: Option<Alert>, // unused by AT
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TripUpdate {
    pub trip: TripDescriptor,
    pub vehicle: Option<VehicleDescriptor>,
    pub stop_time_update: Option<StopTimeUpdate>,
    pub timestamp: Option<u64>,
    pub delay: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopTimeUpdate {
    pub stop_sequence: Option<u32>,
    pub stop_id: Option<String>,
    pub arrival: Option<StopTimeEvent>,
    pub departure: Option<StopTimeEvent>,
    #[serde(default)]
    pub schedule_relationship: ScheduleRelationship,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopTimeEvent {
    pub delay: Option<i32>,
    pub time: Option<i64>,
    pub uncertainty: Option<i32>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum ScheduleRelationship {
    Scheduled = 0,
    Skipped = 1,
    NoData = 2,
}

impl Default for ScheduleRelationship {
    fn default() -> Self {
        Self::Scheduled
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehiclePosition {
    pub trip: Option<TripDescriptor>,
    pub vehicle: Option<VehicleDescriptor>,
    pub position: Option<Position>,
    pub current_stop_sequence: Option<u32>,
    pub stop_id: Option<String>,
    #[serde(default)]
    pub current_status: VehicleStopStatus,
    pub timestamp: Option<u64>,
    pub congestion_level: Option<CongestionLevel>,
    pub occupancy_status: Option<OccupancyStatus>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum VehicleStopStatus {
    // The vehicle is just about to arrive at the stop (on a stop display, the vehicle symbol
    // typically flashes).
    IncomingAt = 0,

    // The vehicle is standing at the stop.
    StoppedAt = 1,

    // The vehicle has departed and is in transit to the next stop.
    InTransitTo = 2,
}

impl Default for VehicleStopStatus {
    fn default() -> Self {
        Self::InTransitTo
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum CongestionLevel {
    UnknownCongestionLevel = 0,
    RunningSmoothly = 1,
    StopAndGo = 2,
    Congestion = 3,
    SevereCongestion = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum OccupancyStatus {
    Empty = 0,
    ManySeatsAvailable = 1,
    FewSeatsAvailable = 2,
    StandingRoomOnly = 3,
    CrushedStandingRoomOnly = 4,
    Full = 5,
    NotAcceptingPassengers = 6,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_bearing")]
    pub bearing: Option<f32>,
    pub odometer: Option<f64>,
    pub speed: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TripDescriptor {
    pub trip_id: Option<String>,
    pub route_id: Option<String>,
    pub direction_id: Option<u32>,
    pub start_time: Option<String>,
    pub start_date: Option<String>,
    pub schedule_relationship: Option<ScheduleRelationshipTripDescriptor>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum ScheduleRelationshipTripDescriptor {
    Scheduled = 0,
    Added = 1,
    Unscheduled = 2,
    Cancelled = 3,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleDescriptor {
    pub id: Option<String>,
    pub label: Option<String>,
    pub license_plate: Option<String>,
}

impl Entity {
    /// Returns the trip ID with the GTFS version truncated.
    pub fn trip_id(&self) -> Option<String> {
        Self::substr_to_char(self.trip_update.as_ref()?.trip.trip_id.as_ref()?, '-')
    }

    /// Returns the route ID with the GTFS version truncated.
    pub fn route_id(&self) -> Option<String> {
        Self::substr_to_char(self.trip_update.as_ref()?.trip.route_id.as_ref()?, '-')
    }

    /// Returns the current stop ID with the GTFS version truncated.
    pub fn stop_id(&self) -> Option<String> {
        Self::substr_to_char(
            self.trip_update
                .as_ref()?
                .stop_time_update
                .as_ref()?
                .stop_id
                .as_ref()?,
            '-',
        )
    }

    #[inline]
    fn substr_to_char<T: AsRef<str>>(str: T, c: char) -> Option<String> {
        let str = str.as_ref();
        Some(str.chars().take(str.find(c)?).collect())
    }
}

/// Serialize, Deserializes a bearing which is sent in the realtime GTFS output from Auckland Transport.
/// Requires a seperate deserialization function due to AT sending a float, integer, string or
/// nothing for this field.
pub fn deserialize_bearing<'de, D>(deserializer: D) -> std::result::Result<Option<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Bearing;

    impl<'de> serde::de::Visitor<'de> for Bearing {
        type Value = f32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("float, integer or string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse().map_err(serde::de::Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&v)
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let v: i16 = v.try_into().map_err(serde::de::Error::custom)?;
            Ok(v.into())
        }
    }

    Ok(deserializer.deserialize_any(Bearing).ok())
}
