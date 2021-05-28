//! Types and structures returned from the Auckland Transport API.

pub mod gtfs;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

use self::gtfs::Entity;

#[derive(Debug, Deserialize, Clone)]
pub struct ATResponse {
    pub status: String,
    pub response: Response,
    pub error: Option<()>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    pub header: Header,
    pub entity: Vec<Entity>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Header {
    pub gtfs_realtime_version: String,
    #[serde(default)]
    pub incrementality: Incrementality,
    pub timestamp: Option<f64>,
}

#[derive(Debug, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum Incrementality {
    FullDataset = 0,
    Differential = 1,
}

impl Default for Incrementality {
    fn default() -> Self {
        Self::FullDataset
    }
}
