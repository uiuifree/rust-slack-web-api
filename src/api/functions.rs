// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`functions.completeError`](https://docs.slack.dev/reference/methods/functions.completeError): Signal that a function failed to complete
///
/// Send it with [`SlackClient::functions_complete_error`].
#[doc(alias = "functions.completeError")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FunctionsCompleteErrorRequest {
    /// Context identifier that maps to the executed function
    pub function_execution_id: String,
    /// A human-readable error message that contains information about why the function failed to complete
    pub error: String,
}

impl FunctionsCompleteErrorRequest {
    pub fn new(function_execution_id: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            function_execution_id: function_execution_id.into(),
            error: error.into(),
        }
    }
}

impl SlackApiMethod for FunctionsCompleteErrorRequest {
    const METHOD: &'static str = "functions.completeError";
    type Response = FunctionsCompleteErrorResponse;
}

/// Successful response of the Slack Web API method [`functions.completeError`](https://docs.slack.dev/reference/methods/functions.completeError).
#[doc(alias = "functions.completeError")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsCompleteErrorResponse {
    /// Fields returned by Slack. This method has no documented response example.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`functions.completeSuccess`](https://docs.slack.dev/reference/methods/functions.completeSuccess): Signal the successful completion of a function
///
/// Send it with [`SlackClient::functions_complete_success`].
#[doc(alias = "functions.completeSuccess")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FunctionsCompleteSuccessRequest {
    /// Context identifier that maps to the executed function
    pub function_execution_id: String,
    /// A JSON-based object that conforms to the [output parameters](https://docs.slack.dev/deno-slack-sdk/guides/creating-custom-functions.md#input-output) schema for the custom function defined in the manifest
    pub outputs: serde_json::Value,
}

impl FunctionsCompleteSuccessRequest {
    pub fn new(function_execution_id: impl Into<String>, outputs: serde_json::Value) -> Self {
        Self {
            function_execution_id: function_execution_id.into(),
            outputs,
        }
    }
}

impl SlackApiMethod for FunctionsCompleteSuccessRequest {
    const METHOD: &'static str = "functions.completeSuccess";
    type Response = FunctionsCompleteSuccessResponse;
}

/// Successful response of the Slack Web API method [`functions.completeSuccess`](https://docs.slack.dev/reference/methods/functions.completeSuccess).
#[doc(alias = "functions.completeSuccess")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsCompleteSuccessResponse {
    /// Fields returned by Slack. This method has no documented response example.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`functions.distributions.permissions.add`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.add): Grant users access to a custom slack function if its permission_type is set to named_entities
///
/// Send it with [`SlackClient::functions_distributions_permissions_add`].
#[doc(alias = "functions.distributions.permissions.add")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FunctionsDistributionsPermissionsAddRequest {
    /// The encoded ID of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    /// The callback ID defined in the function's definition file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_callback_id: Option<String>,
    /// The encoded ID of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_app_id: Option<String>,
    /// List of encoded user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl FunctionsDistributionsPermissionsAddRequest {
    pub fn new() -> Self {
        Self {
            function_id: None,
            function_callback_id: None,
            function_app_id: None,
            user_ids: None,
        }
    }

    pub fn function_id(mut self, function_id: impl Into<String>) -> Self {
        self.function_id = Some(function_id.into());
        self
    }

    pub fn function_callback_id(mut self, function_callback_id: impl Into<String>) -> Self {
        self.function_callback_id = Some(function_callback_id.into());
        self
    }

    pub fn function_app_id(mut self, function_app_id: impl Into<String>) -> Self {
        self.function_app_id = Some(function_app_id.into());
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}

impl SlackApiMethod for FunctionsDistributionsPermissionsAddRequest {
    const METHOD: &'static str = "functions.distributions.permissions.add";
    type Response = FunctionsDistributionsPermissionsAddResponse;
}

/// Successful response of the Slack Web API method [`functions.distributions.permissions.add`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.add).
#[doc(alias = "functions.distributions.permissions.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsAddResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<FunctionsDistributionsPermissionsAddResponseUsers>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsAddResponseUsers {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
}

