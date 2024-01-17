# \UserTerminalApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**box_impact_v1_user_terminal_box_get**](UserTerminalApi.md#box_impact_v1_user_terminal_box_get) | **GET** /v1/user_terminal/box | Box Impact
[**box_impact_v1_user_terminal_box_post**](UserTerminalApi.md#box_impact_v1_user_terminal_box_post) | **POST** /v1/user_terminal/box | Box Impact
[**desktop_impact_v1_user_terminal_desktop_get**](UserTerminalApi.md#desktop_impact_v1_user_terminal_desktop_get) | **GET** /v1/user_terminal/desktop | Desktop Impact
[**desktop_impact_v1_user_terminal_desktop_post**](UserTerminalApi.md#desktop_impact_v1_user_terminal_desktop_post) | **POST** /v1/user_terminal/desktop | Desktop Impact
[**external_hdd_impact_v1_user_terminal_external_hdd_get**](UserTerminalApi.md#external_hdd_impact_v1_user_terminal_external_hdd_get) | **GET** /v1/user_terminal/external_hdd | External Hdd Impact
[**external_hdd_impact_v1_user_terminal_external_hdd_post**](UserTerminalApi.md#external_hdd_impact_v1_user_terminal_external_hdd_post) | **POST** /v1/user_terminal/external_hdd | External Hdd Impact
[**external_ssd_impact_v1_user_terminal_external_ssd_get**](UserTerminalApi.md#external_ssd_impact_v1_user_terminal_external_ssd_get) | **GET** /v1/user_terminal/external_ssd | External Ssd Impact
[**external_ssd_impact_v1_user_terminal_external_ssd_post**](UserTerminalApi.md#external_ssd_impact_v1_user_terminal_external_ssd_post) | **POST** /v1/user_terminal/external_ssd | External Ssd Impact
[**laptop_impact_v1_user_terminal_laptop_get**](UserTerminalApi.md#laptop_impact_v1_user_terminal_laptop_get) | **GET** /v1/user_terminal/laptop | Laptop Impact
[**laptop_impact_v1_user_terminal_laptop_post**](UserTerminalApi.md#laptop_impact_v1_user_terminal_laptop_post) | **POST** /v1/user_terminal/laptop | Laptop Impact
[**monitor_impact_v1_user_terminal_monitor_get**](UserTerminalApi.md#monitor_impact_v1_user_terminal_monitor_get) | **GET** /v1/user_terminal/monitor | Monitor Impact
[**monitor_impact_v1_user_terminal_monitor_post**](UserTerminalApi.md#monitor_impact_v1_user_terminal_monitor_post) | **POST** /v1/user_terminal/monitor | Monitor Impact
[**server_get_all_archetype_name_v1_user_terminal_archetypes_get**](UserTerminalApi.md#server_get_all_archetype_name_v1_user_terminal_archetypes_get) | **GET** /v1/user_terminal/archetypes | Server Get All Archetype Name
[**smartphone_impact_v1_user_terminal_smartphone_get**](UserTerminalApi.md#smartphone_impact_v1_user_terminal_smartphone_get) | **GET** /v1/user_terminal/smartphone | Smartphone Impact
[**smartphone_impact_v1_user_terminal_smartphone_post**](UserTerminalApi.md#smartphone_impact_v1_user_terminal_smartphone_post) | **POST** /v1/user_terminal/smartphone | Smartphone Impact
[**tablet_impact_v1_user_terminal_tablet_get**](UserTerminalApi.md#tablet_impact_v1_user_terminal_tablet_get) | **GET** /v1/user_terminal/tablet | Tablet Impact
[**tablet_impact_v1_user_terminal_tablet_post**](UserTerminalApi.md#tablet_impact_v1_user_terminal_tablet_post) | **POST** /v1/user_terminal/tablet | Tablet Impact
[**television_impact_v1_user_terminal_television_get**](UserTerminalApi.md#television_impact_v1_user_terminal_television_get) | **GET** /v1/user_terminal/television | Television Impact
[**television_impact_v1_user_terminal_television_post**](UserTerminalApi.md#television_impact_v1_user_terminal_television_post) | **POST** /v1/user_terminal/television | Television Impact
[**usb_stick_impact_v1_user_terminal_usb_stick_get**](UserTerminalApi.md#usb_stick_impact_v1_user_terminal_usb_stick_get) | **GET** /v1/user_terminal/usb_stick | Usb Stick Impact
[**usb_stick_impact_v1_user_terminal_usb_stick_post**](UserTerminalApi.md#usb_stick_impact_v1_user_terminal_usb_stick_post) | **POST** /v1/user_terminal/usb_stick | Usb Stick Impact
[**user_terminal_get_all_categories_v1_user_terminal_all_categories_get**](UserTerminalApi.md#user_terminal_get_all_categories_v1_user_terminal_all_categories_get) | **GET** /v1/user_terminal/all_categories | User Terminal Get All Categories
[**user_terminal_get_all_subcategories_v1_user_terminal_all_subcategories_get**](UserTerminalApi.md#user_terminal_get_all_subcategories_v1_user_terminal_all_subcategories_get) | **GET** /v1/user_terminal/all_subcategories | User Terminal Get All Subcategories



## box_impact_v1_user_terminal_box_get

> serde_json::Value box_impact_v1_user_terminal_box_get(archetype, verbose, allocation, criteria)
Box Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to box-default]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## box_impact_v1_user_terminal_box_post

> serde_json::Value box_impact_v1_user_terminal_box_post(verbose, allocation, archetype, criteria, r#box)
Box Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to box-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**r#box** | Option<[**Box**](Box.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## desktop_impact_v1_user_terminal_desktop_get

> serde_json::Value desktop_impact_v1_user_terminal_desktop_get(archetype, verbose, allocation, criteria)
Desktop Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to desktop-pro]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## desktop_impact_v1_user_terminal_desktop_post

> serde_json::Value desktop_impact_v1_user_terminal_desktop_post(verbose, allocation, archetype, criteria, desktop)
Desktop Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to desktop-pro]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**desktop** | Option<[**Desktop**](Desktop.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_hdd_impact_v1_user_terminal_external_hdd_get

> serde_json::Value external_hdd_impact_v1_user_terminal_external_hdd_get(archetype, verbose, allocation, criteria)
External Hdd Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to hdd-default]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_hdd_impact_v1_user_terminal_external_hdd_post

> serde_json::Value external_hdd_impact_v1_user_terminal_external_hdd_post(verbose, allocation, archetype, criteria, external_hdd)
External Hdd Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to hdd-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**external_hdd** | Option<[**ExternalHdd**](ExternalHdd.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_ssd_impact_v1_user_terminal_external_ssd_get

> serde_json::Value external_ssd_impact_v1_user_terminal_external_ssd_get(archetype, verbose, allocation, criteria)
External Ssd Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to external-ssd-default]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_ssd_impact_v1_user_terminal_external_ssd_post

> serde_json::Value external_ssd_impact_v1_user_terminal_external_ssd_post(verbose, allocation, archetype, criteria, external_ssd)
External Ssd Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to external-ssd-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**external_ssd** | Option<[**ExternalSsd**](ExternalSsd.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## laptop_impact_v1_user_terminal_laptop_get

> serde_json::Value laptop_impact_v1_user_terminal_laptop_get(archetype, verbose, allocation, criteria)
Laptop Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to laptop-pro]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## laptop_impact_v1_user_terminal_laptop_post

> serde_json::Value laptop_impact_v1_user_terminal_laptop_post(verbose, allocation, archetype, criteria, laptop)
Laptop Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to laptop-pro]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**laptop** | Option<[**Laptop**](Laptop.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitor_impact_v1_user_terminal_monitor_get

> serde_json::Value monitor_impact_v1_user_terminal_monitor_get(archetype, verbose, allocation, criteria)
Monitor Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to monitor-default]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitor_impact_v1_user_terminal_monitor_post

> serde_json::Value monitor_impact_v1_user_terminal_monitor_post(verbose, allocation, archetype, criteria, monitor)
Monitor Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to monitor-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**monitor** | Option<[**Monitor**](Monitor.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_get_all_archetype_name_v1_user_terminal_archetypes_get

> serde_json::Value server_get_all_archetype_name_v1_user_terminal_archetypes_get(name)
Server Get All Archetype Name

# ✔️ Get all the available user terminal archetype for a given user terminal name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |[default to laptop]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smartphone_impact_v1_user_terminal_smartphone_get

> serde_json::Value smartphone_impact_v1_user_terminal_smartphone_get(archetype, verbose, allocation, criteria)
Smartphone Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to smartphone-default]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smartphone_impact_v1_user_terminal_smartphone_post

> serde_json::Value smartphone_impact_v1_user_terminal_smartphone_post(verbose, allocation, archetype, criteria, smartphone)
Smartphone Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to smartphone-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**smartphone** | Option<[**Smartphone**](Smartphone.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tablet_impact_v1_user_terminal_tablet_get

> serde_json::Value tablet_impact_v1_user_terminal_tablet_get(archetype, verbose, allocation, criteria)
Tablet Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to tablet-default]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tablet_impact_v1_user_terminal_tablet_post

> serde_json::Value tablet_impact_v1_user_terminal_tablet_post(verbose, allocation, archetype, criteria, tablet)
Tablet Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to tablet-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**tablet** | Option<[**Tablet**](Tablet.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## television_impact_v1_user_terminal_television_get

> serde_json::Value television_impact_v1_user_terminal_television_get(archetype, verbose, allocation, criteria)
Television Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to tv-pro]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## television_impact_v1_user_terminal_television_post

> serde_json::Value television_impact_v1_user_terminal_television_post(verbose, allocation, archetype, criteria, television)
Television Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to tv-pro]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**television** | Option<[**Television**](Television.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usb_stick_impact_v1_user_terminal_usb_stick_get

> serde_json::Value usb_stick_impact_v1_user_terminal_usb_stick_get(archetype, verbose, allocation, criteria)
Usb Stick Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to usb-stick-default]
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usb_stick_impact_v1_user_terminal_usb_stick_post

> serde_json::Value usb_stick_impact_v1_user_terminal_usb_stick_post(verbose, allocation, archetype, criteria, usb_stick)
Usb Stick Impact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**allocation** | Option<[**crate::models::Allocation**](.md)> |  |  |[default to TOTAL]
**archetype** | Option<**String**> |  |  |[default to usb-stick-default]
**criteria** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["gwp","adp","pe"]]
**usb_stick** | Option<[**UsbStick**](UsbStick.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_terminal_get_all_categories_v1_user_terminal_all_categories_get

> serde_json::Value user_terminal_get_all_categories_v1_user_terminal_all_categories_get()
User Terminal Get All Categories

# ✔️ Get all the available user terminal

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


## user_terminal_get_all_subcategories_v1_user_terminal_all_subcategories_get

> serde_json::Value user_terminal_get_all_subcategories_v1_user_terminal_all_subcategories_get(device_type)
User Terminal Get All Subcategories

# ✔️ Get all the available user terminal subcategories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_type** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

