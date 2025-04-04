# \ThingTypesApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_thing_type_by_id**](ThingTypesApi.md#get_thing_type_by_id) | **GET** /thing-types/{thingTypeUID} | Gets thing type by UID.
[**get_thing_types**](ThingTypesApi.md#get_thing_types) | **GET** /thing-types | Gets all available thing types without config description, channels and properties.



## get_thing_type_by_id

> models::ThingTypeDto get_thing_type_by_id(thing_type_uid, accept_language)
Gets thing type by UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_type_uid** | **String** | thingTypeUID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::ThingTypeDto**](ThingTypeDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thing_types

> Vec<models::StrippedThingTypeDto> get_thing_types(accept_language, binding_id)
Gets all available thing types without config description, channels and properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**binding_id** | Option<**String**> | filter by binding Id |  |

### Return type

[**Vec<models::StrippedThingTypeDto>**](StrippedThingTypeDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

