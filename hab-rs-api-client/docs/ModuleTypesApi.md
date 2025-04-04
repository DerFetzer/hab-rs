# \ModuleTypesApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_module_type_by_id**](ModuleTypesApi.md#get_module_type_by_id) | **GET** /module-types/{moduleTypeUID} | Gets a module type corresponding to the given UID.
[**get_module_types**](ModuleTypesApi.md#get_module_types) | **GET** /module-types | Get all available module types.



## get_module_type_by_id

> models::ModuleTypeDto get_module_type_by_id(module_type_uid, accept_language)
Gets a module type corresponding to the given UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_type_uid** | **String** | moduleTypeUID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::ModuleTypeDto**](ModuleTypeDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_module_types

> Vec<models::ModuleTypeDto> get_module_types(accept_language, tags, r#type)
Get all available module types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**tags** | Option<**String**> | tags for filtering |  |
**r#type** | Option<**String**> | filtering by action, condition or trigger |  |

### Return type

[**Vec<models::ModuleTypeDto>**](ModuleTypeDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

