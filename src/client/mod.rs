mod endpoints;
use crate::request;
use crate::response;
use reqwest;
use reqwest::{
    header::{HeaderName, HeaderValue, ACCEPT, ACCEPT_LANGUAGE},
    Response,
};
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct AirlyClient {
    api_key: HeaderValue,
    client: reqwest::Client,
}

impl AirlyClient {
    /// Constructs AirlyClinet
    ///
    /// # Arguments:
    /// * api_key - personal api key that can be obtained from https://developer.airly.eu/login
    ///
    /// # Returns instance of AirlyClient struct
    ///
    pub fn new(api_key: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        if api_key.len() == 32 {
            let client = reqwest::Client::new();
            let api_key = HeaderValue::from_static(api_key);
            return Ok(Self { api_key, client });
        }
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Wrong api key length",
        )))
    }

    /// Get installation properties for given id
    ///
    /// # Arguments:
    /// * id - id of installation properties We wont to fetch
    ///
    /// # Returns Success of installation properties if installation is present or Error otherwise
    ///
    pub fn get_instalation(
        self,
        id: i32,
    ) -> Result<response::Installation, Box<dyn std::error::Error>> {
        let mut uri_composed = endpoints::BASIC.to_owned();
        uri_composed.push_str(&format!("{}/{}", endpoints::INSTALATIONS, id));
        let mut res = self.get(&uri_composed)?;
        let installation = res.json::<response::Installation>()?;
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
        circle: request::GeoCircle,
        max_results: i32,
    ) -> Result<Vec<response::Installation>, Box<dyn std::error::Error>> {
        let mut uri_composed = endpoints::BASIC.to_owned();
        uri_composed.push_str(&format!(
            "{}/{}?lat={}&lng={}&maxDistanceKM={}&maxResults={}",
            endpoints::INSTALATIONS,
            endpoints::NEAREST,
            circle.point.lat,
            circle.point.lng,
            circle.radius_km,
            max_results
        ));
        let mut res = self.get(&uri_composed)?;
        let installations = res.json::<Vec<response::Installation>>()?;
        Ok(installations)
    }

    pub fn get_indexes(self) -> Result<Vec<response::IndexType>, Box<dyn std::error::Error>> {
        let mut uri_composed = endpoints::BASIC.to_owned();
        uri_composed.push_str(&format!("{}", endpoints::INDEXES));
        let mut res = self.get(&uri_composed)?;
        let indexes_types = res.json::<Vec<response::IndexType>>()?;
        Ok(indexes_types)
    }

    pub fn get_measurements_types(
        self,
    ) -> Result<Vec<response::MeasurementType>, Box<dyn std::error::Error>> {
        let mut uri_composed = endpoints::BASIC.to_owned();
        uri_composed.push_str(&format!("{}", endpoints::MEASUREMENTS_TYPES));
        let mut res = self.get(&uri_composed)?;
        let measurements_types = res.json::<Vec<response::MeasurementType>>()?;
        Ok(measurements_types)
    }

    pub fn get_instalation_measurements(
        self,
        id: i32,
        index_type: response::IndexType,
        included_wind: bool,
    ) -> Result<response::Measurements, Box<dyn std::error::Error>> {
        if let Some(name) = index_type.name {
            let mut uri_composed = endpoints::BASIC.to_owned();
            let mut included_wind_query = "";
            if included_wind {
                included_wind_query = "includeWind=true&"
            }
            uri_composed.push_str(&format!(
                "{}/{}?{}indexType={}&installationId={}",
                endpoints::MEASUREMENTS,
                endpoints::INSTALATION,
                included_wind_query,
                name,
                id
            ));
            let mut res = self.get(&uri_composed)?;
            println!("{:?}", &res);
            let text = res.text()?;
            println!("{}", &text);
            let measurements = res.json::<response::Measurements>()?;
            println!("{:?}", measurements);
            return Ok(measurements);
        } else {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "IndexType.name is None",
            )));
        }
    }

    pub fn get_measurements_nearest(
        self,
        index_type: response::IndexType,
        circle: request::GeoCircle,
    ) -> Result<response::Measurements, Box<dyn std::error::Error>> {
        if let Some(name) = index_type.name {
            let mut uri_composed = endpoints::BASIC.to_owned();
            uri_composed.push_str(&format!(
                "{}/{}?indexType={}&lat={}&lng={}&maxDistanceKM={}",
                endpoints::MEASUREMENTS,
                endpoints::NEAREST,
                name,
                circle.point.lat,
                circle.point.lng,
                circle.radius_km,
            ));
            let mut res = self.get(&uri_composed)?;
            println!("{:?}", &res);
            let text = res.text()?;
            println!("{}", &text);
            let measurements = res.json::<response::Measurements>()?;
            println!("{:?}", measurements);
            return Ok(measurements);
        } else {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "IndexType.name is None",
            )));
        }
    }

    pub fn get_measurements_point(
        self,
        index_type: response::IndexType,
        point: request::GeoPoint,
    ) -> Result<response::Measurements, Box<dyn std::error::Error>> {
        if let Some(name) = index_type.name {
            let mut uri_composed = endpoints::BASIC.to_owned();
            uri_composed.push_str(&format!(
                "{}/{}?indexType={}&lat={}&lng={}",
                endpoints::MEASUREMENTS,
                endpoints::POINT,
                name,
                point.lat,
                point.lng,
            ));
            let mut res = self.get(&uri_composed)?;
            println!("{:?}", &res);
            let text = res.text()?;
            println!("{}", &text);
            let measurements = res.json::<response::Measurements>()?;
            println!("{:?}", measurements);
            return Ok(measurements);
        } else {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "IndexType.name is None",
            )));
        }
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

