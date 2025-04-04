# \SitemapsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sitemap_event_subscription**](SitemapsApi.md#create_sitemap_event_subscription) | **POST** /sitemaps/events/subscribe | Creates a sitemap event subscription.
[**get_sitemap_by_name**](SitemapsApi.md#get_sitemap_by_name) | **GET** /sitemaps/{sitemapname} | Get sitemap by name.
[**get_sitemap_events**](SitemapsApi.md#get_sitemap_events) | **GET** /sitemaps/events/{subscriptionid}/* | Get sitemap events for a whole sitemap. Not recommended due to potentially high traffic.
[**get_sitemap_events1**](SitemapsApi.md#get_sitemap_events1) | **GET** /sitemaps/events/{subscriptionid} | Get sitemap events.
[**get_sitemaps**](SitemapsApi.md#get_sitemaps) | **GET** /sitemaps | Get all available sitemaps.
[**poll_data_for_page**](SitemapsApi.md#poll_data_for_page) | **GET** /sitemaps/{sitemapname}/{pageid} | Polls the data for one page of a sitemap.
[**poll_data_for_sitemap**](SitemapsApi.md#poll_data_for_sitemap) | **GET** /sitemaps/{sitemapname}/* | Polls the data for a whole sitemap. Not recommended due to potentially high traffic.



## create_sitemap_event_subscription

> create_sitemap_event_subscription()
Creates a sitemap event subscription.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sitemap_by_name

> models::SitemapDto get_sitemap_by_name(sitemapname, accept_language, r#type, jsoncallback, include_hidden)
Get sitemap by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sitemapname** | **String** | sitemap name | [required] |
**accept_language** | Option<**String**> | language |  |
**r#type** | Option<**String**> |  |  |
**jsoncallback** | Option<**String**> |  |  |[default to callback]
**include_hidden** | Option<**bool**> | include hidden widgets |  |

### Return type

[**models::SitemapDto**](SitemapDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sitemap_events

> get_sitemap_events(subscriptionid, sitemap)
Get sitemap events for a whole sitemap. Not recommended due to potentially high traffic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscriptionid** | **String** | subscription id | [required] |
**sitemap** | Option<**String**> | sitemap name |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sitemap_events1

> get_sitemap_events1(subscriptionid, sitemap, pageid)
Get sitemap events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscriptionid** | **String** | subscription id | [required] |
**sitemap** | Option<**String**> | sitemap name |  |
**pageid** | Option<**String**> | page id |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sitemaps

> Vec<models::SitemapDto> get_sitemaps()
Get all available sitemaps.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SitemapDto>**](SitemapDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## poll_data_for_page

> models::PageDto poll_data_for_page(sitemapname, pageid, accept_language, subscriptionid, include_hidden)
Polls the data for one page of a sitemap.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sitemapname** | **String** | sitemap name | [required] |
**pageid** | **String** | page id | [required] |
**accept_language** | Option<**String**> | language |  |
**subscriptionid** | Option<**String**> | subscriptionid |  |
**include_hidden** | Option<**bool**> | include hidden widgets |  |

### Return type

[**models::PageDto**](PageDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## poll_data_for_sitemap

> models::SitemapDto poll_data_for_sitemap(sitemapname, accept_language, subscriptionid, include_hidden)
Polls the data for a whole sitemap. Not recommended due to potentially high traffic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sitemapname** | **String** | sitemap name | [required] |
**accept_language** | Option<**String**> | language |  |
**subscriptionid** | Option<**String**> | subscriptionid |  |
**include_hidden** | Option<**bool**> | include hidden widgets |  |

### Return type

[**models::SitemapDto**](SitemapDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

