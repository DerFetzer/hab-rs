# \ItemsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_member_to_group_item**](ItemsApi.md#add_member_to_group_item) | **PUT** /items/{itemName}/members/{memberItemName} | Adds a new member to a group item.
[**add_metadata_to_item**](ItemsApi.md#add_metadata_to_item) | **PUT** /items/{itemname}/metadata/{namespace} | Adds metadata to an item.
[**add_or_update_item_in_registry**](ItemsApi.md#add_or_update_item_in_registry) | **PUT** /items/{itemname} | Adds a new item to the registry or updates the existing item.
[**add_or_update_items_in_registry**](ItemsApi.md#add_or_update_items_in_registry) | **PUT** /items | Adds a list of items to the registry or updates the existing items.
[**add_tag_to_item**](ItemsApi.md#add_tag_to_item) | **PUT** /items/{itemname}/tags/{tag} | Adds a tag to an item.
[**get_item_by_name**](ItemsApi.md#get_item_by_name) | **GET** /items/{itemname} | Gets a single item.
[**get_item_namespaces**](ItemsApi.md#get_item_namespaces) | **GET** /items/{itemname}/metadata/namespaces | Gets the namespace of an item.
[**get_item_state1**](ItemsApi.md#get_item_state1) | **GET** /items/{itemname}/state | Gets the state of an item.
[**get_items**](ItemsApi.md#get_items) | **GET** /items | Get all available items.
[**get_semantic_item**](ItemsApi.md#get_semantic_item) | **GET** /items/{itemName}/semantic/{semanticClass} | Gets the item which defines the requested semantics of an item.
[**purge_database**](ItemsApi.md#purge_database) | **POST** /items/metadata/purge | Remove unused/orphaned metadata.
[**remove_item_from_registry**](ItemsApi.md#remove_item_from_registry) | **DELETE** /items/{itemname} | Removes an item from the registry.
[**remove_member_from_group_item**](ItemsApi.md#remove_member_from_group_item) | **DELETE** /items/{itemName}/members/{memberItemName} | Removes an existing member from a group item.
[**remove_metadata_from_item**](ItemsApi.md#remove_metadata_from_item) | **DELETE** /items/{itemname}/metadata/{namespace} | Removes metadata from an item.
[**remove_tag_from_item**](ItemsApi.md#remove_tag_from_item) | **DELETE** /items/{itemname}/tags/{tag} | Removes a tag from an item.
[**send_item_command**](ItemsApi.md#send_item_command) | **POST** /items/{itemname} | Sends a command to an item.
[**update_item_state**](ItemsApi.md#update_item_state) | **PUT** /items/{itemname}/state | Updates the state of an item.



## add_member_to_group_item

> add_member_to_group_item(item_name, member_item_name)
Adds a new member to a group item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_name** | **String** | item name | [required] |
**member_item_name** | **String** | member item name | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_metadata_to_item

> add_metadata_to_item(itemname, namespace, metadata_dto)
Adds metadata to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**namespace** | **String** | namespace | [required] |
**metadata_dto** | [**MetadataDto**](MetadataDto.md) | metadata | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_or_update_item_in_registry

> models::EnrichedItemDto add_or_update_item_in_registry(itemname, group_item_dto, accept_language)
Adds a new item to the registry or updates the existing item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**group_item_dto** | [**GroupItemDto**](GroupItemDto.md) | item data | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::EnrichedItemDto**](EnrichedItemDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_or_update_items_in_registry

> String add_or_update_items_in_registry(group_item_dto)
Adds a list of items to the registry or updates the existing items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_item_dto** | [**Vec<models::GroupItemDto>**](GroupItemDTO.md) | array of item data | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tag_to_item

> add_tag_to_item(itemname, tag)
Adds a tag to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**tag** | **String** | tag | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_by_name

> models::EnrichedItemDto get_item_by_name(itemname, accept_language, metadata, recursive)
Gets a single item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**accept_language** | Option<**String**> | language |  |
**metadata** | Option<**String**> | metadata selector - a comma separated list or a regular expression (returns all if no value given) |  |[default to .*]
**recursive** | Option<**bool**> | get member items if the item is a group item |  |[default to true]

### Return type

[**models::EnrichedItemDto**](EnrichedItemDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_namespaces

> String get_item_namespaces(itemname, accept_language)
Gets the namespace of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_state1

> String get_item_state1(itemname)
Gets the state of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_items

> Vec<models::EnrichedItemDto> get_items(accept_language, r#type, tags, metadata, recursive, fields, static_data_only)
Get all available items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**r#type** | Option<**String**> | item type filter |  |
**tags** | Option<**String**> | item tag filter |  |
**metadata** | Option<**String**> | metadata selector - a comma separated list or a regular expression (returns all if no value given) |  |[default to .*]
**recursive** | Option<**bool**> | get member items recursively |  |[default to false]
**fields** | Option<**String**> | limit output to the given fields (comma separated) |  |
**static_data_only** | Option<**bool**> | provides a cacheable list of values not expected to change regularly and checks the If-Modified-Since header, all other parameters are ignored except \"metadata\" |  |[default to false]

### Return type

[**Vec<models::EnrichedItemDto>**](EnrichedItemDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_semantic_item

> get_semantic_item(item_name, semantic_class, accept_language)
Gets the item which defines the requested semantics of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_name** | **String** | item name | [required] |
**semantic_class** | **String** | semantic class | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_database

> purge_database()
Remove unused/orphaned metadata.

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


## remove_item_from_registry

> remove_item_from_registry(itemname)
Removes an item from the registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_member_from_group_item

> remove_member_from_group_item(item_name, member_item_name)
Removes an existing member from a group item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_name** | **String** | item name | [required] |
**member_item_name** | **String** | member item name | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_metadata_from_item

> remove_metadata_from_item(itemname, namespace)
Removes metadata from an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**namespace** | **String** | namespace | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_tag_from_item

> remove_tag_from_item(itemname, tag)
Removes a tag from an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**tag** | **String** | tag | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_item_command

> send_item_command(itemname, body)
Sends a command to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**body** | **String** | valid item command (e.g. ON, OFF, UP, DOWN, REFRESH) | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item_state

> update_item_state(itemname, body, accept_language)
Updates the state of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | item name | [required] |
**body** | **String** | valid item state (e.g. ON, OFF) | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

