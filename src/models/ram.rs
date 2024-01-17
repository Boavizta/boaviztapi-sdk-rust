/*
 * BOAVIZTAPI - DEMO
 *
 * <p>🎯 Retrieving the impacts of digital elements.</p> <p>This is a quick demo, to see full documentation <a href=\"https://doc.api.boavizta.org\">click here</a></p> <h2>Features</h2> <p>Bellow a list of all available features.</p> <h3>👄 Verbose</h3> <p>Verbose is an HTTP parameter. If set at true :</p> <ul> <li>Shows the impacts of each component</li> <li>Shows the value used for each attribute</li> </ul> <p><em>\"attribute\": {\"value\": \"value\", \"unit\": \"unit\", \"status\": \"Status\", \"source\": \"Source\", \"min\":\"min\", \"max\":\"max\", \"significant_figures\":\"significant_figures\"}</em></p> <h3>🔨 Embedded</h3> <ul> <li>Embedded impacts are the impacts occurring during raw material extraction, manufacture, distribution and end of life</li> <li>When end of life is not taken into account, we specified it in the <code>warnings</code></li> </ul> <h3>🔌  Usage</h3> <p>Usage impacts are assessed by multiplying :</p> <ul> <li> <p>a <strong>duration</strong></p> </li> <li> <p>an <strong>impact factor</strong> </p> </li> <li> <p>an <strong>electrical consumption</strong> </p> </li> </ul> <h4>⏲ Duration</h4> <p>Usage impacts can be given as a router parameter, in hours.</p> <p>If no duration is given, <strong>the impact is assess for the all life duration of the asset</strong>.</p> <h4>✖️ Impact factors</h4> <ul> <li>Impact factors can be given : <em>\"usage\":{\"elec_factors\":{[criterion]_factors: 0.38}}</em></li> <li> <p>Impact factors can be retrieved from : <em>\"usage\":{\"usage_location\": \"FRA\"}</em>. </p> </li> <li> <p>See the list of locations : <a href=\"/v1/utils/country_code\">/v1/utils/country_code</a>*</p> </li> </ul> <h4>⚡ Electrical consumption</h4> <h5>⏺️ Given</h5> <ul> <li>Electrical consumption can be given for one hour (average) <em>\"usage\":{\"avg_power\": 1}</em>.</li> </ul> <h5>📈 Modeled</h5> <ul> <li>Electrical consumption can be retrieved from consumption profile using <em>usage:{time_workload: 50}</em>.</li> </ul> <h5>📋 Archetype</h5> <ul> <li>In some cases, default electrical consumption can be taken from the archetype</li> </ul> <h3>🔃 Auto-complete &amp; 📋 Archetype</h3> <p>The API will complete the missing attributes in a request with a completion function or with values taken from the <code>archetype</code> specified in the route parameter.</p> <h3>⏬ Allocation</h3> <ul> <li>Usage impacts are assessed on the duration given in route parameter</li> <li>Embedded impacts are allocated linearly on the duration given in parameter <code>embedded_impact = impact * (duration/life_duration)</code></li> </ul> <p>If no duration is given, the life_duration (<code>`hours_life_time</code>) of the asset is used.</p>
 *
 * The version of the OpenAPI document: 1.0.0a3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Ram : BaseDTO is simple BaseModel object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Ram {
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<i32>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<crate::models::Usage1>>,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "density", skip_serializing_if = "Option::is_none")]
    pub density: Option<f32>,
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: Option<f32>,
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl Ram {
    /// BaseDTO is simple BaseModel object
    pub fn new() -> Ram {
        Ram {
            units: None,
            usage: None,
            capacity: None,
            density: None,
            process: None,
            manufacturer: None,
            model: None,
        }
    }
}


