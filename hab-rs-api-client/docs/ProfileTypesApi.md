# \ProfileTypesApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_profile_types**](ProfileTypesApi.md#get_profile_types) | **GET** /profile-types | Gets all available profile types.



## get_profile_types

> Vec<models::ProfileTypeDto> get_profile_types(accept_language, channel_type_uid, item_type)
Gets all available profile types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**channel_type_uid** | Option<**String**> | channel type filter |  |
**item_type** | Option<**String**> | item type filter |  |

### Return type

[**Vec<models::ProfileTypeDto>**](ProfileTypeDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

