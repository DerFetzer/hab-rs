# \TagsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_semantic_tag**](TagsApi.md#create_semantic_tag) | **POST** /tags | Creates a new semantic tag and adds it to the registry.
[**get_semantic_tag_and_sub_tags**](TagsApi.md#get_semantic_tag_and_sub_tags) | **GET** /tags/{tagId} | Gets a semantic tag and its sub tags.
[**get_semantic_tags**](TagsApi.md#get_semantic_tags) | **GET** /tags | Get all available semantic tags.
[**remove_semantic_tag**](TagsApi.md#remove_semantic_tag) | **DELETE** /tags/{tagId} | Removes a semantic tag and its sub tags from the registry.
[**update_semantic_tag**](TagsApi.md#update_semantic_tag) | **PUT** /tags/{tagId} | Updates a semantic tag.



## create_semantic_tag

> serde_json::Value create_semantic_tag(body, accept_language)
Creates a new semantic tag and adds it to the registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** | tag data | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_semantic_tag_and_sub_tags

> Vec<serde_json::Value> get_semantic_tag_and_sub_tags(tag_id, accept_language)
Gets a semantic tag and its sub tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | tag id | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_semantic_tags

> Vec<serde_json::Value> get_semantic_tags(accept_language)
Get all available semantic tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_semantic_tag

> remove_semantic_tag(tag_id, accept_language)
Removes a semantic tag and its sub tags from the registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | tag id | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_semantic_tag

> serde_json::Value update_semantic_tag(tag_id, body, accept_language)
Updates a semantic tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | tag id | [required] |
**body** | **serde_json::Value** | tag data | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

