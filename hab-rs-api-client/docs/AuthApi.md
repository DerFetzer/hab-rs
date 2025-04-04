# \AuthApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_session**](AuthApi.md#delete_session) | **POST** /auth/logout | Delete the session associated with a refresh token.
[**get_api_tokens**](AuthApi.md#get_api_tokens) | **GET** /auth/apitokens | List the API tokens associated to the authenticated user.
[**get_o_auth_token**](AuthApi.md#get_o_auth_token) | **POST** /auth/token | Get access and refresh tokens.
[**get_sessions_for_current_user**](AuthApi.md#get_sessions_for_current_user) | **GET** /auth/sessions | List the sessions associated to the authenticated user.
[**remove_api_token**](AuthApi.md#remove_api_token) | **DELETE** /auth/apitokens/{name} | Revoke a specified API token associated to the authenticated user.



## delete_session

> delete_session(id, refresh_token)
Delete the session associated with a refresh token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**refresh_token** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_tokens

> Vec<models::UserApiTokenDto> get_api_tokens()
List the API tokens associated to the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserApiTokenDto>**](UserApiTokenDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_o_auth_token

> models::TokenResponseDto get_o_auth_token(use_cookie, client_id, code, code_verifier, grant_type, redirect_uri, refresh_token)
Get access and refresh tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**use_cookie** | Option<**bool**> |  |  |
**client_id** | Option<**String**> |  |  |
**code** | Option<**String**> |  |  |
**code_verifier** | Option<**String**> |  |  |
**grant_type** | Option<**String**> |  |  |
**redirect_uri** | Option<**String**> |  |  |
**refresh_token** | Option<**String**> |  |  |

### Return type

[**models::TokenResponseDto**](TokenResponseDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sessions_for_current_user

> Vec<models::UserSessionDto> get_sessions_for_current_user()
List the sessions associated to the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserSessionDto>**](UserSessionDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_api_token

> remove_api_token(name)
Revoke a specified API token associated to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

