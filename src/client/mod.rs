mod endpoints;
use crate::types;
use reqwest;
use reqwest::{
    header::{HeaderName, HeaderValue, ACCEPT, ACCEPT_LANGUAGE},
    Response,
};
use std::io::{Error, ErrorKind};

const API_KEY_LEN: usize = 32;
const ERR_API_KEY: &str = "Wrong api key length";

enum IncludeWind {
    YES,
    NO
}

#[derive(Debug, Clone)]
pub struct AirlyClient {
    api_key: HeaderValue,
    client: reqwest::Client,
}

impl AirlyClient {
    /// Constructs AirlyClient
    ///
    /// # Arguments:
    /// * api_key - personal api key that can be obtained from https://developer.airly.eu/login
    ///
    /// # Returns instance of AirlyClient struct if api_key of correct length Error otherwise
    ///
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        if api_key.len() == API_KEY_LEN {
            let client = reqwest::Client::new();
            let _api_key = HeaderValue::from_str(&api_key);
            if let Ok(api_key) = _api_key {
                return Ok(Self { api_key, client });
            }
        }
        Err(Box::new(Error::new(
            ErrorKind::Other,
            format!("{}, expected: {}, got: {}", ERR_API_KEY, API_KEY_LEN, &api_key.len()),
        )))
    }

    /// Get installation properties for given id
    ///
    /// # Arguments:
    /// * id - id of installation properties We want to fetch
    ///
    /// # Returns Success of installation properties if installation is present or Error otherwise
    ///
    pub fn get_installation(
        self,
        id: u32,
    ) -> Result<types::Installation, Box<dyn std::error::Error>> {
        let mut uri_composed = String::new();
        uri_composed.push_str(
            &format!("{}/{}/{}", endpoints::BASE_URL, endpoints::INSTALLATIONS_URL, id)
        );
        let mut res = self.get(&uri_composed)?;
        let text = res.text()?;
        let installation: types::Installation = serde_json::from_str(&text)?;
        Ok(installation)
    }

    /// Get nearest installations
    ///
    /// # Arguments:
    /// * circle - geo-circle that describes area to fetch installations from
    /// * max_results - max number of installations to fetch
    ///
    /// # Returns Success of installations vector if installations are present in the circle or Error otherwise
    ///
    pub fn get_nearest(
        self,
        circle: types::GeoCircle,
        max_results: u32,
    ) -> Result<Vec<types::Installation>, Box<dyn std::error::Error>> {
        let mut uri_composed = String::new();
        let point = circle.get_point();
        uri_composed.push_str(&format!(
            "{}/{}/{}?lat={}&lng={}&maxDistanceKM={}&maxResults={}",
            endpoints::BASE_URL,
            endpoints::INSTALLATIONS_URL,
            endpoints::NEAREST_URL,
            point.get_lat(),
            point.get_lng(),
            circle.get_radius_km(),
            max_results
        ));
        let mut res = self.get(&uri_composed)?;
        let installations = res.json::<Vec<types::Installation>>()?;
        Ok(installations)
    }

    /// Get indexes types
    ///
    /// # Returns Success of indexes types or Error otherwise
    ///
    pub fn get_indices(self) -> Result<Vec<types::IndexType>, Box<dyn std::error::Error>> {
        let mut uri_composed = String::new();
        uri_composed.push_str(&format!("{}/{}", endpoints::BASE_URL, endpoints::INDICES_URL));
        let mut res = self.get(&uri_composed)?;
        let indexes_types = res.json::<Vec<types::IndexType>>()?;
        Ok(indexes_types)
    }

    /// Get meta measurement types
    ///
    /// # Returns Success of measurement types or Error otherwise
    ///
    pub fn get_meta_measurements(
        self,
    ) -> Result<Vec<types::MeasurementType>, Box<dyn std::error::Error>> {
        let mut uri_composed = String::new();
        uri_composed.push_str(
            &format!("{}/{}", endpoints::BASE_URL, endpoints::META_MEASUREMENTS_URL)
        );
        let mut res = self.get(&uri_composed)?;
        let measurements_types = res.json::<Vec<types::MeasurementType>>()?;
        Ok(measurements_types)
    }

    /// Get measurements of specific installation including wind
    ///
    /// # Arguments:
    /// * id - id of the installation We want to get
    /// * index_type - type of index of the installation measurements
    ///
    /// # Returns Success of measurements with wind value or Error otherwise
    ///
    pub fn get_installation_measurements_with_wind(
        self,
        id: u32,
        index_type: types::IndexType,
    ) -> Result<types::Measurements, Box<dyn std::error::Error>> {
        if let Some(type_name) = index_type.name {
            let uri_composed = get_measurements_query_string(id, type_name, IncludeWind::YES);
            self.get_installation_measurements(uri_composed)
        } else {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "IndexType.name is None",
            )));
        }
    }

    /// Get measurements of specific installation but don't include wind
    ///
    /// # Arguments:
    /// * id - id of the installation We want to get
    /// * index_type - type of index of the installation measurements
    ///
    /// # Returns Success of measurements without wind value or Error otherwise
    ///
    pub fn get_installation_measurements_without_wind(
        self,
        id: u32,
        index_type: types::IndexType,
    ) -> Result<types::Measurements, Box<dyn std::error::Error>> {
        if let Some(type_name) = index_type.name {
            let uri_composed = get_measurements_query_string(id, type_name, IncludeWind::NO);
            self.get_installation_measurements(uri_composed)
        } else {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "IndexType.name is None",
            )));
        }
    }

    /// Get measurements of installation nearest the specified point in circle boundaries
    ///
    /// # Arguments:
    /// * index_type - type of index of the installation measurements
    /// * circle - circle describing center point and boundaries
    ///
    /// # Returns Success of measurements or Error otherwise
    ///
    pub fn get_measurements_nearest(
        self,
        index_type: types::IndexType,
        circle: types::GeoCircle,
    ) -> Result<types::Measurements, Box<dyn std::error::Error>> {
        if let Some(name) = index_type.name {
            let mut uri_composed = String::new();
            let point = circle.get_point();
            uri_composed.push_str(&format!(
                "{}/{}/{}?indexType={}&lat={}&lng={}&maxDistanceKM={}",
                endpoints::BASE_URL,
                endpoints::MEASUREMENTS_URL,
                endpoints::NEAREST_URL,
                name,
                point.get_lat(),
                point.get_lng(),
                circle.get_radius_km(),
            ));
            let mut res = self.get(&uri_composed)?;
            let text = res.text()?;
            let measurements: types::Measurements = serde_json::from_str(&text)?;
            return Ok(measurements);
        } else {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "IndexType.name is None",
            )));
        }
    }

    /// Get measurements of interpolated values for given point on map
    ///
    /// # Arguments:
    /// * index_type - type of index of the installation measurements
    /// * point - location for which interpolation of measurements should be calculated
    ///
    /// # Returns Success of interpolated measurements or Error otherwise
    ///
    pub fn get_measurements_point(
        self,
        index_type: types::IndexType,
        point: types::GeoPoint,
    ) -> Result<types::Measurements, Box<dyn std::error::Error>> {
        if let Some(name) = index_type.name {
            let mut uri_composed = String::new();
            uri_composed.push_str(&format!(
                "{}/{}/{}?indexType={}&lat={}&lng={}",
                endpoints::BASE_URL,
                endpoints::MEASUREMENTS_URL,
                endpoints::POINT_URL,
                name,
                point.get_lat(),
                point.get_lng(),
            ));
            let mut res = self.get(&uri_composed)?;
            let text = res.text()?;
            let measurements: types::Measurements = serde_json::from_str(&text)?;
            return Ok(measurements);
        } else {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "IndexType.name is None",
            )));
        }
    }

    fn get_installation_measurements(
        self, uri_composed: String
    ) -> Result<types::Measurements, Box<dyn std::error::Error>> {
        let mut res = self.get(&uri_composed)?;
        let text = res.text()?;
        let measurements: types::Measurements = serde_json::from_str(&text)?;
        return Ok(measurements);
    }

    fn get(self, uri_req: &String) -> Result<Response, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(uri_req)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .header(ACCEPT_LANGUAGE, HeaderValue::from_static("en"))
            .header(HeaderName::from_static("apikey"), self.api_key)
            .send()?;
        Ok(res)
    }
}

