mod endpoints;
use crate::request;
use crate::response;
use reqwest;
use reqwest::{
    header::{HeaderName, HeaderValue, CONTENT_TYPE},
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

    /// calls api to get installation properties for given id
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
        uri_composed.push_str(endpoints::INSTALATIONS);
        let id_str = &format!("/{}", id);
        uri_composed.push_str(id_str);
        let mut res = self.get(&uri_composed)?;
        let installation = res.json::<response::Installation>()?;
        Ok(installation)
    }

    /// calls api to get nearest installations
    ///
    /// # Arguments:
    /// * circle - geo circle that describes area to fetch installations from
    /// * max_results - max number of installations to fetch
    ///
    /// # Returns Success of installations vector if installations are present in the circle
    ///           or Error otherwise
    ///
    pub fn get_nearest(
        self,
        circle: request::GeoCircle,
        max_results: i32,
    ) -> Result<Vec<response::Installation>, Box<dyn std::error::Error>> {
        let mut uri_composed = endpoints::BASIC.to_owned();
        let lat = &format!("?lat={}", circle.point.lat);
        let lng = &format!("&lng={}", circle.point.lng);
        let max_dist = &format!("&maxDistanceKM={}", circle.radius_km);
        let _max_results = &format!("&maxResults={}", max_results);
        uri_composed.push_str(endpoints::INSTALATIONS);
        uri_composed.push_str(lat);
        uri_composed.push_str(lng);
        uri_composed.push_str(max_dist);
        uri_composed.push_str(_max_results);
        println!("{:?}", &uri_composed);
        let mut res = self.get(&uri_composed)?;
        let installations = res.json::<Vec<response::Installation>>()?;
        println!("{:?}", installations);
        Ok(installations)
    }

    fn get(self, uri_req: &String) -> Result<Response, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(uri_req)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(HeaderName::from_static("apikey"), self.api_key)
            .send()?;
        Ok(res)
    }
}

#[cfg(test)]
mod test_clinet {
    const API_KEY: &str = "";
    #[test]
    fn test_get_instalation() {
        if API_KEY.len() == 0 {
            panic!("Please set API_KEY const for tests.");
        }
        let id = 34;
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            if let Ok(installation) = client.get_instalation(id) {
                println!("Fetched installation: {:?}", installation);
                assert_eq!(installation.id, id);
            } else {
                panic!("Error while fetching data, run with: -- --nocapture, to see details.");
            }
        } else {
            panic!("Cannot establish https connection.")
        }
    }
    #[test]
    fn test_get_nearest() {
        if API_KEY.len() == 0 {
            panic!("Please set API_KEY const for tests.");
        }
        // /nearest?lat=50.062006&lng=19.940984&maxDistanceKM=3&maxResults=1
        let circle =
            super::request::GeoCircle::new(super::request::GeoPoint::new(50.062006, 9.940984), 10);
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            if let Ok(installations) = client.get_nearest(circle, 3) {
                println!("Fetched installations: {:?}", installations);
                assert_eq!(installations.len(), 3);
            } else {
                panic!("Error while fetching data, run with: -- --nocapture, to see details.");
            }
        } else {
            panic!("Cannot establish https connection.")
        }
    }
}
