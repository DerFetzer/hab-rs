# \PersistenceApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_item_from_persistence_service**](PersistenceApi.md#delete_item_from_persistence_service) | **DELETE** /persistence/items/{itemname} | Deletes item persistence data from a specific persistence service in a given time range.
[**delete_persistence_service_configuration**](PersistenceApi.md#delete_persistence_service_configuration) | **DELETE** /persistence/{serviceId} | Deletes a persistence service configuration.
[**get_item_data_from_persistence_service**](PersistenceApi.md#get_item_data_from_persistence_service) | **GET** /persistence/items/{itemname} | Gets item persistence data from the persistence service.
[**get_items_for_persistence_service**](PersistenceApi.md#get_items_for_persistence_service) | **GET** /persistence/items | Gets a list of items available via a specific persistence service.
[**get_persistence_service_configuration**](PersistenceApi.md#get_persistence_service_configuration) | **GET** /persistence/{serviceId} | Gets a persistence service configuration.
[**get_persistence_services**](PersistenceApi.md#get_persistence_services) | **GET** /persistence | Gets a list of persistence services.
[**put_persistence_service_configuration**](PersistenceApi.md#put_persistence_service_configuration) | **PUT** /persistence/{serviceId} | Sets a persistence service configuration.
[**store_item_data_in_persistence_service**](PersistenceApi.md#store_item_data_in_persistence_service) | **PUT** /persistence/items/{itemname} | Stores item persistence data into the persistence service.



## delete_item_from_persistence_service

> Vec<String> delete_item_from_persistence_service(service_id, itemname, starttime, endtime)
Deletes item persistence data from a specific persistence service in a given time range.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Id of the persistence service. | [required] |
**itemname** | **String** | The item name. | [required] |
**starttime** | **String** | Start of the time range to be deleted. [yyyy-MM-dd'T'HH:mm:ss.SSSZ] | [required] |
**endtime** | **String** | End of the time range to be deleted. [yyyy-MM-dd'T'HH:mm:ss.SSSZ] | [required] |

### Return type

**Vec<String>**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_persistence_service_configuration

> delete_persistence_service_configuration(service_id)
Deletes a persistence service configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Id of the persistence service. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_data_from_persistence_service

> models::ItemHistoryDto get_item_data_from_persistence_service(itemname, service_id, starttime, endtime, page, pagelength, boundary, item_state)
Gets item persistence data from the persistence service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | The item name | [required] |
**service_id** | Option<**String**> | Id of the persistence service. If not provided the default service will be used |  |
**starttime** | Option<**String**> | Start time of the data to return. Will default to 1 day before endtime. [yyyy-MM-dd'T'HH:mm:ss.SSSZ] |  |
**endtime** | Option<**String**> | End time of the data to return. Will default to current time. [yyyy-MM-dd'T'HH:mm:ss.SSSZ] |  |
**page** | Option<**i32**> | Page number of data to return. This parameter will enable paging. |  |
**pagelength** | Option<**i32**> | The length of each page. |  |
**boundary** | Option<**bool**> | Gets one value before and after the requested period. |  |
**item_state** | Option<**bool**> | Adds the current Item state into the requested period (the item state will be before or at the endtime) |  |

### Return type

[**models::ItemHistoryDto**](ItemHistoryDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_items_for_persistence_service

> Vec<models::PersistenceItemInfo> get_items_for_persistence_service(service_id)
Gets a list of items available via a specific persistence service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | Option<**String**> | Id of the persistence service. If not provided the default service will be used |  |

### Return type

[**Vec<models::PersistenceItemInfo>**](PersistenceItemInfo.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_persistence_service_configuration

> models::PersistenceServiceConfigurationDto get_persistence_service_configuration(service_id)
Gets a persistence service configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Id of the persistence service. | [required] |

### Return type

[**models::PersistenceServiceConfigurationDto**](PersistenceServiceConfigurationDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_persistence_services

> Vec<models::PersistenceServiceDto> get_persistence_services(accept_language)
Gets a list of persistence services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::PersistenceServiceDto>**](PersistenceServiceDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_persistence_service_configuration

> models::PersistenceServiceConfigurationDto put_persistence_service_configuration(service_id, persistence_service_configuration_dto)
Sets a persistence service configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Id of the persistence service. | [required] |
**persistence_service_configuration_dto** | [**PersistenceServiceConfigurationDto**](PersistenceServiceConfigurationDto.md) | service configuration | [required] |

### Return type

[**models::PersistenceServiceConfigurationDto**](PersistenceServiceConfigurationDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_item_data_in_persistence_service

> store_item_data_in_persistence_service(itemname, time, state, service_id)
Stores item persistence data into the persistence service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**itemname** | **String** | The item name. | [required] |
**time** | **String** | Time of the data to be stored. Will default to current time. [yyyy-MM-dd'T'HH:mm:ss.SSSZ] | [required] |
**state** | **String** | The state to store. | [required] |
**service_id** | Option<**String**> | Id of the persistence service. If not provided the default service will be used |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

