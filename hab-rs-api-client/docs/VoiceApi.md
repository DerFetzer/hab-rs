# \VoiceApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default_voice**](VoiceApi.md#get_default_voice) | **GET** /voice/defaultvoice | Gets the default voice.
[**get_voice_interpreter_by_uid**](VoiceApi.md#get_voice_interpreter_by_uid) | **GET** /voice/interpreters/{id} | Gets a single interpreter.
[**get_voice_interpreters**](VoiceApi.md#get_voice_interpreters) | **GET** /voice/interpreters | Get the list of all interpreters.
[**get_voices**](VoiceApi.md#get_voices) | **GET** /voice/voices | Get the list of all voices.
[**interpret_text**](VoiceApi.md#interpret_text) | **POST** /voice/interpreters/{ids} | Sends a text to a given human language interpreter(s).
[**interpret_text_by_default_interpreter**](VoiceApi.md#interpret_text_by_default_interpreter) | **POST** /voice/interpreters | Sends a text to the default human language interpreter.
[**listen_and_answer**](VoiceApi.md#listen_and_answer) | **POST** /voice/listenandanswer | Executes a simple dialog sequence without keyword spotting for a given audio source.
[**start_dialog**](VoiceApi.md#start_dialog) | **POST** /voice/dialog/start | Start dialog processing for a given audio source.
[**stop_dialog**](VoiceApi.md#stop_dialog) | **POST** /voice/dialog/stop | Stop dialog processing for a given audio source.
[**text_to_speech**](VoiceApi.md#text_to_speech) | **POST** /voice/say | Speaks a given text with a given voice through the given audio sink.



## get_default_voice

> models::VoiceDto get_default_voice()
Gets the default voice.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::VoiceDto**](VoiceDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_interpreter_by_uid

> Vec<models::HumanLanguageInterpreterDto> get_voice_interpreter_by_uid(id, accept_language)
Gets a single interpreter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | interpreter id | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::HumanLanguageInterpreterDto>**](HumanLanguageInterpreterDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_interpreters

> Vec<models::HumanLanguageInterpreterDto> get_voice_interpreters(accept_language)
Get the list of all interpreters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::HumanLanguageInterpreterDto>**](HumanLanguageInterpreterDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voices

> Vec<models::VoiceDto> get_voices()
Get the list of all voices.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::VoiceDto>**](VoiceDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interpret_text

> String interpret_text(ids, body, accept_language)
Sends a text to a given human language interpreter(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | comma separated list of interpreter ids | [required] |
**body** | **String** | text to interpret | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interpret_text_by_default_interpreter

> String interpret_text_by_default_interpreter(body, accept_language)
Sends a text to the default human language interpreter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | text to interpret | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## listen_and_answer

> listen_and_answer(accept_language, source_id, stt_id, tts_id, voice_id, hli_ids, sink_id, listening_item)
Executes a simple dialog sequence without keyword spotting for a given audio source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**source_id** | Option<**String**> | source ID |  |
**stt_id** | Option<**String**> | Speech-to-Text ID |  |
**tts_id** | Option<**String**> | Text-to-Speech ID |  |
**voice_id** | Option<**String**> | voice ID |  |
**hli_ids** | Option<[**Vec<String>**](String.md)> | interpreter IDs |  |
**sink_id** | Option<**String**> | audio sink ID |  |
**listening_item** | Option<**String**> | listening item |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dialog

> start_dialog(accept_language, source_id, ks_id, stt_id, tts_id, voice_id, hli_ids, sink_id, keyword, listening_item)
Start dialog processing for a given audio source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |
**source_id** | Option<**String**> | source ID |  |
**ks_id** | Option<**String**> | keywork spotter ID |  |
**stt_id** | Option<**String**> | Speech-to-Text ID |  |
**tts_id** | Option<**String**> | Text-to-Speech ID |  |
**voice_id** | Option<**String**> | voice ID |  |
**hli_ids** | Option<**String**> | comma separated list of interpreter IDs |  |
**sink_id** | Option<**String**> | audio sink ID |  |
**keyword** | Option<**String**> | keyword |  |
**listening_item** | Option<**String**> | listening item |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dialog

> stop_dialog(source_id)
Stop dialog processing for a given audio source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_id** | Option<**String**> | source ID |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## text_to_speech

> text_to_speech(body, voiceid, sinkid, volume)
Speaks a given text with a given voice through the given audio sink.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | text to speak | [required] |
**voiceid** | Option<**String**> | voice id |  |
**sinkid** | Option<**String**> | audio sink id |  |
**volume** | Option<**String**> | volume level |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

