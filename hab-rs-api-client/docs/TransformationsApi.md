# \TransformationsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_transformation**](TransformationsApi.md#delete_transformation) | **DELETE** /transformations/{uid} | Get a single transformation
[**get_transformation**](TransformationsApi.md#get_transformation) | **GET** /transformations/{uid} | Get a single transformation
[**get_transformation_services**](TransformationsApi.md#get_transformation_services) | **GET** /transformations/services | Get all transformation services
[**get_transformations**](TransformationsApi.md#get_transformations) | **GET** /transformations | Get a list of all transformations
[**put_transformation**](TransformationsApi.md#put_transformation) | **PUT** /transformations/{uid} | Put a single transformation



## delete_transformation

> delete_transformation(uid)
Get a single transformation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Transformation UID | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transformation

> models::Transformation get_transformation(uid)
Get a single transformation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Transformation UID | [required] |

### Return type

[**models::Transformation**](Transformation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transformation_services

> Vec<String> get_transformation_services()
Get all transformation services

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transformations

> Vec<models::TransformationDto> get_transformations()
Get a list of all transformations

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TransformationDto>**](TransformationDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_transformation

> put_transformation(uid, transformation_dto)
Put a single transformation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Transformation UID | [required] |
**transformation_dto** | [**TransformationDto**](TransformationDto.md) | transformation | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

