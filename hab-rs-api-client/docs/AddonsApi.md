# \AddonsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_addon_by_id**](AddonsApi.md#get_addon_by_id) | **GET** /addons/{addonId} | Get add-on with given ID.
[**get_addon_configuration**](AddonsApi.md#get_addon_configuration) | **GET** /addons/{addonId}/config | Get add-on configuration for given add-on ID.
[**get_addon_services**](AddonsApi.md#get_addon_services) | **GET** /addons/types | Get add-on services.
[**get_addon_types**](AddonsApi.md#get_addon_types) | **GET** /addons/services | Get all add-on types.
[**get_addons**](AddonsApi.md#get_addons) | **GET** /addons | Get all add-ons.
[**get_suggested_addons**](AddonsApi.md#get_suggested_addons) | **GET** /addons/suggestions | Get suggested add-ons to be installed.
[**install_addon_by_id**](AddonsApi.md#install_addon_by_id) | **POST** /addons/{addonId}/install | Installs the add-on with the given ID.
[**install_addon_from_url**](AddonsApi.md#install_addon_from_url) | **POST** /addons/url/{url}/install | Installs the add-on from the given URL.
[**uninstall_addon**](AddonsApi.md#uninstall_addon) | **POST** /addons/{addonId}/uninstall | Uninstalls the add-on with the given ID.
[**update_addon_configuration**](AddonsApi.md#update_addon_configuration) | **PUT** /addons/{addonId}/config | Updates an add-on configuration for given ID and returns the old configuration.



## get_addon_by_id

> models::Addon get_addon_by_id(addon_id, accept_language, service_id)
Get add-on with given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_id** | **String** | addon ID | [required] |
**accept_language** | Option<**String**> | language |  |
**service_id** | Option<**String**> | service ID |  |

### Return type

[**models::Addon**](Addon.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon_configuration

> String get_addon_configuration(addon_id, service_id)
Get add-on configuration for given add-on ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_id** | **String** | addon ID | [required] |
**service_id** | Option<**String**> | service ID |  |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon_services

> Vec<models::AddonType> get_addon_services(accept_language, service_id)
Get add-on services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**service_id** | Option<**String**> | service ID |  |

### Return type

[**Vec<models::AddonType>**](AddonType.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon_types

> Vec<models::AddonType> get_addon_types(accept_language)
Get all add-on types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::AddonType>**](AddonType.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addons

> Vec<models::Addon> get_addons(accept_language, service_id)
Get all add-ons.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**service_id** | Option<**String**> | service ID |  |

### Return type

[**Vec<models::Addon>**](Addon.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_suggested_addons

> Vec<models::Addon> get_suggested_addons(accept_language)
Get suggested add-ons to be installed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::Addon>**](Addon.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_addon_by_id

> install_addon_by_id(addon_id, service_id)
Installs the add-on with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_id** | **String** | addon ID | [required] |
**service_id** | Option<**String**> | service ID |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_addon_from_url

> install_addon_from_url(url)
Installs the add-on from the given URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | addon install URL | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_addon

> uninstall_addon(addon_id, service_id)
Uninstalls the add-on with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_id** | **String** | addon ID | [required] |
**service_id** | Option<**String**> | service ID |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_addon_configuration

> String update_addon_configuration(addon_id, service_id, request_body)
Updates an add-on configuration for given ID and returns the old configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_id** | **String** | Add-on id | [required] |
**service_id** | Option<**String**> | service ID |  |
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  |  |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

