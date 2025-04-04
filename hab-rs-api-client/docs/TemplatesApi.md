# \TemplatesApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_template_by_id**](TemplatesApi.md#get_template_by_id) | **GET** /templates/{templateUID} | Gets a template corresponding to the given UID.
[**get_templates**](TemplatesApi.md#get_templates) | **GET** /templates | Get all available templates.



## get_template_by_id

> models::Template get_template_by_id(template_uid, accept_language)
Gets a template corresponding to the given UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_uid** | **String** | templateUID | [required] |
**accept_language** | Option<**String**> | language |  |

### Return type

[**models::Template**](Template.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_templates

> Vec<models::Template> get_templates(accept_language)
Get all available templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | language |  |

### Return type

[**Vec<models::Template>**](Template.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

