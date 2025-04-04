# \ServicesApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_service_config**](ServicesApi.md#delete_service_config) | **DELETE** /services/{serviceId}/config | Deletes a service configuration for given service ID and returns the old configuration.
[**get_service_config**](ServicesApi.md#get_service_config) | **GET** /services/{serviceId}/config | Get service configuration for given service ID.
[**get_service_context**](ServicesApi.md#get_service_context) | **GET** /services/{serviceId}/contexts | Get existing multiple context service configurations for the given factory PID.
[**get_services**](ServicesApi.md#get_services) | **GET** /services | Get all configurable services.
[**get_services_by_id**](ServicesApi.md#get_services_by_id) | **GET** /services/{serviceId} | Get configurable service for given service ID.
[**update_service_config**](ServicesApi.md#update_service_config) | **PUT** /services/{serviceId}/config | Updates a service configuration for given service ID and returns the old configuration.



## delete_service_config

> String delete_service_config(service_id)
Deletes a service configuration for given service ID and returns the old configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | service ID | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_config

> String get_service_config(service_id)
Get service configuration for given service ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | service ID | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_context

> Vec<models::ConfigurableServiceDto> get_service_context(service_id, accept_language)
Get existing multiple context service configurations for the given factory PID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | service ID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::ConfigurableServiceDto>**](ConfigurableServiceDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> Vec<models::ConfigurableServiceDto> get_services(accept_language)
Get all configurable services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::ConfigurableServiceDto>**](ConfigurableServiceDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services_by_id

> models::ConfigurableServiceDto get_services_by_id(service_id, accept_language)
Get configurable service for given service ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | service ID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::ConfigurableServiceDto**](ConfigurableServiceDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_config

> String update_service_config(service_id, accept_language, request_body)
Updates a service configuration for given service ID and returns the old configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | service ID | [required] |
**accept_language** | Option<**String**> | language |  |
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  |  |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

