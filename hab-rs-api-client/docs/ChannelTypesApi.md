# \ChannelTypesApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_channel_type_by_uid**](ChannelTypesApi.md#get_channel_type_by_uid) | **GET** /channel-types/{channelTypeUID} | Gets channel type by UID.
[**get_channel_types**](ChannelTypesApi.md#get_channel_types) | **GET** /channel-types | Gets all available channel types.
[**get_linkable_item_types_by_channel_type_uid**](ChannelTypesApi.md#get_linkable_item_types_by_channel_type_uid) | **GET** /channel-types/{channelTypeUID}/linkableItemTypes | Gets the item types the given trigger channel type UID can be linked to.



## get_channel_type_by_uid

> models::ChannelTypeDto get_channel_type_by_uid(channel_type_uid, accept_language)
Gets channel type by UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_type_uid** | **String** | channelTypeUID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::ChannelTypeDto**](ChannelTypeDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_types

> Vec<models::ChannelTypeDto> get_channel_types(accept_language, prefixes)
Gets all available channel types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**prefixes** | Option<**String**> | filter UIDs by prefix (multiple comma-separated prefixes allowed, for example: 'system,mqtt') |  |

### Return type

[**Vec<models::ChannelTypeDto>**](ChannelTypeDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linkable_item_types_by_channel_type_uid

> Vec<String> get_linkable_item_types_by_channel_type_uid(channel_type_uid)
Gets the item types the given trigger channel type UID can be linked to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_type_uid** | **String** | channelTypeUID | [required] |

### Return type

**Vec<String>**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

