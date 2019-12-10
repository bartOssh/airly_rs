#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoPoint {
    pub lat: f32,
    pub lng: f32,
}

impl GeoPoint {
    pub fn new(lat: f32, lng: f32) -> Self {
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