/// Arguments for the Slack Web API method [`functions.distributions.permissions.list`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.list): List the access type of a custom slack function and include the users, team or org ids with access if its permission_type is set to named_entities
///
/// Send it with [`SlackClient::functions_distributions_permissions_list`].
#[doc(alias = "functions.distributions.permissions.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FunctionsDistributionsPermissionsListRequest {
    /// The encoded ID of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    /// The callback ID defined in the function's definition file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_callback_id: Option<String>,
    /// The encoded ID of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_app_id: Option<String>,
}

impl FunctionsDistributionsPermissionsListRequest {
    pub fn new() -> Self {
        Self {
            function_id: None,
            function_callback_id: None,
            function_app_id: None,
        }
    }

    pub fn function_id(mut self, function_id: impl Into<String>) -> Self {
        self.function_id = Some(function_id.into());
        self
    }

    pub fn function_callback_id(mut self, function_callback_id: impl Into<String>) -> Self {
        self.function_callback_id = Some(function_callback_id.into());
        self
    }

    pub fn function_app_id(mut self, function_app_id: impl Into<String>) -> Self {
        self.function_app_id = Some(function_app_id.into());
        self
    }
}

impl SlackApiMethod for FunctionsDistributionsPermissionsListRequest {
    const METHOD: &'static str = "functions.distributions.permissions.list";
    type Response = FunctionsDistributionsPermissionsListResponse;
}

/// Successful response of the Slack Web API method [`functions.distributions.permissions.list`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.list).
#[doc(alias = "functions.distributions.permissions.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<FunctionsDistributionsPermissionsListResponseUsers>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsListResponseUsers {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
}

/// Arguments for the Slack Web API method [`functions.distributions.permissions.remove`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.remove): Revoke user access to a custom slack function if permission_type set to named_entities
///
/// Send it with [`SlackClient::functions_distributions_permissions_remove`].
#[doc(alias = "functions.distributions.permissions.remove")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FunctionsDistributionsPermissionsRemoveRequest {
    /// The encoded ID of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    /// The callback ID defined in the function's definition file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_callback_id: Option<String>,
    /// The encoded ID of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_app_id: Option<String>,
    /// List of encoded user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl FunctionsDistributionsPermissionsRemoveRequest {
    pub fn new() -> Self {
        Self {
            function_id: None,
            function_callback_id: None,
            function_app_id: None,
            user_ids: None,
        }
    }

    pub fn function_id(mut self, function_id: impl Into<String>) -> Self {
        self.function_id = Some(function_id.into());
        self
    }

    pub fn function_callback_id(mut self, function_callback_id: impl Into<String>) -> Self {
        self.function_callback_id = Some(function_callback_id.into());
        self
    }

    pub fn function_app_id(mut self, function_app_id: impl Into<String>) -> Self {
        self.function_app_id = Some(function_app_id.into());
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}

impl SlackApiMethod for FunctionsDistributionsPermissionsRemoveRequest {
    const METHOD: &'static str = "functions.distributions.permissions.remove";
    type Response = FunctionsDistributionsPermissionsRemoveResponse;
}

/// Successful response of the Slack Web API method [`functions.distributions.permissions.remove`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.remove).
#[doc(alias = "functions.distributions.permissions.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsRemoveResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<FunctionsDistributionsPermissionsRemoveResponseUsers>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsRemoveResponseUsers {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
}

/// Arguments for the Slack Web API method [`functions.distributions.permissions.set`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.set): Set the access type of a custom slack function and define the users, team or org ids to be granted access if permission_type is set to named_entities
///
/// Send it with [`SlackClient::functions_distributions_permissions_set`].
#[doc(alias = "functions.distributions.permissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FunctionsDistributionsPermissionsSetRequest {
    /// The encoded ID of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    /// The callback ID defined in the function's definition file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_callback_id: Option<String>,
    /// The encoded ID of the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_app_id: Option<String>,
    /// The type of permission that defines how the function can be distributed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<String>,
    /// List of encoded user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// List of team IDs to allow for named\_entities permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// List of org IDs to allow for named\_entities permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_ids: Option<Vec<String>>,
}

