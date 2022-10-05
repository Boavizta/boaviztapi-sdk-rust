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
pub struct Cpu {
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<i32>,
    #[serde(rename = "core_units", skip_serializing_if = "Option::is_none")]
    pub core_units: Option<i32>,
    #[serde(rename = "die_size", skip_serializing_if = "Option::is_none")]
    pub die_size: Option<f32>,
    #[serde(rename = "die_size_per_core", skip_serializing_if = "Option::is_none")]
    pub die_size_per_core: Option<f32>,
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: Option<f32>,
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "manufacture_date", skip_serializing_if = "Option::is_none")]
    pub manufacture_date: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            units: None,
            core_units: None,
            die_size: None,
            die_size_per_core: None,
            process: None,
            manufacturer: None,
            manufacture_date: None,
            model: None,
            family: None,
        }
    }
}


