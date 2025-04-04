# \DiscoveryApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bindings_with_discovery_support**](DiscoveryApi.md#get_bindings_with_discovery_support) | **GET** /discovery | Gets all bindings that support discovery.
[**get_discovery_services_info**](DiscoveryApi.md#get_discovery_services_info) | **GET** /discovery/bindings/{bindingId}/info | Gets information about the discovery services for a binding.
[**scan**](DiscoveryApi.md#scan) | **POST** /discovery/bindings/{bindingId}/scan | Starts asynchronous discovery process for a binding and returns the timeout in seconds of the discovery operation.



## get_bindings_with_discovery_support

> Vec<String> get_bindings_with_discovery_support()
Gets all bindings that support discovery.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discovery_services_info

> models::DiscoveryInfoDto get_discovery_services_info(binding_id, accept_language)
Gets information about the discovery services for a binding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**binding_id** | **String** | binding Id | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::DiscoveryInfoDto**](DiscoveryInfoDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan

> i32 scan(binding_id, input)
Starts asynchronous discovery process for a binding and returns the timeout in seconds of the discovery operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**binding_id** | **String** | binding Id | [required] |
**input** | Option<**String**> | input parameter to start the discovery |  |

### Return type

**i32**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

