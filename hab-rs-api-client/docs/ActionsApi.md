# \ActionsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**execute_thing_action**](ActionsApi.md#execute_thing_action) | **POST** /actions/{thingUID}/{actionUid} | Executes a thing action.
[**get_available_actions_for_thing**](ActionsApi.md#get_available_actions_for_thing) | **GET** /actions/{thingUID} | Get all available actions for provided thing UID



## execute_thing_action

> String execute_thing_action(thing_uid, action_uid, accept_language, request_body)
Executes a thing action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |
**action_uid** | **String** | action type UID (including scope, separated by '.') | [required] |
**accept_language** | Option<**String**> | language |  |
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | action inputs as map (parameter name as key / argument as value) |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_actions_for_thing

> Vec<models::ThingActionDto> get_available_actions_for_thing(thing_uid, accept_language)
Get all available actions for provided thing UID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::ThingActionDto>**](ThingActionDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

