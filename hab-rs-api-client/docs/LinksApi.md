# \LinksApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_item_link**](LinksApi.md#get_item_link) | **GET** /links/{itemName}/{channelUID} | Retrieves an individual link.
[**get_item_links**](LinksApi.md#get_item_links) | **GET** /links | Gets all available links.
[**get_orphan_links**](LinksApi.md#get_orphan_links) | **GET** /links/orphans | Get orphan links between items and broken/non-existent thing channels
[**link_item_to_channel**](LinksApi.md#link_item_to_channel) | **PUT** /links/{itemName}/{channelUID} | Links an item to a channel.
[**purge_database1**](LinksApi.md#purge_database1) | **POST** /links/purge | Remove unused/orphaned links.
[**remove_all_links_for_object**](LinksApi.md#remove_all_links_for_object) | **DELETE** /links/{object} | Delete all links that refer to an item or thing.
[**unlink_item_from_channel**](LinksApi.md#unlink_item_from_channel) | **DELETE** /links/{itemName}/{channelUID} | Unlinks an item from a channel.



## get_item_link

> models::EnrichedItemChannelLinkDto get_item_link(item_name, channel_uid)
Retrieves an individual link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_name** | **String** | item name | [required] |
**channel_uid** | **String** | channel UID | [required] |

### Return type

[**models::EnrichedItemChannelLinkDto**](EnrichedItemChannelLinkDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_links

> Vec<models::EnrichedItemChannelLinkDto> get_item_links(channel_uid, item_name)
Gets all available links.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_uid** | Option<**String**> | filter by channel UID |  |
**item_name** | Option<**String**> | filter by item name |  |

### Return type

[**Vec<models::EnrichedItemChannelLinkDto>**](EnrichedItemChannelLinkDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orphan_links

> get_orphan_links()
Get orphan links between items and broken/non-existent thing channels

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_item_to_channel

> link_item_to_channel(item_name, channel_uid, item_channel_link_dto)
Links an item to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_name** | **String** | itemName | [required] |
**channel_uid** | **String** | channelUID | [required] |
**item_channel_link_dto** | Option<[**ItemChannelLinkDto**](ItemChannelLinkDto.md)> | link data |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_database1

> purge_database1()
Remove unused/orphaned links.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_all_links_for_object

> remove_all_links_for_object(object)
Delete all links that refer to an item or thing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** | item name or thing UID | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_item_from_channel

> unlink_item_from_channel(item_name, channel_uid)
Unlinks an item from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_name** | **String** | itemName | [required] |
**channel_uid** | **String** | channelUID | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

