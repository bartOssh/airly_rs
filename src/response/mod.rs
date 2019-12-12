#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
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
    pub location: Location,
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