impl FunctionsDistributionsPermissionsSetRequest {
    pub fn new() -> Self {
        Self {
            function_id: None,
            function_callback_id: None,
            function_app_id: None,
            permission_type: None,
            user_ids: None,
            team_ids: None,
            org_ids: None,
        }
    }

    pub fn function_id(mut self, function_id: impl Into<String>) -> Self {
        self.function_id = Some(function_id.into());
        self
    }

    pub fn function_callback_id(mut self, function_callback_id: impl Into<String>) -> Self {
        self.function_callback_id = Some(function_callback_id.into());
        self
    }

    pub fn function_app_id(mut self, function_app_id: impl Into<String>) -> Self {
        self.function_app_id = Some(function_app_id.into());
        self
    }

    pub fn permission_type(mut self, permission_type: impl Into<String>) -> Self {
        self.permission_type = Some(permission_type.into());
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn team_ids(mut self, team_ids: Vec<String>) -> Self {
        self.team_ids = Some(team_ids);
        self
    }

    pub fn org_ids(mut self, org_ids: Vec<String>) -> Self {
        self.org_ids = Some(org_ids);
        self
    }
}

impl SlackApiMethod for FunctionsDistributionsPermissionsSetRequest {
    const METHOD: &'static str = "functions.distributions.permissions.set";
    type Response = FunctionsDistributionsPermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`functions.distributions.permissions.set`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.set).
#[doc(alias = "functions.distributions.permissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsSetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<FunctionsDistributionsPermissionsSetResponseUsers>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsDistributionsPermissionsSetResponseUsers {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
}

/// Arguments for the Slack Web API method [`functions.workflows.steps.list`](https://docs.slack.dev/reference/methods/functions.workflows.steps.list): List the steps of a specific function of a workflow's versions
///
/// Send it with [`SlackClient::functions_workflows_steps_list`].
#[doc(alias = "functions.workflows.steps.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FunctionsWorkflowsStepsListRequest {
    /// The ID of the function to query
    pub function_id: String,
    /// The workflow ID, starts with Wf\*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// The workflow encoded ID or workflow reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
    /// The app tied to the workflow reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_app_id: Option<String>,
}

impl FunctionsWorkflowsStepsListRequest {
    pub fn new(function_id: impl Into<String>) -> Self {
        Self {
            function_id: function_id.into(),
            workflow_id: None,
            workflow: None,
            workflow_app_id: None,
        }
    }

    pub fn workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    pub fn workflow(mut self, workflow: impl Into<String>) -> Self {
        self.workflow = Some(workflow.into());
        self
    }

    pub fn workflow_app_id(mut self, workflow_app_id: impl Into<String>) -> Self {
        self.workflow_app_id = Some(workflow_app_id.into());
        self
    }
}

impl SlackApiMethod for FunctionsWorkflowsStepsListRequest {
    const METHOD: &'static str = "functions.workflows.steps.list";
    type Response = FunctionsWorkflowsStepsListResponse;
}

/// Successful response of the Slack Web API method [`functions.workflows.steps.list`](https://docs.slack.dev/reference/methods/functions.workflows.steps.list).
#[doc(alias = "functions.workflows.steps.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsWorkflowsStepsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub steps_versions: Vec<FunctionsWorkflowsStepsListResponseStepsVersions>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsWorkflowsStepsListResponseStepsVersions {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub step_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deleted: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_version_created: Option<String>,
}

/// Arguments for the Slack Web API method [`functions.workflows.steps.responses.export`](https://docs.slack.dev/reference/methods/functions.workflows.steps.responses.export): Download form responses of a workflow
///
/// Send it with [`SlackClient::functions_workflows_steps_responses_export`].
#[doc(alias = "functions.workflows.steps.responses.export")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FunctionsWorkflowsStepsResponsesExportRequest {
    /// The ID of the OpenForm step to export.
    pub step_id: String,
    /// The workflow ID, starts with Wf\*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// The workflow encoded ID or workflow reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
    /// The app tied to the workflow reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_app_id: Option<String>,
}

