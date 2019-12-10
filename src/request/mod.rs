#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoPoint {
    pub lat: f64,
    pub lng: f64,
}

impl GeoPoint {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoCircle {
    pub point: GeoPoint,
    pub radius_km: i32,
}

impl GeoCircle {
    pub fn new(point: GeoPoint, radius_km: i32) -> Self {
        Self { point, radius_km }
    }
}
