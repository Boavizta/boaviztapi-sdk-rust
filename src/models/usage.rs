/*
 * BOAVIZTAPI - DEMO
 *
 * <p>🎯 Retrieving the impacts of digital elements.</p> <p>This is a quick demo, to see full documentation <a href=\"https://doc.api.boavizta.org\">click here</a></p> <h2>Features</h2> <p>Bellow a list of all available features.</p> <h3>👄 Verbose</h3> <p>Verbose is an HTTP parameter. If set at true :</p> <ul> <li>Shows the impacts of each component</li> <li>Shows the value used for each attribute</li> </ul> <p><em>\"attribute\": {\"value\": \"value\", \"unit\": \"unit\", \"status\": \"Status\", \"source\": \"Source\", \"min\":\"min\", \"max\":\"max\", \"significant_figures\":\"significant_figures\"}</em></p> <h3>🔨 Embedded</h3> <ul> <li>Embedded impacts are the impacts occurring during raw material extraction, manufacture, distribution and end of life</li> <li>When end of life is not taken into account, we specified it in the <code>warnings</code></li> </ul> <h3>🔌  Usage</h3> <p>Usage impacts are assessed by multiplying :</p> <ul> <li> <p>a <strong>duration</strong></p> </li> <li> <p>an <strong>impact factor</strong> </p> </li> <li> <p>an <strong>electrical consumption</strong> </p> </li> </ul> <h4>⏲ Duration</h4> <p>Usage impacts can be given as a router parameter, in hours.</p> <p>If no duration is given, <strong>the impact is assess for the all life duration of the asset</strong>.</p> <h4>✖️ Impact factors</h4> <ul> <li>Impact factors can be given : <em>\"usage\":{\"elec_factors\":{[criteria]: 0.38}}</em></li> <li> <p>Impact factors can be retrieved from : <em>\"usage\":{\"usage_location\": \"FRA\"}</em>. </p> </li> <li> <p>See the list of locations : <a href=\"/v1/utils/country_code\">/v1/utils/country_code</a>*</p> </li> </ul> <h4>⚡ Electrical consumption</h4> <h5>⏺️ Given</h5> <ul> <li>Electrical consumption can be given for one hour (average) <em>\"usage\":{\"avg_power\": 1}</em>.</li> </ul> <h5>📈 Modeled</h5> <ul> <li>Electrical consumption can be retrieved from consumption profile using <em>usage:{time_workload: 50}</em>.</li> </ul> <h5>📋 Archetype</h5> <ul> <li>In some cases, default electrical consumption can be taken from the archetype</li> </ul> <h3>🔃 Auto-complete &amp; 📋 Archetype</h3> <p>The API will complete the missing attributes in a request with a completion function or with values taken from the <code>archetype</code> specified in the route parameter.</p> <h3>⏬ Allocation</h3> <ul> <li>Usage impacts are assessed on the duration given in route parameter</li> <li>Embedded impacts are allocated linearly on the duration given in parameter <code>embedded_impact = impact * (duration/life_duration)</code></li> </ul> <p>If no duration is given, the life_duration (<code>`hours_life_time</code>) of the asset is used.</p>
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Usage : BaseDTO is simple BaseModel object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "use_time_ratio", skip_serializing_if = "Option::is_none")]
    pub use_time_ratio: Option<f32>,
    #[serde(rename = "hours_life_time", skip_serializing_if = "Option::is_none")]
    pub hours_life_time: Option<f32>,
    #[serde(rename = "avg_power", skip_serializing_if = "Option::is_none")]
    pub avg_power: Option<f32>,
    // Temporary workaround until better fix:  force time_workload as a float (instead of generated code that is not usable)
    // In practice, this limits the use of API: we can only query API by passing an average  load for the entire duration of the usage.
   // This is equivalent to passing the following workload value
   // "time_workload": [
   // {
   //     "time_percentage": 100,
    //     "load_percentage": <value>
    // }
    // ]
   // See https://github.com/Boavizta/boaviztapi-sdk-rust/issues/6
   #[serde(rename = "time_workload", skip_serializing_if = "Option::is_none")]
   pub time_workload: Option<f32>,
    #[serde(rename = "usage_location", skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[serde(rename = "elec_factors", skip_serializing_if = "Option::is_none")]
    pub elec_factors: Option<Box<crate::models::ElecFactors>>,
}

impl Usage {
    /// BaseDTO is simple BaseModel object
    pub fn new() -> Usage {
        Usage {
            use_time_ratio: None,
            hours_life_time: None,
            avg_power: None,
            time_workload: None,
            usage_location: None,
            elec_factors: None,
        }
    }
}


