use chrono::Utc;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::{alloc::System, time::SystemTime};

use crate::models::{dog_model::Dog, owner_model::Owner};

#[derive(Debug, Deserialize, Serialize)]
pub struct Booking {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_in_minutes: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBooking {
    pub _id: ObjectId,
    pub owner: Owner,
    pub dogs: Vec<Dog>,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: BookingRequest) -> Result<Self, Self::Error> {
        let chono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.start_time)
            .map_err(|err| format!("Format to parse start_time: {} ", err))?
            .with_timezone(&Utc)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).expect("Failed to parse owner"),
            duration_in_minutes: item.duration_in_minutes,
            start_time: DateTime::from(chono_datetime),
            cancelled: false,
        })
    }
}
