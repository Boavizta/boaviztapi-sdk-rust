/*
 * BOAVIZTAPI - DEMO
 *
 * # 🎯 Retrieving the impacts of digital elements This is a quick demo, to see full documentation [click here](https://doc.api.boavizta.org)  ## ➡️Server router  ### Server routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Cloud router  ### Cloud routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Component router  ### Component routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |          | |   ADP  |        X       |          | |   PE   |        X       |          | 
 *
 * The version of the OpenAPI document: 0.1.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Disk {
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "density", skip_serializing_if = "Option::is_none")]
    pub density: Option<f32>,
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "manufacture_date", skip_serializing_if = "Option::is_none")]
    pub manufacture_date: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl Disk {
    pub fn new() -> Disk {
        Disk {
            units: None,
            _type: None,
            capacity: None,
            density: None,
            manufacturer: None,
            manufacture_date: None,
            model: None,
        }
    }
}

