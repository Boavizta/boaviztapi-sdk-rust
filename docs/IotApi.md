# \IotApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_archetype_config_v1_iot_iot_device_archetype_config_get**](IotApi.md#get_archetype_config_v1_iot_iot_device_archetype_config_get) | **GET** /v1/iot/iot_device/archetype_config | Get Archetype Config
[**iot_device_get_all_archetype_name_v1_iot_iot_device_archetypes_get**](IotApi.md#iot_device_get_all_archetype_name_v1_iot_iot_device_archetypes_get) | **GET** /v1/iot/iot_device/archetypes | Iot Device Get All Archetype Name
[**iot_device_impact_v1_iot_iot_device_get**](IotApi.md#iot_device_impact_v1_iot_iot_device_get) | **GET** /v1/iot/iot_device | Iot Device Impact
[**iot_device_impact_v1_iot_iot_device_post**](IotApi.md#iot_device_impact_v1_iot_iot_device_post) | **POST** /v1/iot/iot_device | Iot Device Impact



## get_archetype_config_v1_iot_iot_device_archetype_config_get

> serde_json::Value get_archetype_config_v1_iot_iot_device_archetype_config_get(archetype)
Get Archetype Config

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


## iot_device_get_all_archetype_name_v1_iot_iot_device_archetypes_get

> serde_json::Value iot_device_get_all_archetype_name_v1_iot_iot_device_archetypes_get()
Iot Device Get All Archetype Name

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


## iot_device_impact_v1_iot_iot_device_get

> serde_json::Value iot_device_impact_v1_iot_iot_device_get(archetype, verbose, duration, criteria)
Iot Device Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to iot-device-default]
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


## iot_device_impact_v1_iot_iot_device_post

> serde_json::Value iot_device_impact_v1_iot_iot_device_post(verbose, duration, archetype, criteria, io_t)
Iot Device Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f64**> |  |  |
**archetype** | Option<**String**> |  |  |[default to iot-device-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to [gwp, adp, pe]]
**io_t** | Option<[**IoT**](IoT.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

