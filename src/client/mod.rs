mod endpoints;
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
    #[test]
    fn test_connection() {
        const API_KEY: &str = "";
        if API_KEY.len() == 0 {
            println!("Please set API_KEY const for tests.");
            assert_eq!(true, false);
        }
        if let Ok(client) = super::AirlyClient::new(API_KEY) {
            let result = client.get_instalation(34);
            println!("{:?}", result);
        } else {
            assert_eq!(true, false);
        }
    }
}
