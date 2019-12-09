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
    pub id: i32,
    pub location: Location,
    pub address: Address,
    pub elevation: f64,
    pub airly: bool,
    pub sponsor: Sponsor,
}
