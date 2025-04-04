# \ConfigDescriptionsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_config_description_by_uri**](ConfigDescriptionsApi.md#get_config_description_by_uri) | **GET** /config-descriptions/{uri} | Gets a config description by URI.
[**get_config_descriptions**](ConfigDescriptionsApi.md#get_config_descriptions) | **GET** /config-descriptions | Gets all available config descriptions.



## get_config_description_by_uri

> models::ConfigDescriptionDto get_config_description_by_uri(uri, accept_language)
Gets a config description by URI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uri** | **String** | uri | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::ConfigDescriptionDto**](ConfigDescriptionDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_descriptions

> Vec<models::ConfigDescriptionDto> get_config_descriptions(accept_language, scheme)
Gets all available config descriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**scheme** | Option<**String**> | scheme filter |  |

### Return type

[**Vec<models::ConfigDescriptionDto>**](ConfigDescriptionDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

