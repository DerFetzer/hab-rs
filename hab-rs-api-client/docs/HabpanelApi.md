# \HabpanelApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gallery_widget_list**](HabpanelApi.md#get_gallery_widget_list) | **GET** /habpanel/gallery/{galleryName}/widgets | Gets the list of widget gallery items.
[**get_gallery_widgets_item**](HabpanelApi.md#get_gallery_widgets_item) | **GET** /habpanel/gallery/{galleryName}/widgets/{id} | Gets the details about a widget gallery item.



## get_gallery_widget_list

> Vec<models::GalleryWidgetsListItem> get_gallery_widget_list(gallery_name)
Gets the list of widget gallery items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gallery_name** | **String** | gallery name e.g. 'community' | [required] |

### Return type

[**Vec<models::GalleryWidgetsListItem>**](GalleryWidgetsListItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gallery_widgets_item

> models::GalleryItem get_gallery_widgets_item(gallery_name, id)
Gets the details about a widget gallery item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gallery_name** | **String** | gallery name e.g. 'community' | [required] |
**id** | **String** | id within the gallery | [required] |

### Return type

[**models::GalleryItem**](GalleryItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

