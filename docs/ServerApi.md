# \ServerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_archetype_config_v1_server_archetype_config_get**](ServerApi.md#get_archetype_config_v1_server_archetype_config_get) | **GET** /v1/server/archetype_config | Get Archetype Config
[**server_get_all_archetype_name_v1_server_archetypes_get**](ServerApi.md#server_get_all_archetype_name_v1_server_archetypes_get) | **GET** /v1/server/archetypes | Server Get All Archetype Name
[**server_impact_from_configuration_v1_server_post**](ServerApi.md#server_impact_from_configuration_v1_server_post) | **POST** /v1/server/ | Server Impact From Configuration
[**server_impact_from_model_v1_server_get**](ServerApi.md#server_impact_from_model_v1_server_get) | **GET** /v1/server/ | Server Impact From Model



## get_archetype_config_v1_server_archetype_config_get

> serde_json::Value get_archetype_config_v1_server_archetype_config_get(archetype)
Get Archetype Config

# ✔️ Get the configuration of a given archetype

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_get_all_archetype_name_v1_server_archetypes_get

> serde_json::Value server_get_all_archetype_name_v1_server_archetypes_get()
Server Get All Archetype Name

# ✔️ Get all the available server archetype

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_impact_from_configuration_v1_server_post

> serde_json::Value server_impact_from_configuration_v1_server_post(verbose, duration, archetype, criteria, server)
Server Impact From Configuration

# ✔️ Server impacts from configuration Retrieve the impacts of a given server configuration. ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  🔌 Usage  * ⏺️  Given  * 📈 Modeled  📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
**archetype** | Option<**String**> |  |  |[default to compute_medium]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**server** | Option<[**Server**](Server.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_impact_from_model_v1_server_get

> serde_json::Value server_impact_from_model_v1_server_get(archetype, verbose, duration, criteria)
Server Impact From Model

# ✔ ️Server impacts from model name Retrieve the impacts of a given server archetype. ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  🔌 Usage  📋 Archetype: Uses the classic server impacts router with a pre-registered archetype   ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to compute_medium]
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