#[cfg(test)]
mod test_clinet {
    const API_KEY: &str = "";
    const INFO_DETAILS: &str =
        "Error while fetching data, run with: -- --nocapture, to see details.";
    const INFO_CONNECTION: &str = "Cannot establish https connection.";
    #[test]
    fn test_get_instalation() {
        if API_KEY.len() == 0 {
            panic!("Please set API_KEY for tests.");
        }
        let id = 34;
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            if let Ok(installation) = client.get_instalation(id) {
                println!("Fetched installation: {:?}", installation);
                assert_eq!(installation.id, id);
            } else {
                panic!(INFO_DETAILS);
            }
        } else {
            panic!(INFO_CONNECTION)
        }
    }
    #[test]
    fn test_get_nearest() {
        if API_KEY.len() == 0 {
            panic!("Please set API_KEY for tests.");
        }
        let circle =
            super::request::GeoCircle::new(super::request::GeoPoint::new(54.347279, 18.653846), 5);
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            if let Ok(installations) = client.get_nearest(circle, 3) {
                println!("Fetched installations: {:?}", installations);
                assert_eq!(installations.len(), 3);
            } else {
                panic!(INFO_DETAILS);
            }
        } else {
            panic!(INFO_CONNECTION)
        }
    }
    #[test]
    fn test_get_indexes() {
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            if let Ok(index_types) = client.get_indexes() {
                println!("Fetched indexes: {:?}", index_types);
                assert_eq!(index_types.len() > 0, true);
            } else {
                panic!(INFO_DETAILS);
            }
        } else {
            panic!(INFO_CONNECTION)
        }
    }
    #[test]
    #[ignore]
    fn test_get_instalation_measurements() {
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            let id = 34;
            let name = Some(format!("AIRLY_CAQI"));
            let level = None;
            let index_type = super::response::IndexType { name, level };
            if let Ok(measurements) = client.get_instalation_measurements(id, index_type, true) {
                println!("Fetched measurements: {:?}", measurements);
                if let Some(current) = measurements.current.clone() {
                    if let Some(values) = current.values.clone() {
                        println!("{:?}", measurements);
                        assert_eq!(values.len() > 0, true);
                    }
                }
            } else {
                panic!(INFO_DETAILS);
            }
        } else {
            panic!(INFO_CONNECTION)
        }
    }

    #[test]
    fn test_get_measurements_types() {
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            if let Ok(measurements_types) = client.get_measurements_types() {
                println!("Fetched measurements types: {:?}", measurements_types);
                assert_eq!(measurements_types.len() > 0, true);
            } else {
                panic!(INFO_DETAILS);
            }
        } else {
            panic!(INFO_CONNECTION)
        }
    }
}
