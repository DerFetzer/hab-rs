# \InboxApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_inbox_item_by_id**](InboxApi.md#approve_inbox_item_by_id) | **POST** /inbox/{thingUID}/approve | Approves the discovery result by adding the thing to the registry.
[**flag_inbox_item_as_ignored**](InboxApi.md#flag_inbox_item_as_ignored) | **POST** /inbox/{thingUID}/ignore | Flags a discovery result as ignored for further processing.
[**get_discovered_inbox_items**](InboxApi.md#get_discovered_inbox_items) | **GET** /inbox | Get all discovered things.
[**remove_ignore_flag_on_inbox_item**](InboxApi.md#remove_ignore_flag_on_inbox_item) | **POST** /inbox/{thingUID}/unignore | Removes ignore flag from a discovery result.
[**remove_item_from_inbox**](InboxApi.md#remove_item_from_inbox) | **DELETE** /inbox/{thingUID} | Removes the discovery result from the inbox.



## approve_inbox_item_by_id

> approve_inbox_item_by_id(thing_uid, accept_language, new_thing_id, body)
Approves the discovery result by adding the thing to the registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |
**accept_language** | Option<**String**> | language |  |
**new_thing_id** | Option<**String**> | new thing ID |  |
**body** | Option<**String**> | thing label |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flag_inbox_item_as_ignored

> flag_inbox_item_as_ignored(thing_uid)
Flags a discovery result as ignored for further processing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discovered_inbox_items

> Vec<models::DiscoveryResultDto> get_discovered_inbox_items(include_ignored)
Get all discovered things.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_ignored** | Option<**bool**> | If true, include ignored inbox entries. Defaults to true |  |[default to true]

### Return type

[**Vec<models::DiscoveryResultDto>**](DiscoveryResultDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_ignore_flag_on_inbox_item

> remove_ignore_flag_on_inbox_item(thing_uid)
Removes ignore flag from a discovery result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_item_from_inbox

> remove_item_from_inbox(thing_uid)
Removes the discovery result from the inbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

