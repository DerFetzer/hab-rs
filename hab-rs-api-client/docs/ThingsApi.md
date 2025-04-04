# \ThingsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_thing_in_registry**](ThingsApi.md#create_thing_in_registry) | **POST** /things | Creates a new thing and adds it to the registry.
[**enable_thing**](ThingsApi.md#enable_thing) | **PUT** /things/{thingUID}/enable | Sets the thing enabled status.
[**get_available_firmwares_for_thing**](ThingsApi.md#get_available_firmwares_for_thing) | **GET** /things/{thingUID}/firmwares | Get all available firmwares for provided thing UID
[**get_thing_by_id**](ThingsApi.md#get_thing_by_id) | **GET** /things/{thingUID} | Gets thing by UID.
[**get_thing_config_status**](ThingsApi.md#get_thing_config_status) | **GET** /things/{thingUID}/config/status | Gets thing config status.
[**get_thing_firmware_status**](ThingsApi.md#get_thing_firmware_status) | **GET** /things/{thingUID}/firmware/status | Gets thing's firmware status.
[**get_thing_status**](ThingsApi.md#get_thing_status) | **GET** /things/{thingUID}/status | Gets thing status.
[**get_things**](ThingsApi.md#get_things) | **GET** /things | Get all available things.
[**remove_thing_by_id**](ThingsApi.md#remove_thing_by_id) | **DELETE** /things/{thingUID} | Removes a thing from the registry. Set 'force' to __true__ if you want the thing to be removed immediately.
[**update_thing**](ThingsApi.md#update_thing) | **PUT** /things/{thingUID} | Updates a thing.
[**update_thing_config**](ThingsApi.md#update_thing_config) | **PUT** /things/{thingUID}/config | Updates thing's configuration.
[**update_thing_firmware**](ThingsApi.md#update_thing_firmware) | **PUT** /things/{thingUID}/firmware/{firmwareVersion} | Update thing firmware.



## create_thing_in_registry

> models::EnrichedThingDto create_thing_in_registry(thing_dto, accept_language)
Creates a new thing and adds it to the registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_dto** | [**ThingDto**](ThingDto.md) | thing data | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::EnrichedThingDto**](EnrichedThingDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_thing

> models::EnrichedThingDto enable_thing(thing_uid, accept_language, body)
Sets the thing enabled status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thing | [required] |
**accept_language** | Option<**String**> | language |  |
**body** | Option<**String**> | enabled |  |

### Return type

[**models::EnrichedThingDto**](EnrichedThingDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_firmwares_for_thing

> Vec<models::FirmwareDto> get_available_firmwares_for_thing(thing_uid, accept_language)
Get all available firmwares for provided thing UID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::FirmwareDto>**](FirmwareDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thing_by_id

> models::EnrichedThingDto get_thing_by_id(thing_uid, accept_language)
Gets thing by UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::EnrichedThingDto**](EnrichedThingDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thing_config_status

> Vec<models::ConfigStatusMessage> get_thing_config_status(thing_uid, accept_language)
Gets thing config status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thing | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::ConfigStatusMessage>**](ConfigStatusMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thing_firmware_status

> models::FirmwareStatusDto get_thing_firmware_status(thing_uid, accept_language)
Gets thing's firmware status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thing | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::FirmwareStatusDto**](FirmwareStatusDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thing_status

> models::ThingStatusInfo get_thing_status(thing_uid, accept_language)
Gets thing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thing | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::ThingStatusInfo**](ThingStatusInfo.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_things

> Vec<models::EnrichedThingDto> get_things(accept_language, summary, static_data_only)
Get all available things.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**summary** | Option<**bool**> | summary fields only |  |
**static_data_only** | Option<**bool**> | provides a cacheable list of values not expected to change regularly and checks the If-Modified-Since header |  |[default to false]

### Return type

[**Vec<models::EnrichedThingDto>**](EnrichedThingDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_thing_by_id

> remove_thing_by_id(thing_uid, accept_language, force)
Removes a thing from the registry. Set 'force' to __true__ if you want the thing to be removed immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |
**accept_language** | Option<**String**> | language |  |
**force** | Option<**bool**> | force |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_thing

> models::EnrichedThingDto update_thing(thing_uid, thing_dto, accept_language)
Updates a thing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thingUID | [required] |
**thing_dto** | [**ThingDto**](ThingDto.md) | thing | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::EnrichedThingDto**](EnrichedThingDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_thing_config

> models::EnrichedThingDto update_thing_config(thing_uid, accept_language, request_body)
Updates thing's configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thing | [required] |
**accept_language** | Option<**String**> | language |  |
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | configuration parameters |  |

### Return type

[**models::EnrichedThingDto**](EnrichedThingDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_thing_firmware

> update_thing_firmware(thing_uid, firmware_version, accept_language)
Update thing firmware.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thing_uid** | **String** | thing | [required] |
**firmware_version** | **String** | version | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