impl FunctionsWorkflowsStepsResponsesExportRequest {
    pub fn new(step_id: impl Into<String>) -> Self {
        Self {
            step_id: step_id.into(),
            workflow_id: None,
            workflow: None,
            workflow_app_id: None,
        }
    }

    pub fn workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    pub fn workflow(mut self, workflow: impl Into<String>) -> Self {
        self.workflow = Some(workflow.into());
        self
    }

    pub fn workflow_app_id(mut self, workflow_app_id: impl Into<String>) -> Self {
        self.workflow_app_id = Some(workflow_app_id.into());
        self
    }
}

impl SlackApiMethod for FunctionsWorkflowsStepsResponsesExportRequest {
    const METHOD: &'static str = "functions.workflows.steps.responses.export";
    type Response = FunctionsWorkflowsStepsResponsesExportResponse;
}

/// Successful response of the Slack Web API method [`functions.workflows.steps.responses.export`](https://docs.slack.dev/reference/methods/functions.workflows.steps.responses.export).
#[doc(alias = "functions.workflows.steps.responses.export")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FunctionsWorkflowsStepsResponsesExportResponse {
    /// Fields returned by Slack. This method has no documented response example.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`functions.completeError`](https://docs.slack.dev/reference/methods/functions.completeError): Signal that a function failed to complete
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.completeError")]
    pub async fn functions_complete_error(
        &self,
        request: &FunctionsCompleteErrorRequest,
    ) -> Result<FunctionsCompleteErrorResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`functions.completeSuccess`](https://docs.slack.dev/reference/methods/functions.completeSuccess): Signal the successful completion of a function
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.completeSuccess")]
    pub async fn functions_complete_success(
        &self,
        request: &FunctionsCompleteSuccessRequest,
    ) -> Result<FunctionsCompleteSuccessResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`functions.distributions.permissions.add`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.add): Grant users access to a custom slack function if its permission_type is set to named_entities
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.distributions.permissions.add")]
    pub async fn functions_distributions_permissions_add(
        &self,
        request: &FunctionsDistributionsPermissionsAddRequest,
    ) -> Result<FunctionsDistributionsPermissionsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`functions.distributions.permissions.list`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.list): List the access type of a custom slack function and include the users, team or org ids with access if its permission_type is set to named_entities
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.distributions.permissions.list")]
    pub async fn functions_distributions_permissions_list(
        &self,
        request: &FunctionsDistributionsPermissionsListRequest,
    ) -> Result<FunctionsDistributionsPermissionsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`functions.distributions.permissions.remove`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.remove): Revoke user access to a custom slack function if permission_type set to named_entities
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.distributions.permissions.remove")]
    pub async fn functions_distributions_permissions_remove(
        &self,
        request: &FunctionsDistributionsPermissionsRemoveRequest,
    ) -> Result<FunctionsDistributionsPermissionsRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`functions.distributions.permissions.set`](https://docs.slack.dev/reference/methods/functions.distributions.permissions.set): Set the access type of a custom slack function and define the users, team or org ids to be granted access if permission_type is set to named_entities
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.distributions.permissions.set")]
    pub async fn functions_distributions_permissions_set(
        &self,
        request: &FunctionsDistributionsPermissionsSetRequest,
    ) -> Result<FunctionsDistributionsPermissionsSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`functions.workflows.steps.list`](https://docs.slack.dev/reference/methods/functions.workflows.steps.list): List the steps of a specific function of a workflow's versions
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.workflows.steps.list")]
    pub async fn functions_workflows_steps_list(
        &self,
        request: &FunctionsWorkflowsStepsListRequest,
    ) -> Result<FunctionsWorkflowsStepsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`functions.workflows.steps.responses.export`](https://docs.slack.dev/reference/methods/functions.workflows.steps.responses.export): Download form responses of a workflow
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "functions.workflows.steps.responses.export")]
    pub async fn functions_workflows_steps_responses_export(
        &self,
        request: &FunctionsWorkflowsStepsResponsesExportRequest,
    ) -> Result<FunctionsWorkflowsStepsResponsesExportResponse, SlackError> {
        self.call(request).await
    }
}
