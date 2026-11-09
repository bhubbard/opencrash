//! Core traffic collision severity, spatial distance, intersection hazard brief, and police agency types.

use serde::{Deserialize, Serialize};

/// Mean Earth radius in meters used for haversine distance calculations.
pub const EARTH_RADIUS_METERS: f64 = 6_371_008.8;

/// KABCO / SWITRS collision injury severity scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CollisionSeverity {
    /// Property damage only (PDO).
    PropertyDamageOnly,
    /// Complaint of pain / possible injury.
    PossibleInjury,
    /// Other visible injury.
    VisibleInjury,
    /// Severe / incapacitating injury.
    SevereInjury,
    /// Fatal collision (at least one fatality within 30 days).
    Fatal,
}

/// Summary hazard statistics for an intersection or road corridor (used in PI intake & demand packets).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntersectionHazardBrief {
    /// Primary road name (e.g., `"WILSHIRE BLVD"`).
    pub primary_road: String,
    /// Secondary cross street or postmile (e.g., `"WESTERN AVE"`).
    pub secondary_road: String,
    /// Search radius in meters around the intersection centroid.
    pub radius_meters: u32,
    /// Total injury and fatal collisions in the trailing 5-year window.
    pub injury_collisions_5yr: u32,
    /// Total fatal collisions in the trailing 5-year window.
    pub fatal_collisions_5yr: u32,
    /// Total severe injury collisions in the trailing 5-year window.
    pub severe_injury_collisions_5yr: u32,
    /// County-wide hazard percentile (`0.0..=100.0`).
    pub county_hazard_percentile: f32,
    /// Law enforcement agency responsible for Traffic Collision Reports (TCR) at this location.
    pub reporting_agency: String,
}

/// Computes the great-circle haversine distance in meters between two `(lat, lon)` points in decimal degrees.
pub fn haversine_meters(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let r_lat1 = lat1.to_radians();
    let r_lat2 = lat2.to_radians();

    let a = (d_lat * 0.5).sin().powi(2) + r_lat1.cos() * r_lat2.cos() * (d_lon * 0.5).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_METERS * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_haversine_distance_accurately() {
        // Wilshire & Western to Wilshire & Normandie (~800m in Los Angeles)
        let d = haversine_meters(34.0617, -118.3090, 34.0617, -118.3003);
        assert!((d - 801.0).abs() < 15.0, "expected ~801m, got {d}");
    }
}
