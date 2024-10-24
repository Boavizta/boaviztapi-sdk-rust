# \CloudApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_archetype_config_v1_cloud_instance_instance_config_get**](CloudApi.md#get_archetype_config_v1_cloud_instance_instance_config_get) | **GET** /v1/cloud/instance/instance_config | Get Archetype Config
[**instance_cloud_impact_v1_cloud_instance_get**](CloudApi.md#instance_cloud_impact_v1_cloud_instance_get) | **GET** /v1/cloud/instance | Instance Cloud Impact
[**instance_cloud_impact_v1_cloud_instance_post**](CloudApi.md#instance_cloud_impact_v1_cloud_instance_post) | **POST** /v1/cloud/instance | Instance Cloud Impact
[**server_get_all_archetype_name_v1_cloud_instance_all_instances_get**](CloudApi.md#server_get_all_archetype_name_v1_cloud_instance_all_instances_get) | **GET** /v1/cloud/instance/all_instances | Server Get All Archetype Name
[**server_get_all_provider_name_v1_cloud_instance_all_providers_get**](CloudApi.md#server_get_all_provider_name_v1_cloud_instance_all_providers_get) | **GET** /v1/cloud/instance/all_providers | Server Get All Provider Name



## get_archetype_config_v1_cloud_instance_instance_config_get

> serde_json::Value get_archetype_config_v1_cloud_instance_instance_config_get(provider, instance_type)
Get Archetype Config

# ✔️ Get the configuration of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | Option<**String**> |  |  |[default to aws]
**instance_type** | Option<**String**> |  |  |[default to a1.4xlarge]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_cloud_impact_v1_cloud_instance_get

> serde_json::Value instance_cloud_impact_v1_cloud_instance_get(provider, instance_type, verbose, duration, criteria)
Instance Cloud Impact

# ✔ ️Cloud instance impacts from provider, instance type and usage  Retrieve the impacts of a given Cloud instance and usage.  ### Features  📋 Provider   Name of the cloud provider. You can retrieve the [list here](#/cloud_instance/server_get_all_cloud_providers).  📋 Instance type   Name of the chosen instance. You can retrieve the [list here](#/cloud/server_get_archetype_name_v1_cloud_all_aws_instances_get).  👄 Verbose  🔨 Embedded  🔌 Usage   * 📈 Modeled  📋 Archetype : The configuration is set by the API, only usage is given by the user  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | Option<**String**> |  |  |[default to aws]
**instance_type** | Option<**String**> |  |  |[default to a1.4xlarge]
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f64**> |  |  |
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to [gwp, adp, pe]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_cloud_impact_v1_cloud_instance_post

> serde_json::Value instance_cloud_impact_v1_cloud_instance_post(verbose, duration, criteria, cloud)
Instance Cloud Impact

# ✔ ️Cloud instance impacts from provider, instance type and usage  Retrieve the impacts of a given Cloud instance and usage.  ### Features  📋 Provider   Name of the cloud provider. You can retrieve the [list here](#/cloud_instance/server_get_all_cloud_providers).  📋 Instance type   Name of the chosen instance. You can retrieve the [list here](#/cloud/server_get_archetype_name_v1_cloud_all_aws_instances_get).  👄 Verbose  🔨 Embedded  🔌 Usage   * 📈 Modeled  📋 Archetype : The configuration is set by the API, only usage is given by the user  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f64**> |  |  |
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to [gwp, adp, pe]]
**cloud** | Option<[**Cloud**](Cloud.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_get_all_archetype_name_v1_cloud_instance_all_instances_get

> serde_json::Value server_get_all_archetype_name_v1_cloud_instance_all_instances_get(provider)
Server Get All Archetype Name

# ✔ ️Get all the available instances for a given Cloud provider 📜 Return the name of all pre-registered instances for the Cloud provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_get_all_provider_name_v1_cloud_instance_all_providers_get

> serde_json::Value server_get_all_provider_name_v1_cloud_instance_all_providers_get()
Server Get All Provider Name

# ✔ ️Get all the available Cloud providers 📜 Return the names of all pre-registered Cloud providers

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

