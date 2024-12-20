# \UtilsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**name_to_cpu_v1_utils_name_to_cpu_get**](UtilsApi.md#name_to_cpu_v1_utils_name_to_cpu_get) | **GET** /v1/utils/name_to_cpu | Name To Cpu
[**utils_get_all_case_type_v1_utils_case_type_get**](UtilsApi.md#utils_get_all_case_type_v1_utils_case_type_get) | **GET** /v1/utils/case_type | Utils Get All Case Type
[**utils_get_all_countries_v1_utils_country_code_get**](UtilsApi.md#utils_get_all_countries_v1_utils_country_code_get) | **GET** /v1/utils/country_code | Utils Get All Countries
[**utils_get_all_cpu_family_v1_utils_cpu_family_get**](UtilsApi.md#utils_get_all_cpu_family_v1_utils_cpu_family_get) | **GET** /v1/utils/cpu_family | Utils Get All Cpu Family
[**utils_get_all_cpu_model_range_v1_utils_cpu_model_range_get**](UtilsApi.md#utils_get_all_cpu_model_range_v1_utils_cpu_model_range_get) | **GET** /v1/utils/cpu_model_range | Utils Get All Cpu Model Range
[**utils_get_all_cpu_name_v1_utils_cpu_name_get**](UtilsApi.md#utils_get_all_cpu_name_v1_utils_cpu_name_get) | **GET** /v1/utils/cpu_name | Utils Get All Cpu Name
[**utils_get_all_impacts_criteria_v1_utils_impact_criteria_get**](UtilsApi.md#utils_get_all_impacts_criteria_v1_utils_impact_criteria_get) | **GET** /v1/utils/impact_criteria | Utils Get All Impacts Criteria
[**utils_get_all_ram_manufacturer_v1_utils_ram_manufacturer_get**](UtilsApi.md#utils_get_all_ram_manufacturer_v1_utils_ram_manufacturer_get) | **GET** /v1/utils/ram_manufacturer | Utils Get All Ram Manufacturer
[**utils_get_all_ssd_manufacturer_v1_utils_ssd_manufacturer_get**](UtilsApi.md#utils_get_all_ssd_manufacturer_v1_utils_ssd_manufacturer_get) | **GET** /v1/utils/ssd_manufacturer | Utils Get All Ssd Manufacturer
[**version_v1_utils_version_get**](UtilsApi.md#version_v1_utils_version_get) | **GET** /v1/utils/version | Version



## name_to_cpu_v1_utils_name_to_cpu_get

> serde_json::Value name_to_cpu_v1_utils_name_to_cpu_get(cpu_name)
Name To Cpu

# ✔ ️Complete a cpu attributes from a cpu name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cpu_name** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## utils_get_all_case_type_v1_utils_case_type_get

> serde_json::Value utils_get_all_case_type_v1_utils_case_type_get()
Utils Get All Case Type

# ✔ ️Get all the available case type in the API (*model:{case:'blade'}*)

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


## utils_get_all_countries_v1_utils_country_code_get

> serde_json::Value utils_get_all_countries_v1_utils_country_code_get()
Utils Get All Countries

# ✔ ️Get all the available countries with their trigram code (*usage:{usage_location: 'FRA'}*)

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


## utils_get_all_cpu_family_v1_utils_cpu_family_get

> serde_json::Value utils_get_all_cpu_family_v1_utils_cpu_family_get()
Utils Get All Cpu Family

# ✔ ️Get all the available cpu family in the API (*cpu:{family:'skylake'}*)

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


## utils_get_all_cpu_model_range_v1_utils_cpu_model_range_get

> serde_json::Value utils_get_all_cpu_model_range_v1_utils_cpu_model_range_get()
Utils Get All Cpu Model Range

# ✔ ️Get all the available cpu family in the API (*cpu:{model_range:'xeon platinum'}*)

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


## utils_get_all_cpu_name_v1_utils_cpu_name_get

> serde_json::Value utils_get_all_cpu_name_v1_utils_cpu_name_get()
Utils Get All Cpu Name

# ✔ ️Get all the available cpu name in the API (*cpu:{name:'intel xeon platinum 8175m'}*)

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


## utils_get_all_impacts_criteria_v1_utils_impact_criteria_get

> serde_json::Value utils_get_all_impacts_criteria_v1_utils_impact_criteria_get()
Utils Get All Impacts Criteria

# ✔ ️Get all the available criteria for the impacts calculation

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


## utils_get_all_ram_manufacturer_v1_utils_ram_manufacturer_get

> serde_json::Value utils_get_all_ram_manufacturer_v1_utils_ram_manufacturer_get()
Utils Get All Ram Manufacturer

# ✔ ️Get all the available ram manufacturer in the API (*ram:{manufacturer:'samsung'}*)

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


## utils_get_all_ssd_manufacturer_v1_utils_ssd_manufacturer_get

> serde_json::Value utils_get_all_ssd_manufacturer_v1_utils_ssd_manufacturer_get()
Utils Get All Ssd Manufacturer

# ✔ ️Get all the available ssd manufacturer in the API (*ssd:{manufacturer:'samsung'}*)

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


## version_v1_utils_version_get

> serde_json::Value version_v1_utils_version_get()
Version

Get the version of the API

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

