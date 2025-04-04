# \LoggingApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logger**](LoggingApi.md#get_logger) | **GET** /logging/{loggerName} | Get a single logger.
[**get_logger1**](LoggingApi.md#get_logger1) | **GET** /logging | Get all loggers
[**put_logger**](LoggingApi.md#put_logger) | **PUT** /logging/{loggerName} | Modify or add logger
[**remove_logger**](LoggingApi.md#remove_logger) | **DELETE** /logging/{loggerName} | Remove a single logger.



## get_logger

> models::LoggerInfo get_logger(logger_name)
Get a single logger.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logger_name** | **String** | logger name | [required] |

### Return type

[**models::LoggerInfo**](LoggerInfo.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logger1

> models::LoggerBean get_logger1()
Get all loggers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LoggerBean**](LoggerBean.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_logger

> put_logger(logger_name, logger_info)
Modify or add logger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logger_name** | **String** | logger name | [required] |
**logger_info** | [**LoggerInfo**](LoggerInfo.md) | logger | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_logger

> remove_logger(logger_name)
Remove a single logger.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logger_name** | **String** | logger name | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

