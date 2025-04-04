# \SysteminfoApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_information**](SysteminfoApi.md#get_system_information) | **GET** /systeminfo | Gets information about the system.
[**get_uo_m_information**](SysteminfoApi.md#get_uo_m_information) | **GET** /systeminfo/uom | Get all supported dimensions and their system units.



## get_system_information

> models::SystemInfoBean get_system_information()
Gets information about the system.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemInfoBean**](SystemInfoBean.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_uo_m_information

> models::UoMInfoBean get_uo_m_information()
Get all supported dimensions and their system units.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UoMInfoBean**](UoMInfoBean.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

