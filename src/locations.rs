use serde::{Deserialize, Serialize};

use crate::Job;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

impl Location {
    pub fn new_random() -> Self {
        Self {
           lat: rand::random(),
           lon: rand::random(),
        }
    }
}

const R: f32 = 6371.0;

pub fn haversine_dist(loc1: &Location, loc2: &Location) -> f32 {
    let d_lat = (loc2.lat - loc1.lat).to_radians();
    let d_lon = (loc2.lon - loc2.lat).to_radians();

    let lat1 = loc1.lat.to_radians();
    let lat2 = loc2.lat.to_radians();

    let a = (d_lat/2.0).sin().powf(2.0) + (d_lon/2.0).sin().powf(2.0) * (lat1.cos()) * (lat2.cos());
    let c = 2.0 * ((a.sqrt()).atan2((1.0-a).sqrt()));

    R * c
}

pub fn driving_time(job1: &Job, job2: &Job) -> f32 {
    haversine_dist(&job1.location, &job2.location) / 25.0
}
