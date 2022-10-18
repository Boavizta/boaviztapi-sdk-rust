/*
 * BOAVIZTAPI - DEMO
 *
 * <p>🎯 Retrieving the impacts of digital elements.</p> <p>This is a quick demo, to see full documentation <a href=\"https://doc.api.boavizta.org\">click here</a></p> <h2>Features</h2> <p>Bellow a list of all available features. Implemented features are specified in each route.</p> <h3>👄 Verbose</h3> <p>Verbose is an HTTP parameter. If set at true : * Shows the impacts of each component * Shows the value used for each attributes</p> <p><em>\"attribute\": {\"value\": \"value\", \"unit\": \"unit\", \"status\": \"Status\", \"source\": \"Source\"}</em></p> <h3>🔨 Manufacture</h3> <ul> <li>Manufacture impacts of devices are the sum of the impacts of its components</li> <li>Manufacture impacts equations of components are given for each component</li> </ul> <h3>🔌  Usage</h3> <p>Usage impacts are measured by multiplying :</p> <ul> <li> <p>a <strong>duration</strong></p> </li> <li> <p>an <strong>impact factor</strong> </p> </li> <li> <p>an <strong>electrical consumption</strong> </p> </li> </ul> <h4>⏲ Duration</h4> <p>Usage impacts are given for a specific time duration. Duration can be given in :</p> <ul> <li>HOURS : <em>usage:{hours_use_time: 1}</em></li> <li>DAYS : <em>usage:{days_use_time: 1}</em></li> <li>YEARS : <em>usage:{years_use_time: 1}</em> </li> </ul> <p>If no duration is given, <strong>the impact is measured for a year</strong>. <em>Note</em> : units are cumulative</p> <h4>✖️ Impact factors</h4> <ul> <li>Impact factors can be given : <em>usage:{[criterion]_factors: 0.38}</em></li> <li>Impact factors can be retrieved from : <em>usage:{usage_location: \"FRA\"}</em>. See the list of usage location :</li> </ul> <h4>⚡ Electrical consumption</h4> <h5>⏺️ Given</h5> <p><em>Electrical consumption can be given for one hour (average) </em>usage:{hours_electrical_consumption: 1}*.</p> <h5>📈 Modeled</h5> <ul> <li>Electrical consumption can be retrieved from consumption profile using <em>usage:{time_workload: 50}</em>. </li> </ul> <h3>🔃 Auto-complete</h3> <p>The API will complete the missing attributes in a request. Components have different completion strategies. Devices have minimal required components. If not given in the request a component with default characteristics are used.</p> <h3>📋 Archetype</h3> <p>If an archetype is given, the missing attributes will be complete with the archetypes attributes instead of default attributes</p> <h3>⏬ Allocation</h3> <p>Allocation is an HTTP parameter. </p> <ul> <li>If set at TOTAL, the total manufacture impact is returned.</li> <li>If set at LINEAR the manufacture impact is allocated linearly hover a specific lifespan given or set by default : {\"usage\":{\"years_life_time\":1}}</li> </ul>
 *
 * The version of the OpenAPI document: 0.2.0-alpha
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WorkloadPower : BaseDTO is simple BaseModel object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkloadPower {
    #[serde(rename = "load_percentage", skip_serializing_if = "Option::is_none")]
    pub load_percentage: Option<f32>,
    #[serde(rename = "power_watt", skip_serializing_if = "Option::is_none")]
    pub power_watt: Option<f32>,
}

impl WorkloadPower {
    /// BaseDTO is simple BaseModel object
    pub fn new() -> WorkloadPower {
        WorkloadPower {
            load_percentage: None,
            power_watt: None,
        }
    }
}

