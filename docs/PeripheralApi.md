# \PeripheralApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**external_hdd_get_all_archetype_name_v1_peripheral_external_hdd_archetypes_get**](PeripheralApi.md#external_hdd_get_all_archetype_name_v1_peripheral_external_hdd_archetypes_get) | **GET** /v1/peripheral/external_hdd/archetypes | External Hdd Get All Archetype Name
[**external_hdd_get_archetype_config_v1_peripheral_external_hdd_archetype_config_get**](PeripheralApi.md#external_hdd_get_archetype_config_v1_peripheral_external_hdd_archetype_config_get) | **GET** /v1/peripheral/external_hdd/archetype_config | External Hdd Get Archetype Config
[**external_hdd_impact_v1_peripheral_external_hdd_get**](PeripheralApi.md#external_hdd_impact_v1_peripheral_external_hdd_get) | **GET** /v1/peripheral/external_hdd | External Hdd Impact
[**external_hdd_impact_v1_peripheral_external_hdd_post**](PeripheralApi.md#external_hdd_impact_v1_peripheral_external_hdd_post) | **POST** /v1/peripheral/external_hdd | External Hdd Impact
[**external_ssd_get_all_archetype_name_v1_peripheral_external_ssd_archetypes_get**](PeripheralApi.md#external_ssd_get_all_archetype_name_v1_peripheral_external_ssd_archetypes_get) | **GET** /v1/peripheral/external_ssd/archetypes | External Ssd Get All Archetype Name
[**external_ssd_get_archetype_config_v1_peripheral_external_ssd_archetype_config_get**](PeripheralApi.md#external_ssd_get_archetype_config_v1_peripheral_external_ssd_archetype_config_get) | **GET** /v1/peripheral/external_ssd/archetype_config | External Ssd Get Archetype Config
[**external_ssd_impact_v1_peripheral_external_ssd_get**](PeripheralApi.md#external_ssd_impact_v1_peripheral_external_ssd_get) | **GET** /v1/peripheral/external_ssd | External Ssd Impact
[**external_ssd_impact_v1_peripheral_external_ssd_post**](PeripheralApi.md#external_ssd_impact_v1_peripheral_external_ssd_post) | **POST** /v1/peripheral/external_ssd | External Ssd Impact
[**monitor_get_all_archetype_name_v1_peripheral_monitor_archetypes_get**](PeripheralApi.md#monitor_get_all_archetype_name_v1_peripheral_monitor_archetypes_get) | **GET** /v1/peripheral/monitor/archetypes | Monitor Get All Archetype Name
[**monitor_get_archetype_config_v1_peripheral_monitor_archetype_config_get**](PeripheralApi.md#monitor_get_archetype_config_v1_peripheral_monitor_archetype_config_get) | **GET** /v1/peripheral/monitor/archetype_config | Monitor Get Archetype Config
[**monitor_impact_v1_peripheral_monitor_get**](PeripheralApi.md#monitor_impact_v1_peripheral_monitor_get) | **GET** /v1/peripheral/monitor | Monitor Impact
[**monitor_impact_v1_peripheral_monitor_post**](PeripheralApi.md#monitor_impact_v1_peripheral_monitor_post) | **POST** /v1/peripheral/monitor | Monitor Impact
[**peripheral_get_all_categories_v1_peripheral_all_get**](PeripheralApi.md#peripheral_get_all_categories_v1_peripheral_all_get) | **GET** /v1/peripheral/all | Peripheral Get All Categories
[**usb_stick_get_all_archetype_name_v1_peripheral_usb_stick_archetypes_get**](PeripheralApi.md#usb_stick_get_all_archetype_name_v1_peripheral_usb_stick_archetypes_get) | **GET** /v1/peripheral/usb_stick/archetypes | Usb Stick Get All Archetype Name
[**usb_stick_get_archetype_config_v1_peripheral_usb_stick_archetype_config_get**](PeripheralApi.md#usb_stick_get_archetype_config_v1_peripheral_usb_stick_archetype_config_get) | **GET** /v1/peripheral/usb_stick/archetype_config | Usb Stick Get Archetype Config
[**usb_stick_impact_v1_peripheral_usb_stick_get**](PeripheralApi.md#usb_stick_impact_v1_peripheral_usb_stick_get) | **GET** /v1/peripheral/usb_stick | Usb Stick Impact
[**usb_stick_impact_v1_peripheral_usb_stick_post**](PeripheralApi.md#usb_stick_impact_v1_peripheral_usb_stick_post) | **POST** /v1/peripheral/usb_stick | Usb Stick Impact



## external_hdd_get_all_archetype_name_v1_peripheral_external_hdd_archetypes_get

> serde_json::Value external_hdd_get_all_archetype_name_v1_peripheral_external_hdd_archetypes_get()
External Hdd Get All Archetype Name

# ✔️ Get all the available user terminal archetype for a given user terminal name

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


## external_hdd_get_archetype_config_v1_peripheral_external_hdd_archetype_config_get

> serde_json::Value external_hdd_get_archetype_config_v1_peripheral_external_hdd_archetype_config_get(archetype)
External Hdd Get Archetype Config

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


## external_hdd_impact_v1_peripheral_external_hdd_get

> serde_json::Value external_hdd_impact_v1_peripheral_external_hdd_get(archetype, verbose, duration, criteria)
External Hdd Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to hdd-default]
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


## external_hdd_impact_v1_peripheral_external_hdd_post

> serde_json::Value external_hdd_impact_v1_peripheral_external_hdd_post(verbose, duration, archetype, criteria, external_hdd)
External Hdd Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f64**> |  |  |
**archetype** | Option<**String**> |  |  |[default to hdd-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to [gwp, adp, pe]]
**external_hdd** | Option<[**ExternalHdd**](ExternalHdd.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_ssd_get_all_archetype_name_v1_peripheral_external_ssd_archetypes_get

> serde_json::Value external_ssd_get_all_archetype_name_v1_peripheral_external_ssd_archetypes_get()
External Ssd Get All Archetype Name

# ✔️ Get all the available user terminal archetype for a given user terminal name

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


## external_ssd_get_archetype_config_v1_peripheral_external_ssd_archetype_config_get

> serde_json::Value external_ssd_get_archetype_config_v1_peripheral_external_ssd_archetype_config_get(archetype)
External Ssd Get Archetype Config

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


## external_ssd_impact_v1_peripheral_external_ssd_get

> serde_json::Value external_ssd_impact_v1_peripheral_external_ssd_get(archetype, verbose, duration, criteria)
External Ssd Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to external-ssd-default]
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


## external_ssd_impact_v1_peripheral_external_ssd_post

> serde_json::Value external_ssd_impact_v1_peripheral_external_ssd_post(verbose, duration, archetype, criteria, external_ssd)
External Ssd Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f64**> |  |  |
**archetype** | Option<**String**> |  |  |[default to external-ssd-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to [gwp, adp, pe]]
**external_ssd** | Option<[**ExternalSsd**](ExternalSsd.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitor_get_all_archetype_name_v1_peripheral_monitor_archetypes_get

> serde_json::Value monitor_get_all_archetype_name_v1_peripheral_monitor_archetypes_get()
Monitor Get All Archetype Name

# ✔️ Get all the available user terminal archetype for a given user terminal name

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


## monitor_get_archetype_config_v1_peripheral_monitor_archetype_config_get

> serde_json::Value monitor_get_archetype_config_v1_peripheral_monitor_archetype_config_get(archetype)
Monitor Get Archetype Config

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


## monitor_impact_v1_peripheral_monitor_get

> serde_json::Value monitor_impact_v1_peripheral_monitor_get(archetype, verbose, duration, criteria)
Monitor Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to monitor-default]
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


## monitor_impact_v1_peripheral_monitor_post

> serde_json::Value monitor_impact_v1_peripheral_monitor_post(verbose, duration, archetype, criteria, monitor)
Monitor Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f64**> |  |  |
**archetype** | Option<**String**> |  |  |[default to monitor-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to [gwp, adp, pe]]
**monitor** | Option<[**Monitor**](Monitor.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peripheral_get_all_categories_v1_peripheral_all_get

> serde_json::Value peripheral_get_all_categories_v1_peripheral_all_get()
Peripheral Get All Categories

# ✔️ Get all the available user peripheral router

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


## usb_stick_get_all_archetype_name_v1_peripheral_usb_stick_archetypes_get

> serde_json::Value usb_stick_get_all_archetype_name_v1_peripheral_usb_stick_archetypes_get()
Usb Stick Get All Archetype Name

# ✔️ Get all the available user terminal archetype for a given user terminal name

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


## usb_stick_get_archetype_config_v1_peripheral_usb_stick_archetype_config_get

> serde_json::Value usb_stick_get_archetype_config_v1_peripheral_usb_stick_archetype_config_get(archetype)
Usb Stick Get Archetype Config

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


## usb_stick_impact_v1_peripheral_usb_stick_get

> serde_json::Value usb_stick_impact_v1_peripheral_usb_stick_get(archetype, verbose, duration, criteria)
Usb Stick Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to usb-stick-default]
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


## usb_stick_impact_v1_peripheral_usb_stick_post

> serde_json::Value usb_stick_impact_v1_peripheral_usb_stick_post(verbose, duration, archetype, criteria, usb_stick)
Usb Stick Impact

# ✔ Peripheral impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f64**> |  |  |
**archetype** | Option<**String**> |  |  |[default to usb-stick-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to [gwp, adp, pe]]
**usb_stick** | Option<[**UsbStick**](UsbStick.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

