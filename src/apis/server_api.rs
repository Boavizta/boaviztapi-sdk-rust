/*
 * BOAVIZTAPI - DEMO
 *
 * # 🎯 Retrieving the impacts of digital elements This is a quick demo, to see full documentation [click here](https://doc.api.boavizta.org)  ## ➡️Server router  ### Server routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Cloud router  ### Cloud routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Component router  ### Component routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |          | |   ADP  |        X       |          | |   PE   |        X       |          | 
 *
 * The version of the OpenAPI document: 0.1.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`server_get_all_archetype_name_v1_server_all_default_models_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerGetAllArchetypeNameV1ServerAllDefaultModelsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`server_impact_by_config_v1_server_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerImpactByConfigV1ServerPostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`server_impact_by_model_v1_server_model_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerImpactByModelV1ServerModelGetError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// # ✔️Get all the available server models Return the name of all pre-registered server models
pub async fn server_get_all_archetype_name_v1_server_all_default_models_get(configuration: &configuration::Configuration, ) -> Result<serde_json::Value, Error<ServerGetAllArchetypeNameV1ServerAllDefaultModelsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/server/all_default_models", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServerGetAllArchetypeNameV1ServerAllDefaultModelsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️Server impacts from configuration Retrieve the impacts of a given server configuration. ### 💡 Smart complete All missing components and components attributes are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the impacts of each components and the value used for each attributes   ### 📋 Archetype An archetype is a pre-registered server model. An ```archetype``` can be specify in the model object. In case an archetype is specified, all missing data are retrieve from the archetype. You can have a list of available archetype's server models [here](#/server/server_get_all_archetype_name_v1_server_all_default_models_get)   ### ⏲ Duration Usage impacts are given for a specific time duration. Duration can be given in : | time unit | Usage parameter | |------|-----| | HOURS | ```hours_use_time``` | | DAYS | ```days_use_time``` | | YEARS | ```years_use_time``` | If no duration is given, **the impact is measured for a year**. *Note* : units are cumulative ### 🧮 Measure 🔨 Manufacture impacts are the sum of the components impacts  🔌 Usage impacts are measured by multiplying : * a **duration**  * an **impact factor** (```gwp_factor```, ```pe_factor```, ```adp_factor```) - retrieve with ```usage_location``` if not given  * an **electrical consumption** (```hours_electrical_consumption```) - retrieve with ```workload``` if not given
pub async fn server_impact_by_config_v1_server_post(configuration: &configuration::Configuration, verbose: Option<bool>, server_dto: Option<crate::models::ServerDto>) -> Result<serde_json::Value, Error<ServerImpactByConfigV1ServerPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/server/", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&server_dto);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServerImpactByConfigV1ServerPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️Server impacts from model name Retrieve the impacts of a given server name (archetype). ### 📋 Model Uses the [classic server impacts router](#/server/server_impact_by_config_v1_server__post) with a pre-registered model  ### 👄 Verbose If set at true, shows the impacts of each components and the value used for each attributes   ### 📋 Model name You can have a list of available server models names [here](#/server/server_get_all_archetype_name_v1_server_all_default_models_get)   ### 🧮 Measure 🔨 Manufacture impacts are the sum of the pre-registered components impacts  🔌 Usage impacts are measured based on the electrical consumption of the pre-registered model for a year 
pub async fn server_impact_by_model_v1_server_model_get(configuration: &configuration::Configuration, archetype: Option<&str>, verbose: Option<bool>) -> Result<serde_json::Value, Error<ServerImpactByModelV1ServerModelGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/server/model", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = archetype {
        local_var_req_builder = local_var_req_builder.query(&[("archetype", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServerImpactByModelV1ServerModelGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

