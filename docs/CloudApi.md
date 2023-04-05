# \CloudApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instance_cloud_impact_v1_cloud_get**](CloudApi.md#instance_cloud_impact_v1_cloud_get) | **GET** /v1/cloud/ | Instance Cloud Impact
[**instance_cloud_impact_v1_cloud_post**](CloudApi.md#instance_cloud_impact_v1_cloud_post) | **POST** /v1/cloud/ | Instance Cloud Impact
[**legacy_instance_cloud_impact_v1_cloud_aws_post**](CloudApi.md#legacy_instance_cloud_impact_v1_cloud_aws_post) | **POST** /v1/cloud/aws | Legacy Instance Cloud Impact
[**legacy_server_get_all_archetype_name_v1_cloud_aws_all_instances_get**](CloudApi.md#legacy_server_get_all_archetype_name_v1_cloud_aws_all_instances_get) | **GET** /v1/cloud/aws/all_instances | Legacy Server Get All Archetype Name
[**server_get_all_archetype_name_v1_cloud_all_instances_get**](CloudApi.md#server_get_all_archetype_name_v1_cloud_all_instances_get) | **GET** /v1/cloud/all_instances | Server Get All Archetype Name
[**server_get_all_provider_name_v1_cloud_all_providers_get**](CloudApi.md#server_get_all_provider_name_v1_cloud_all_providers_get) | **GET** /v1/cloud/all_providers | Server Get All Provider Name



## instance_cloud_impact_v1_cloud_get

> serde_json::Value instance_cloud_impact_v1_cloud_get(provider, instance_type, verbose, allocation)
Instance Cloud Impact

# ✔ ️Cloud instance impacts from provider, instance type and usage  Retrieve the impacts of a given Cloud instance and usage.  ### Features  📋 Provider   Name of the cloud provider. You can retrieve the [list here](#/cloud_instance/server_get_all_cloud_providers).  📋 Instance type   Name of the chosen instance. You can retrieve the [list here](#/cloud/server_get_archetype_name_v1_cloud_all_aws_instances_get).  👄 Verbose  🔨 Manufacture  🔌 Usage   * 📈 Modeled  📋 Archetype : The configuration is set by the API, only usage is given by the user  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | Option<**String**> |  |  |
**instance_type** | Option<**String**> |  |  |
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_cloud_impact_v1_cloud_post

> serde_json::Value instance_cloud_impact_v1_cloud_post(verbose, allocation, cloud)
Instance Cloud Impact

# ✔ ️Cloud instance impacts from provider, instance type and usage  Retrieve the impacts of a given Cloud instance and usage.  ### Features  📋 Provider   Name of the cloud provider. You can retrieve the [list here](#/cloud_instance/server_get_all_cloud_providers).  📋 Instance type   Name of the chosen instance. You can retrieve the [list here](#/cloud/server_get_archetype_name_v1_cloud_all_aws_instances_get).  👄 Verbose  🔨 Manufacture  🔌 Usage   * 📈 Modeled  📋 Archetype : The configuration is set by the API, only usage is given by the user  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**cloud** | Option<[**Cloud**](Cloud.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## legacy_instance_cloud_impact_v1_cloud_aws_post

> serde_json::Value legacy_instance_cloud_impact_v1_cloud_aws_post(instance_type, verbose, allocation, usage_cloud)
Legacy Instance Cloud Impact

# ✔ (LEGACY) ️AWS instance impacts from instance type and usage  Retrieve the impacts of a given AWS instance and usage.  ### Features  📋 Instance type   AWS name of the chosen instance. You can retrieve the [list here](#/cloud/server_get_all_archetype_name_v1_cloud_all_aws_instances_get).  👄 Verbose  🔨 Manufacture  🔌 Usage   * 📈 Modeled  📋 Archetype : The configuration is set by the API, only usage is given by the user  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_type** | Option<**String**> |  |  |
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**usage_cloud** | Option<[**UsageCloud**](UsageCloud.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## legacy_server_get_all_archetype_name_v1_cloud_aws_all_instances_get

> serde_json::Value legacy_server_get_all_archetype_name_v1_cloud_aws_all_instances_get()
Legacy Server Get All Archetype Name

# ✔ (LEGACY)️ Get all the available aws instances 📜 Return the name of all pre-registered aws instances

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


## server_get_all_archetype_name_v1_cloud_all_instances_get

> serde_json::Value server_get_all_archetype_name_v1_cloud_all_instances_get(provider)
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


## server_get_all_provider_name_v1_cloud_all_providers_get

> serde_json::Value server_get_all_provider_name_v1_cloud_all_providers_get()
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

