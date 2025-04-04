# \UiApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_ui_component_to_namespace**](UiApi.md#add_ui_component_to_namespace) | **POST** /ui/components/{namespace} | Add a UI component in the specified namespace.
[**get_registered_ui_components_in_namespace**](UiApi.md#get_registered_ui_components_in_namespace) | **GET** /ui/components/{namespace} | Get all registered UI components in the specified namespace.
[**get_ui_component_in_namespace**](UiApi.md#get_ui_component_in_namespace) | **GET** /ui/components/{namespace}/{componentUID} | Get a specific UI component in the specified namespace.
[**get_ui_tiles**](UiApi.md#get_ui_tiles) | **GET** /ui/tiles | Get all registered UI tiles.
[**remove_ui_component_from_namespace**](UiApi.md#remove_ui_component_from_namespace) | **DELETE** /ui/components/{namespace}/{componentUID} | Remove a specific UI component in the specified namespace.
[**update_ui_component_in_namespace**](UiApi.md#update_ui_component_in_namespace) | **PUT** /ui/components/{namespace}/{componentUID} | Update a specific UI component in the specified namespace.



## add_ui_component_to_namespace

> models::RootUiComponent add_ui_component_to_namespace(namespace, root_ui_component)
Add a UI component in the specified namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**root_ui_component** | Option<[**RootUiComponent**](RootUiComponent.md)> |  |  |

### Return type

[**models::RootUiComponent**](RootUIComponent.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registered_ui_components_in_namespace

> Vec<models::RootUiComponent> get_registered_ui_components_in_namespace(namespace, summary)
Get all registered UI components in the specified namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**summary** | Option<**bool**> | summary fields only |  |

### Return type

[**Vec<models::RootUiComponent>**](RootUIComponent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ui_component_in_namespace

> models::RootUiComponent get_ui_component_in_namespace(namespace, component_uid)
Get a specific UI component in the specified namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**component_uid** | **String** |  | [required] |

### Return type

[**models::RootUiComponent**](RootUIComponent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ui_tiles

> Vec<models::TileDto> get_ui_tiles()
Get all registered UI tiles.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TileDto>**](TileDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_ui_component_from_namespace

> remove_ui_component_from_namespace(namespace, component_uid)
Remove a specific UI component in the specified namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**component_uid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ui_component_in_namespace

> models::RootUiComponent update_ui_component_in_namespace(namespace, component_uid, root_ui_component)
Update a specific UI component in the specified namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**component_uid** | **String** |  | [required] |
**root_ui_component** | Option<[**RootUiComponent**](RootUiComponent.md)> |  |  |

### Return type

[**models::RootUiComponent**](RootUIComponent.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

