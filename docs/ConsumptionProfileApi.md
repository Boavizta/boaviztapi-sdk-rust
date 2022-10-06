# \ConsumptionProfileApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cpu_consumption_profile_v1_consumption_profile_cpu_post**](ConsumptionProfileApi.md#cpu_consumption_profile_v1_consumption_profile_cpu_post) | **POST** /v1/consumption_profile/cpu | Cpu Consumption Profile



## cpu_consumption_profile_v1_consumption_profile_cpu_post

> serde_json::Value cpu_consumption_profile_v1_consumption_profile_cpu_post(verbose, consumption_profile_cpu)
Cpu Consumption Profile

cpu consumption profile generator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**consumption_profile_cpu** | Option<[**ConsumptionProfileCpu**](ConsumptionProfileCpu.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

