# \EventsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events**](EventsApi.md#get_events) | **GET** /events | Get all events.
[**init_new_state_tacker**](EventsApi.md#init_new_state_tacker) | **GET** /events/states | Initiates a new item state tracker connection
[**update_item_list_for_state_updates**](EventsApi.md#update_item_list_for_state_updates) | **POST** /events/states/{connectionId} | Changes the list of items a SSE connection will receive state updates to.



## get_events

> get_events(topics)
Get all events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topics** | Option<**String**> | topics |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## init_new_state_tacker

> init_new_state_tacker()
Initiates a new item state tracker connection

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item_list_for_state_updates

> update_item_list_for_state_updates(connection_id, request_body)
Changes the list of items a SSE connection will receive state updates to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |
**request_body** | Option<[**Vec<String>**](String.md)> | items |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

