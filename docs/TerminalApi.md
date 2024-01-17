# \TerminalApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**box_get_all_archetype_name_v1_terminal_box_archetypes_get**](TerminalApi.md#box_get_all_archetype_name_v1_terminal_box_archetypes_get) | **GET** /v1/terminal/box/archetypes | Box Get All Archetype Name
[**box_get_archetype_config_v1_terminal_box_archetype_config_get**](TerminalApi.md#box_get_archetype_config_v1_terminal_box_archetype_config_get) | **GET** /v1/terminal/box/archetype_config | Box Get Archetype Config
[**box_impact_v1_terminal_box_get**](TerminalApi.md#box_impact_v1_terminal_box_get) | **GET** /v1/terminal/box | Box Impact
[**box_impact_v1_terminal_box_post**](TerminalApi.md#box_impact_v1_terminal_box_post) | **POST** /v1/terminal/box | Box Impact
[**desktop_get_all_archetype_name_v1_terminal_desktop_archetypes_get**](TerminalApi.md#desktop_get_all_archetype_name_v1_terminal_desktop_archetypes_get) | **GET** /v1/terminal/desktop/archetypes | Desktop Get All Archetype Name
[**desktop_get_archetype_config_v1_terminal_desktop_archetype_config_get**](TerminalApi.md#desktop_get_archetype_config_v1_terminal_desktop_archetype_config_get) | **GET** /v1/terminal/desktop/archetype_config | Desktop Get Archetype Config
[**desktop_impact_v1_terminal_desktop_get**](TerminalApi.md#desktop_impact_v1_terminal_desktop_get) | **GET** /v1/terminal/desktop | Desktop Impact
[**desktop_impact_v1_terminal_desktop_post**](TerminalApi.md#desktop_impact_v1_terminal_desktop_post) | **POST** /v1/terminal/desktop | Desktop Impact
[**laptop_get_all_archetype_name_v1_terminal_laptop_archetypes_get**](TerminalApi.md#laptop_get_all_archetype_name_v1_terminal_laptop_archetypes_get) | **GET** /v1/terminal/laptop/archetypes | Laptop Get All Archetype Name
[**laptop_get_archetype_config_v1_terminal_laptop_archetype_config_get**](TerminalApi.md#laptop_get_archetype_config_v1_terminal_laptop_archetype_config_get) | **GET** /v1/terminal/laptop/archetype_config | Laptop Get Archetype Config
[**laptop_impact_v1_terminal_laptop_get**](TerminalApi.md#laptop_impact_v1_terminal_laptop_get) | **GET** /v1/terminal/laptop | Laptop Impact
[**laptop_impact_v1_terminal_laptop_post**](TerminalApi.md#laptop_impact_v1_terminal_laptop_post) | **POST** /v1/terminal/laptop | Laptop Impact
[**smartphone_get_all_archetype_name_v1_terminal_smartphone_archetypes_get**](TerminalApi.md#smartphone_get_all_archetype_name_v1_terminal_smartphone_archetypes_get) | **GET** /v1/terminal/smartphone/archetypes | Smartphone Get All Archetype Name
[**smartphone_get_archetype_config_v1_terminal_smartphone_archetype_config_get**](TerminalApi.md#smartphone_get_archetype_config_v1_terminal_smartphone_archetype_config_get) | **GET** /v1/terminal/smartphone/archetype_config | Smartphone Get Archetype Config
[**smartphone_impact_v1_terminal_smartphone_get**](TerminalApi.md#smartphone_impact_v1_terminal_smartphone_get) | **GET** /v1/terminal/smartphone | Smartphone Impact
[**smartphone_impact_v1_terminal_smartphone_post**](TerminalApi.md#smartphone_impact_v1_terminal_smartphone_post) | **POST** /v1/terminal/smartphone | Smartphone Impact
[**tablet_get_all_archetype_name_v1_terminal_tablet_archetypes_get**](TerminalApi.md#tablet_get_all_archetype_name_v1_terminal_tablet_archetypes_get) | **GET** /v1/terminal/tablet/archetypes | Tablet Get All Archetype Name
[**tablet_get_archetype_config_v1_terminal_tablet_archetype_config_get**](TerminalApi.md#tablet_get_archetype_config_v1_terminal_tablet_archetype_config_get) | **GET** /v1/terminal/tablet/archetype_config | Tablet Get Archetype Config
[**tablet_impact_v1_terminal_tablet_get**](TerminalApi.md#tablet_impact_v1_terminal_tablet_get) | **GET** /v1/terminal/tablet | Tablet Impact
[**tablet_impact_v1_terminal_tablet_post**](TerminalApi.md#tablet_impact_v1_terminal_tablet_post) | **POST** /v1/terminal/tablet | Tablet Impact
[**television_get_all_archetype_name_v1_terminal_television_archetypes_get**](TerminalApi.md#television_get_all_archetype_name_v1_terminal_television_archetypes_get) | **GET** /v1/terminal/television/archetypes | Television Get All Archetype Name
[**television_get_archetype_config_v1_terminal_television_archetype_config_get**](TerminalApi.md#television_get_archetype_config_v1_terminal_television_archetype_config_get) | **GET** /v1/terminal/television/archetype_config | Television Get Archetype Config
[**television_impact_v1_terminal_television_get**](TerminalApi.md#television_impact_v1_terminal_television_get) | **GET** /v1/terminal/television | Television Impact
[**television_impact_v1_terminal_television_post**](TerminalApi.md#television_impact_v1_terminal_television_post) | **POST** /v1/terminal/television | Television Impact
[**terminal_get_all_categories_v1_terminal_all_get**](TerminalApi.md#terminal_get_all_categories_v1_terminal_all_get) | **GET** /v1/terminal/all | Terminal Get All Categories



## box_get_all_archetype_name_v1_terminal_box_archetypes_get

> serde_json::Value box_get_all_archetype_name_v1_terminal_box_archetypes_get()
Box Get All Archetype Name

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


## box_get_archetype_config_v1_terminal_box_archetype_config_get

> serde_json::Value box_get_archetype_config_v1_terminal_box_archetype_config_get(archetype)
Box Get Archetype Config

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


## box_impact_v1_terminal_box_get

> serde_json::Value box_impact_v1_terminal_box_get(archetype, verbose, duration, criteria)
Box Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to box-default]
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


## box_impact_v1_terminal_box_post

> serde_json::Value box_impact_v1_terminal_box_post(verbose, duration, archetype, criteria, r#box)
Box Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
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


## desktop_get_all_archetype_name_v1_terminal_desktop_archetypes_get

> serde_json::Value desktop_get_all_archetype_name_v1_terminal_desktop_archetypes_get()
Desktop Get All Archetype Name

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


## desktop_get_archetype_config_v1_terminal_desktop_archetype_config_get

> serde_json::Value desktop_get_archetype_config_v1_terminal_desktop_archetype_config_get(archetype)
Desktop Get Archetype Config

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


## desktop_impact_v1_terminal_desktop_get

> serde_json::Value desktop_impact_v1_terminal_desktop_get(archetype, verbose, duration, criteria)
Desktop Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to desktop-pro]
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


## desktop_impact_v1_terminal_desktop_post

> serde_json::Value desktop_impact_v1_terminal_desktop_post(verbose, duration, archetype, criteria, desktop)
Desktop Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
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


## laptop_get_all_archetype_name_v1_terminal_laptop_archetypes_get

> serde_json::Value laptop_get_all_archetype_name_v1_terminal_laptop_archetypes_get()
Laptop Get All Archetype Name

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


## laptop_get_archetype_config_v1_terminal_laptop_archetype_config_get

> serde_json::Value laptop_get_archetype_config_v1_terminal_laptop_archetype_config_get(archetype)
Laptop Get Archetype Config

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


## laptop_impact_v1_terminal_laptop_get

> serde_json::Value laptop_impact_v1_terminal_laptop_get(archetype, verbose, duration, criteria)
Laptop Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to laptop-pro]
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


## laptop_impact_v1_terminal_laptop_post

> serde_json::Value laptop_impact_v1_terminal_laptop_post(verbose, duration, archetype, criteria, laptop)
Laptop Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
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


## smartphone_get_all_archetype_name_v1_terminal_smartphone_archetypes_get

> serde_json::Value smartphone_get_all_archetype_name_v1_terminal_smartphone_archetypes_get()
Smartphone Get All Archetype Name

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


## smartphone_get_archetype_config_v1_terminal_smartphone_archetype_config_get

> serde_json::Value smartphone_get_archetype_config_v1_terminal_smartphone_archetype_config_get(archetype)
Smartphone Get Archetype Config

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


## smartphone_impact_v1_terminal_smartphone_get

> serde_json::Value smartphone_impact_v1_terminal_smartphone_get(archetype, verbose, duration, criteria)
Smartphone Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to smartphone-default]
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


## smartphone_impact_v1_terminal_smartphone_post

> serde_json::Value smartphone_impact_v1_terminal_smartphone_post(verbose, duration, archetype, criteria, smartphone)
Smartphone Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
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


## tablet_get_all_archetype_name_v1_terminal_tablet_archetypes_get

> serde_json::Value tablet_get_all_archetype_name_v1_terminal_tablet_archetypes_get()
Tablet Get All Archetype Name

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


## tablet_get_archetype_config_v1_terminal_tablet_archetype_config_get

> serde_json::Value tablet_get_archetype_config_v1_terminal_tablet_archetype_config_get(archetype)
Tablet Get Archetype Config

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


## tablet_impact_v1_terminal_tablet_get

> serde_json::Value tablet_impact_v1_terminal_tablet_get(archetype, verbose, duration, criteria)
Tablet Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to tablet-default]
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


## tablet_impact_v1_terminal_tablet_post

> serde_json::Value tablet_impact_v1_terminal_tablet_post(verbose, duration, archetype, criteria, tablet)
Tablet Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
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


## television_get_all_archetype_name_v1_terminal_television_archetypes_get

> serde_json::Value television_get_all_archetype_name_v1_terminal_television_archetypes_get()
Television Get All Archetype Name

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


## television_get_archetype_config_v1_terminal_television_archetype_config_get

> serde_json::Value television_get_archetype_config_v1_terminal_television_archetype_config_get(archetype)
Television Get Archetype Config

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


## television_impact_v1_terminal_television_get

> serde_json::Value television_impact_v1_terminal_television_get(archetype, verbose, duration, criteria)
Television Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |[default to tv-pro]
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


## television_impact_v1_terminal_television_post

> serde_json::Value television_impact_v1_terminal_television_post(verbose, duration, archetype, criteria, television)
Television Impact

# ✔ Terminal impacts ### Features  👄 Verbose  🔃 Auto-complete  🔨 Embedded  The impacts values are fix🔌 Usage  * ⏺️  Given  * 📋 Archetype  ⏬ Allocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**duration** | Option<**f32**> |  |  |
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


## terminal_get_all_categories_v1_terminal_all_get

> serde_json::Value terminal_get_all_categories_v1_terminal_all_get()
Terminal Get All Categories

# ✔️ Get all the available user terminal router

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

