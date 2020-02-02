use std::io::{Error, ErrorKind};

const ERR_OUT_OF_BOUNDS: &str = "Value of passed argument out of bounds";
const MAX_EARTH_RADIUS_KM: u32 = 6371;
const MAX_LNG: f32 = 180.0;
const MAX_LAT: f32 = 90.0;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GeoPoint {
    #[serde(rename = "latitude")]
    lat: f32,
    #[serde(rename = "longitude")]
    lng: f32,
}

impl GeoPoint {
    /// Creates new GeoPoint if passes arguments validation
    /// 
    /// # Arguments:
    /// 
    /// * lat - latitude
    /// * lng - longitude
    /// 
    /// # Returns GeoPoint struct if validation passed Error otherwise
    /// 
    pub fn new(lat: f32, lng: f32) -> Result<Self, Box<dyn std::error::Error>> {
        if lat.abs() <= MAX_LAT && lng.abs() <= MAX_LNG {
            return Ok(Self { lat, lng });
        }
        Err(Box::new(Error::new(
            ErrorKind::Other,
            format!(
                "{}, expected values for lat max: +/- {} and lng max: +/- {}, got values for lat: {} and lng: {}",
                ERR_OUT_OF_BOUNDS, MAX_LAT, MAX_LNG, lat, lng
            )
        )))
    }

    /// Getter for latitude value
    /// 
    /// # Returns latitude
    /// 
    pub fn get_lat(self) -> f32 {
        self.lat
    }

    /// Getter for longitude value
    /// 
    /// # Returns longitude
    /// 
    pub fn get_lng(self) -> f32 {
        self.lng
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GeoCircle {
    point: GeoPoint,
    radius_km: u32,
}

impl GeoCircle {
    /// Creates new GeoCircle if passes arguments validation
    /// 
    /// # Arguments:
    /// 
    /// * point - localization on the planet Earth
    /// * radius_km - radius in km to collect data form
    /// 
    /// # Returns GeoCircle struct if validation passed Error otherwise
    /// 
    pub fn new(point: GeoPoint, radius_km: u32) -> Result<Self, Box<dyn std::error::Error>> {
        if radius_km < MAX_EARTH_RADIUS_KM {
            return Ok(Self { point, radius_km })
        }
        Err(Box::new(Error::new(
            ErrorKind::Other,
            format!(
                "{}, expected radius max value: {}, got radius value: {}",
                ERR_OUT_OF_BOUNDS, MAX_EARTH_RADIUS_KM, radius_km
            )
        )))
    }

    /// Getter for point struct copy
    /// 
    /// # Returns GeoPoint copy
    /// 
    pub fn get_point(self) -> GeoPoint {
        self.point.clone()
    }

    /// Getter for point struct copy
    /// 
    /// # Returns GeoPoint copy
    /// 
    pub fn get_radius_km(self) -> u32 {
        self.radius_km
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub country: String,
    pub city: String,
    pub street: String,
    pub number: String,
    #[serde(rename = "displayAddress1")]
    pub display_address1: Option<String>,
    #[serde(rename = "displayAddress2")]
    pub display_address2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sponsor {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Installation {
    /// ID of the installation
    pub id: i32,
    /// Location latitude and longitude
    pub location: GeoPoint,
    /// Address on which installation is registered
    pub address: Address,
    /// Elevation over the sea level
    pub elevation: f64,
    /// Indicates if this is Airly sensor
    pub airly: bool,
    /// Sponsor name if present
    pub sponsor: Sponsor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    /// Name of this measurement
    pub name: Option<String>,
    /// Value of this measurement
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    /// Name of this index
    pub name: Option<String>,
    /// Index numerical value
    pub value: Option<f64>,
    /// Index level name
    pub level: Option<String>,
    /// Text describing this air quality level. Text translation is returned according to language specified in the request (English being default)
    pub description: Option<String>,
    /// Piece of advice from Airly regarding air quality. Text translation is returned according to language specified in the request (English being default)
    pub advice: Option<String>,
    /// Color representing this index level, given by hexadecimal css-style triplet
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Standard {
    /// Name of this standard
    pub name: Option<String>,
    /// Pollutant described by this standard
    pub pollutant: Option<String>,
    /// Limit value of the pollutant
    pub limit: Option<f64>,
    /// Pollutant measurement as percent of allowable limit
    pub percent: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AveragedValues {
    /// Left bound of the time period over which average measurements were calculated, inclusive, always UTC
    #[serde(rename = "fromDateTime")]
    pub from_date_time: Option<String>,
    /// Right bound of the time period over which average measurements were calculated, exclusive, always UTC
    #[serde(rename = "tillDateTime")]
    pub till_date_time: Option<String>,
    /// List of raw measurements, averaged over specified period. Measurement types available in this list depend on the capabilities of the queried installation, e.g. particulate matter (PM1, PM25, PM10), gases (CO, NO2, SO2, O3) or weather conditions (temperature, humidity, pressure)
    pub values: Vec<Value>,
    /// List of indexes calculated from the values available. Indexes are defined by relevant national and international institutions, e.g. EU, GIOŚ or US EPA
    pub indexes: Vec<Index>,
    /// List of 'standard' values, or 'limits' for pollutants that should not be exceeded over certain period of time. Limits are defined by relevant national and international institutions, like e.g. WHO or EPA. For each standard limit in this list there is also a corresponding measurement expressed as a percent value of the limit
    pub standards: Vec<Standard>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurements {
    pub current: Option<AveragedValues>,
    pub history: Vec<AveragedValues>,
    pub forecast: Vec<AveragedValues>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexType {
    /// Name of this index
    pub name: Option<String>,
    /// List of possible index levels
    pub level: Option<IndexLevel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexLevel {
    /// Minimum index value for this level
    #[serde(rename = "minValue")]
    pub min_value: Option<i32>,
    /// Maximum index value for this level
    #[serde(rename = "mixValue")]
    pub max_value: Option<i32>,
    /// Values range for this index level
    pub value: Option<String>,
    /// Name of this index level
    pub level: Option<String>,
    /// Text describing this index level
    pub description: Option<String>,
    /// Color representing this index level, given by hexadecimal css-style triplet
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeasurementType {
    /// Short name of this measurement type. This is a translated field and will contain value according to Access-Language header,
    pub name: Option<String>,
    /// Short name of this measurement type. This is a translated field and will contain value according to Access-Language header,
    pub label: Option<String>,
    /// Unit of this measurement type
    pub unit: Option<String>,
}
