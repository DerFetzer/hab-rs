# \AudioApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_audio_default_sink**](AudioApi.md#get_audio_default_sink) | **GET** /audio/defaultsink | Get the default sink if defined or the first available sink.
[**get_audio_default_source**](AudioApi.md#get_audio_default_source) | **GET** /audio/defaultsource | Get the default source if defined or the first available source.
[**get_audio_sinks**](AudioApi.md#get_audio_sinks) | **GET** /audio/sinks | Get the list of all sinks.
[**get_audio_sources**](AudioApi.md#get_audio_sources) | **GET** /audio/sources | Get the list of all sources.



## get_audio_default_sink

> models::AudioSinkDto get_audio_default_sink(accept_language)
Get the default sink if defined or the first available sink.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::AudioSinkDto**](AudioSinkDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_default_source

> models::AudioSourceDto get_audio_default_source(accept_language)
Get the default source if defined or the first available source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::AudioSourceDto**](AudioSourceDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_sinks

> Vec<models::AudioSinkDto> get_audio_sinks(accept_language)
Get the list of all sinks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::AudioSinkDto>**](AudioSinkDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_sources

> Vec<models::AudioSourceDto> get_audio_sources(accept_language)
Get the list of all sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::AudioSourceDto>**](AudioSourceDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

