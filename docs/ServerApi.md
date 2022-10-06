# \ServerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**server_get_all_archetype_name_v1_server_all_default_models_get**](ServerApi.md#server_get_all_archetype_name_v1_server_all_default_models_get) | **GET** /v1/server/all_default_models | Server Get All Archetype Name
[**server_impact_from_configuration_v1_server_post**](ServerApi.md#server_impact_from_configuration_v1_server_post) | **POST** /v1/server/ | Server Impact From Configuration
[**server_impact_from_model_v1_server_model_get**](ServerApi.md#server_impact_from_model_v1_server_model_get) | **GET** /v1/server/model | Server Impact From Model



## server_get_all_archetype_name_v1_server_all_default_models_get

> serde_json::Value server_get_all_archetype_name_v1_server_all_default_models_get()
Server Get All Archetype Name

# ✔️ Get all the available server models 📜 Return the name of all pre-registered server models

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

> serde_json::Value server_impact_from_configuration_v1_server_post(verbose, allocation, server)
Server Impact From Configuration

# ✔️ Server impacts from configuration Retrieve the impacts of a given server configuration. ### Features  👄 Verbose  🔃 Auto-complete  🔨 Manufacture  🔌 Usage  * ⏺️  Given  * 📈 Modeled  📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |
**server** | Option<[**Server**](Server.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_impact_from_model_v1_server_model_get

> serde_json::Value server_impact_from_model_v1_server_model_get(archetype, verbose, allocation)
Server Impact From Model

# ✔ ️Server impacts from model name Retrieve the impacts of a given server name (archetype). ### Features  👄 Verbose  🔃 Auto-complete  🔨 Manufacture  🔌 Usage  📋 Archetype: Uses the [classic server impacts router]with a pre-registered archetype   ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

