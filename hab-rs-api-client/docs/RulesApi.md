# \RulesApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rule**](RulesApi.md#create_rule) | **POST** /rules | Creates a rule.
[**delete_rule**](RulesApi.md#delete_rule) | **DELETE** /rules/{ruleUID} | Removes an existing rule corresponding to the given UID.
[**enable_rule**](RulesApi.md#enable_rule) | **POST** /rules/{ruleUID}/enable | Sets the rule enabled status.
[**get_rule_actions**](RulesApi.md#get_rule_actions) | **GET** /rules/{ruleUID}/actions | Gets the rule actions.
[**get_rule_by_id**](RulesApi.md#get_rule_by_id) | **GET** /rules/{ruleUID} | Gets the rule corresponding to the given UID.
[**get_rule_conditions**](RulesApi.md#get_rule_conditions) | **GET** /rules/{ruleUID}/conditions | Gets the rule conditions.
[**get_rule_configuration**](RulesApi.md#get_rule_configuration) | **GET** /rules/{ruleUID}/config | Gets the rule configuration values.
[**get_rule_module_by_id**](RulesApi.md#get_rule_module_by_id) | **GET** /rules/{ruleUID}/{moduleCategory}/{id} | Gets the rule's module corresponding to the given Category and ID.
[**get_rule_module_config**](RulesApi.md#get_rule_module_config) | **GET** /rules/{ruleUID}/{moduleCategory}/{id}/config | Gets the module's configuration.
[**get_rule_module_config_parameter**](RulesApi.md#get_rule_module_config_parameter) | **GET** /rules/{ruleUID}/{moduleCategory}/{id}/config/{param} | Gets the module's configuration parameter.
[**get_rule_triggers**](RulesApi.md#get_rule_triggers) | **GET** /rules/{ruleUID}/triggers | Gets the rule triggers.
[**get_rules**](RulesApi.md#get_rules) | **GET** /rules | Get available rules, optionally filtered by tags and/or prefix.
[**get_schedule_rule_simulations**](RulesApi.md#get_schedule_rule_simulations) | **GET** /rules/schedule/simulations | Simulates the executions of rules filtered by tag 'Schedule' within the given times.
[**run_rule_now1**](RulesApi.md#run_rule_now1) | **POST** /rules/{ruleUID}/runnow | Executes actions of the rule.
[**set_rule_module_config_parameter**](RulesApi.md#set_rule_module_config_parameter) | **PUT** /rules/{ruleUID}/{moduleCategory}/{id}/config/{param} | Sets the module's configuration parameter value.
[**update_rule**](RulesApi.md#update_rule) | **PUT** /rules/{ruleUID} | Updates an existing rule corresponding to the given UID.
[**update_rule_configuration**](RulesApi.md#update_rule_configuration) | **PUT** /rules/{ruleUID}/config | Sets the rule configuration values.



## create_rule

> create_rule(rule_dto)
Creates a rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_dto** | [**RuleDto**](RuleDto.md) | rule data | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rule

> delete_rule(rule_uid)
Removes an existing rule corresponding to the given UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_rule

> enable_rule(rule_uid, body)
Sets the rule enabled status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**body** | **String** | enable | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_actions

> Vec<models::ActionDto> get_rule_actions(rule_uid)
Gets the rule actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |

### Return type

[**Vec<models::ActionDto>**](ActionDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_by_id

> models::EnrichedRuleDto get_rule_by_id(rule_uid)
Gets the rule corresponding to the given UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |

### Return type

[**models::EnrichedRuleDto**](EnrichedRuleDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_conditions

> Vec<models::ConditionDto> get_rule_conditions(rule_uid)
Gets the rule conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |

### Return type

[**Vec<models::ConditionDto>**](ConditionDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_configuration

> String get_rule_configuration(rule_uid)
Gets the rule configuration values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_module_by_id

> models::ModuleDto get_rule_module_by_id(rule_uid, module_category, id)
Gets the rule's module corresponding to the given Category and ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**module_category** | **String** | moduleCategory | [required] |
**id** | **String** | id | [required] |

### Return type

[**models::ModuleDto**](ModuleDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_module_config

> String get_rule_module_config(rule_uid, module_category, id)
Gets the module's configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**module_category** | **String** | moduleCategory | [required] |
**id** | **String** | id | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_module_config_parameter

> String get_rule_module_config_parameter(rule_uid, module_category, id, param)
Gets the module's configuration parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**module_category** | **String** | moduleCategory | [required] |
**id** | **String** | id | [required] |
**param** | **String** | param | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_triggers

> Vec<models::TriggerDto> get_rule_triggers(rule_uid)
Gets the rule triggers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |

### Return type

[**Vec<models::TriggerDto>**](TriggerDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rules

> Vec<models::EnrichedRuleDto> get_rules(prefix, tags, summary, static_data_only)
Get available rules, optionally filtered by tags and/or prefix.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | Option<**String**> |  |  |
**tags** | Option<[**Vec<String>**](String.md)> |  |  |
**summary** | Option<**bool**> | summary fields only |  |
**static_data_only** | Option<**bool**> | provides a cacheable list of values not expected to change regularly and honors the If-Modified-Since header, all other parameters are ignored |  |[default to false]

### Return type

[**Vec<models::EnrichedRuleDto>**](EnrichedRuleDTO.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule_rule_simulations

> Vec<models::RuleExecution> get_schedule_rule_simulations(from, until)
Simulates the executions of rules filtered by tag 'Schedule' within the given times.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | Start time of the simulated rule executions. Will default to the current time. [yyyy-MM-dd'T'HH:mm:ss.SSSZ] |  |
**until** | Option<**String**> | End time of the simulated rule executions. Will default to 30 days after the start time. Must be less than 180 days after the given start time. [yyyy-MM-dd'T'HH:mm:ss.SSSZ] |  |

### Return type

[**Vec<models::RuleExecution>**](RuleExecution.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_rule_now1

> run_rule_now1(rule_uid, request_body)
Executes actions of the rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | the context for running this rule |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_rule_module_config_parameter

> set_rule_module_config_parameter(rule_uid, module_category, id, param, body)
Sets the module's configuration parameter value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**module_category** | **String** | moduleCategory | [required] |
**id** | **String** | id | [required] |
**param** | **String** | param | [required] |
**body** | **String** | value | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rule

> update_rule(rule_uid, rule_dto)
Updates an existing rule corresponding to the given UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**rule_dto** | [**RuleDto**](RuleDto.md) | rule data | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rule_configuration

> update_rule_configuration(rule_uid, request_body)
Sets the rule configuration values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uid** | **String** | ruleUID | [required] |
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | config |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