fn get_measurements_query_string(id: u32, type_name: String, wind: IncludeWind) -> String {
    let mut uri_composed = String::new();
    let wind_string = match wind {
        IncludeWind::YES => format!("includeWind=true&"),
        IncludeWind::NO => format!(""),
    };
    uri_composed.push_str(&format!(
        "{}/{}/{}?{}indexType={}&installationId={}",
        endpoints::BASE_URL,
        endpoints::MEASUREMENTS_URL,
        endpoints::INSTALLATION_URL,
        wind_string,
        type_name,
        id
    ));
    uri_composed
}

#[cfg(test)]
mod test_client {
    use std::env;
    use dotenv::dotenv;
    const INSTALLATION_ID: u32 = 18;
    const INFO_DETAILS: &str =
        "Error while fetching data, run with: -- --nocapture, to see details.";
    const INFO_CONNECTION: &str = "Cannot establish https connection.";
    const API_KEY_INFO: &str = "API_KEY has wrong length";
    #[test]
    fn test_get_installation() {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        if api_key.len() == 0 {
            panic!(API_KEY_INFO);
        } else {
            if let Ok(client) = super::AirlyClient::new(api_key) {
                if let Ok(installation) = client.get_installation(INSTALLATION_ID) {
                    println!("Fetched installation for id: \n{:?}\n", installation);
                    assert_eq!(installation.id, INSTALLATION_ID as i32);
                } else {
                    panic!(INFO_DETAILS);
                }
            } else {
                panic!(INFO_CONNECTION)
            }
        }
    }
    #[test]
    fn test_get_nearest() {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        if api_key.len() == 0 {
            panic!(API_KEY_INFO);
        } else {
            let circle = super::types::GeoCircle::new(
                super::types::GeoPoint::new(54.347279, 18.653846).unwrap(), // Gdansk, Poland
                5,
            ).unwrap();
            if let Ok(client) = super::AirlyClient::new(api_key) {
                if let Ok(installations) = client.get_nearest(circle, 123) {
                    println!("Fetched installations for nearest: \n{:?}\n", installations);
                    assert_eq!(installations.len() > 0, true);
                } else {
                    panic!(INFO_DETAILS);
                }
            } else {
                panic!(INFO_CONNECTION)
            }
        }
    }
    #[test]
    fn test_get_indices() {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        if api_key.len() == 0 {
            panic!(API_KEY_INFO);
        } else {
            if let Ok(client) = super::AirlyClient::new(api_key) {
                if let Ok(index_types) = client.get_indices() {
                    println!("Fetched indexes: \n{:?}\n", index_types);
                    assert_eq!(index_types.len() > 0, true);
                } else {
                    panic!(INFO_DETAILS);
                }
            } else {
                panic!(INFO_CONNECTION)
            }
        }
    }
    #[test]
    fn test_get_installation_measurements() {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        if api_key.len() == 0 {
            panic!(API_KEY_INFO);
        } else {
            if let Ok(client) = super::AirlyClient::new(api_key) {
                let id = 34;
                let name = Some(format!("AIRLY_CAQI"));
                let level = None;
                let index_type = super::types::IndexType { name, level };
                if let Ok(measurements) = client.clone().get_installation_measurements_with_wind(id, index_type.clone())
                {
                    println!("Fetched measurements for id: {:?}", measurements);
                    if let Some(current) = measurements.current.clone() {
                        assert_eq!(current.values.len() > 0, true);
                    }
                } else {
                    panic!(INFO_DETAILS);
                }
                if let Ok(measurements) = client.get_installation_measurements_without_wind(id, index_type)
                {
                    println!("Fetched measurements for id: {:?}", measurements);
                    if let Some(current) = measurements.current.clone() {
                        assert_eq!(current.values.len() > 0, true);
                    }
                } else {
                    panic!(INFO_DETAILS);
                }
            } else {
                panic!(INFO_CONNECTION)
            }
        }
    }
    #[test]
    fn test_get_measurements_nearest() {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        if api_key.len() == 0 {
            panic!(API_KEY_INFO);
        } else {
            let circle = super::types::GeoCircle::new(
                super::types::GeoPoint::new(54.347279, 18.653846).unwrap(), // Gdansk, Poland
                5,
            ).unwrap();
            if let Ok(client) = super::AirlyClient::new(api_key) {
                let name = Some(format!("AIRLY_CAQI"));
                let level = None;
                let index_type = super::types::IndexType { name, level };
                if let Ok(measurements) = client.get_measurements_nearest(index_type, circle) {
                    println!("Fetched measurements for nearest: {:?}", measurements);
                    if let Some(current) = measurements.current.clone() {
                        assert_eq!(current.values.len() > 0, true);
                    }
                } else {
                    panic!(INFO_DETAILS);
                }
            } else {
                panic!(INFO_CONNECTION)
            }
        }
    }
    #[test]
    fn test_get_measurements_point() {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        if api_key.len() == 0 {
            panic!(API_KEY_INFO);
        } else {
            let point = super::types::GeoPoint::new(54.347279, 18.653846).unwrap(); // Gdansk, Poland
            if let Ok(client) = super::AirlyClient::new(api_key) {
                let name = Some(format!("AIRLY_CAQI"));
                let level = None;
                let index_type = super::types::IndexType { name, level };
                if let Ok(measurements) = client.get_measurements_point(index_type, point) {
                    println!("Fetched measurements for point: {:?}", measurements);
                    if let Some(current) = measurements.current.clone() {
                        assert_eq!(current.values.len() > 0, true);
                    }
                } else {
                    panic!(INFO_DETAILS);
                }
            } else {
                panic!(INFO_CONNECTION)
            }
        }
    }
    #[test]
    fn test_get_meta_measurements() {
        dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        if api_key.len() == 0 {
            panic!(API_KEY_INFO);
        } else {
            if let Ok(client) = super::AirlyClient::new(api_key) {
                if let Ok(measurement_types) = client.get_meta_measurements() {
                    println!("Fetched measurements types: \n{:?}\n", measurement_types);
                    assert_eq!(measurement_types.len() > 0, true);
                } else {
                    panic!(INFO_DETAILS);
                }
            } else {
                panic!(INFO_CONNECTION)
            }
        }
    }
}
