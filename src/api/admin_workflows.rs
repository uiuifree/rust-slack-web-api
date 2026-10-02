// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.workflows.collaborators.add`](https://docs.slack.dev/reference/methods/admin.workflows.collaborators.add): Add collaborators to workflows within the team or enterprise
///
/// Send it with [`SlackClient::admin_workflows_collaborators_add`].
#[doc(alias = "admin.workflows.collaborators.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminWorkflowsCollaboratorsAddRequest {
    /// Array of workflow IDs to edit; max 50
    pub workflow_ids: Vec<String>,
    /// Array of collaborators (encoded user IDs) to add; max 50
    pub collaborator_ids: Vec<String>,
}

impl AdminWorkflowsCollaboratorsAddRequest {
    pub fn new(workflow_ids: Vec<String>, collaborator_ids: Vec<String>) -> Self {
        Self {
            workflow_ids,
            collaborator_ids,
        }
    }
}

impl SlackApiMethod for AdminWorkflowsCollaboratorsAddRequest {
    const METHOD: &'static str = "admin.workflows.collaborators.add";
    type Response = AdminWorkflowsCollaboratorsAddResponse;
}

/// Successful response of the Slack Web API method [`admin.workflows.collaborators.add`](https://docs.slack.dev/reference/methods/admin.workflows.collaborators.add).
#[doc(alias = "admin.workflows.collaborators.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsCollaboratorsAddResponse {
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

/// Arguments for the Slack Web API method [`admin.workflows.collaborators.remove`](https://docs.slack.dev/reference/methods/admin.workflows.collaborators.remove): Remove collaborators from workflows within the team or enterprise
///
/// Send it with [`SlackClient::admin_workflows_collaborators_remove`].
#[doc(alias = "admin.workflows.collaborators.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminWorkflowsCollaboratorsRemoveRequest {
    /// Array of workflow IDs to edit; max 50
    pub workflow_ids: Vec<String>,
    /// Array of collaborators (encoded user IDs) to remove; max 50
    pub collaborator_ids: Vec<String>,
}

impl AdminWorkflowsCollaboratorsRemoveRequest {
    pub fn new(workflow_ids: Vec<String>, collaborator_ids: Vec<String>) -> Self {
        Self {
            workflow_ids,
            collaborator_ids,
        }
    }
}

impl SlackApiMethod for AdminWorkflowsCollaboratorsRemoveRequest {
    const METHOD: &'static str = "admin.workflows.collaborators.remove";
    type Response = AdminWorkflowsCollaboratorsRemoveResponse;
}

/// Successful response of the Slack Web API method [`admin.workflows.collaborators.remove`](https://docs.slack.dev/reference/methods/admin.workflows.collaborators.remove).
#[doc(alias = "admin.workflows.collaborators.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsCollaboratorsRemoveResponse {
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

/// Arguments for the Slack Web API method [`admin.workflows.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.workflows.permissions.lookup): Look up the permissions for a set of workflows
///
/// Send it with [`SlackClient::admin_workflows_permissions_lookup`].
#[doc(alias = "admin.workflows.permissions.lookup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminWorkflowsPermissionsLookupRequest {
    /// An array of workflow IDs to look up permissions for
    pub workflow_ids: Vec<String>,
    /// Maximum number of triggers to fetch for each workflow when determining overall run permissions; max 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_workflow_triggers: Option<i64>,
}

impl AdminWorkflowsPermissionsLookupRequest {
    pub fn new(workflow_ids: Vec<String>) -> Self {
        Self {
            workflow_ids,
            max_workflow_triggers: None,
        }
    }

    pub fn max_workflow_triggers(mut self, max_workflow_triggers: i64) -> Self {
        self.max_workflow_triggers = Some(max_workflow_triggers);
        self
    }
}

impl SlackApiMethod for AdminWorkflowsPermissionsLookupRequest {
    const METHOD: &'static str = "admin.workflows.permissions.lookup";
    type Response = AdminWorkflowsPermissionsLookupResponse;
}

/// Successful response of the Slack Web API method [`admin.workflows.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.workflows.permissions.lookup).
#[doc(alias = "admin.workflows.permissions.lookup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsPermissionsLookupResponse {
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

/// Arguments for the Slack Web API method [`admin.workflows.search`](https://docs.slack.dev/reference/methods/admin.workflows.search): Search workflows within the team or enterprise
///
/// Send it with [`SlackClient::admin_workflows_search`].
#[doc(alias = "admin.workflows.search")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminWorkflowsSearchRequest {
    /// A search query to filter for workflow name or description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The parent app ID for which to return workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of results that will be returned by the API on each invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only include workflows with no collaborators in the result; default is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_collaborators: Option<bool>,
    /// Only include workflows where all of the provided user IDs are a manager/collaborator of that workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_ids: Option<Vec<String>>,
    /// Number of trigger IDs to fetch for each workflow; default is 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_trigger_ids: Option<i64>,
    /// Filter workflows by their Sales Elevate status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sales_elevate: Option<bool>,
    /// Source of workflow creation, either from code or workflow builder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The field used to sort the returned workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Sort direction. Possible values are `asc` for ascending order like (1, 2, 3) or (a, b, c), and `desc` for descending order like (3, 2, 1) or (c, b, a)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<String>,
    /// Only include workflows with this trigger type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type_id: Option<String>,
    /// Filter workflows by their published status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
    /// Only include workflows that use all of the provided step function ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_function_ids: Option<Vec<String>>,
}

impl AdminWorkflowsSearchRequest {
    pub fn new() -> Self {
        Self {
            query: None,
            app_id: None,
            cursor: None,
            limit: None,
            no_collaborators: None,
            collaborator_ids: None,
            num_trigger_ids: None,
            is_sales_elevate: None,
            source: None,
            sort: None,
            sort_dir: None,
            trigger_type_id: None,
            publish_status: None,
            step_function_ids: None,
        }
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn no_collaborators(mut self, no_collaborators: bool) -> Self {
        self.no_collaborators = Some(no_collaborators);
        self
    }

    pub fn collaborator_ids(mut self, collaborator_ids: Vec<String>) -> Self {
        self.collaborator_ids = Some(collaborator_ids);
        self
    }

    pub fn num_trigger_ids(mut self, num_trigger_ids: i64) -> Self {
        self.num_trigger_ids = Some(num_trigger_ids);
        self
    }

    pub fn is_sales_elevate(mut self, is_sales_elevate: bool) -> Self {
        self.is_sales_elevate = Some(is_sales_elevate);
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn sort_dir(mut self, sort_dir: impl Into<String>) -> Self {
        self.sort_dir = Some(sort_dir.into());
        self
    }

    pub fn trigger_type_id(mut self, trigger_type_id: impl Into<String>) -> Self {
        self.trigger_type_id = Some(trigger_type_id.into());
        self
    }

    pub fn publish_status(mut self, publish_status: impl Into<String>) -> Self {
        self.publish_status = Some(publish_status.into());
        self
    }

    pub fn step_function_ids(mut self, step_function_ids: Vec<String>) -> Self {
        self.step_function_ids = Some(step_function_ids);
        self
    }
}

impl SlackApiMethod for AdminWorkflowsSearchRequest {
    const METHOD: &'static str = "admin.workflows.search";
    type Response = AdminWorkflowsSearchResponse;
}

impl CursorPaginated for AdminWorkflowsSearchRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminWorkflowsSearchResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.workflows.search`](https://docs.slack.dev/reference/methods/admin.workflows.search).
#[doc(alias = "admin.workflows.search")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_found: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub workflows: Vec<AdminWorkflowsSearchResponseWorkflows>,
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
pub struct AdminWorkflowsSearchResponseWorkflows {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_function_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_id: Option<String>,
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
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub input_parameters: Option<AdminWorkflowsSearchResponseWorkflowsInputParameters>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub steps: Vec<AdminWorkflowsSearchResponseWorkflowsSteps>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub collaborators: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub icons: Option<Icons>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_published: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub unpublished_change_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_billable: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_source_type: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_source_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_published_version_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_published_date: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trigger_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sales_home_workflow: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sales_elevate: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trigger_types: Vec<AdminWorkflowsSearchResponseWorkflowsTriggerTypes>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsInputParameters {
    #[serde(
        rename = "Ft014FQ980RZ__user_id",
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub ft014_fq980_rz_user_id:
        Option<AdminWorkflowsSearchResponseWorkflowsInputParametersFt014Fq980RzUserId>,
    #[serde(
        rename = "Ft014FQ980RZ__message_context",
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub ft014_fq980_rz_message_context:
        Option<AdminWorkflowsSearchResponseWorkflowsInputParametersFt014Fq980RzMessageContext>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub interactivity: Option<AdminWorkflowsSearchResponseWorkflowsInputParametersInteractivity>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsInputParametersFt014Fq980RzUserId {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsInputParametersFt014Fq980RzMessageContext {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsInputParametersInteractivity {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsSteps {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub inputs: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputs>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputs {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsMessage>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_context: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsMessageContext>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_broadcast: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsReplyBroadcast>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsTitle>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub fields: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsFields>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsDescription>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub submit_label: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsSubmitLabel>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub interactivity: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsInteractivity>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub vibe: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsVibe>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsChannelId>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsMessageContext {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsReplyBroadcast {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsTitle {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsFields {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<AdminWorkflowsSearchResponseWorkflowsStepsInputsFieldsValue>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsFieldsValue {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub elements: Vec<AdminWorkflowsSearchResponseWorkflowsStepsInputsFieldsValueElements>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsFieldsValueElements {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
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
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub long: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub r#enum: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsDescription {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsSubmitLabel {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsInteractivity {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsVibe {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsStepsInputsChannelId {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsSearchResponseWorkflowsTriggerTypes {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtype: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.workflows.triggers.types.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.workflows.triggers.types.permissions.lookup): List the permissions for using each trigger type
///
/// Send it with [`SlackClient::admin_workflows_triggers_types_permissions_lookup`].
#[doc(alias = "admin.workflows.triggers.types.permissions.lookup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminWorkflowsTriggersTypesPermissionsLookupRequest {
    /// The trigger type IDs for which to get the permissions.
    #[serde(serialize_with = "crate::form::as_json")]
    pub trigger_type_ids: Vec<String>,
}

impl AdminWorkflowsTriggersTypesPermissionsLookupRequest {
    pub fn new(trigger_type_ids: Vec<String>) -> Self {
        Self { trigger_type_ids }
    }
}

impl SlackApiMethod for AdminWorkflowsTriggersTypesPermissionsLookupRequest {
    const METHOD: &'static str = "admin.workflows.triggers.types.permissions.lookup";
    type Response = AdminWorkflowsTriggersTypesPermissionsLookupResponse;
}

/// Successful response of the Slack Web API method [`admin.workflows.triggers.types.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.workflows.triggers.types.permissions.lookup).
#[doc(alias = "admin.workflows.triggers.types.permissions.lookup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsTriggersTypesPermissionsLookupResponse {
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

/// Arguments for the Slack Web API method [`admin.workflows.triggers.types.permissions.set`](https://docs.slack.dev/reference/methods/admin.workflows.triggers.types.permissions.set): Set the permissions for using a trigger type
///
/// Send it with [`SlackClient::admin_workflows_triggers_types_permissions_set`].
#[doc(alias = "admin.workflows.triggers.types.permissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminWorkflowsTriggersTypesPermissionsSetRequest {
    /// The trigger type ID for which to set the permissions
    pub id: String,
    /// The function visibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// List of user IDs to allow for named\_entities visibility
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<serde_json::Value>,
}

impl AdminWorkflowsTriggersTypesPermissionsSetRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            visibility: None,
            user_ids: None,
            permissions: None,
        }
    }

    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn permissions(mut self, permissions: serde_json::Value) -> Self {
        self.permissions = Some(permissions);
        self
    }
}

impl SlackApiMethod for AdminWorkflowsTriggersTypesPermissionsSetRequest {
    const METHOD: &'static str = "admin.workflows.triggers.types.permissions.set";
    type Response = AdminWorkflowsTriggersTypesPermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`admin.workflows.triggers.types.permissions.set`](https://docs.slack.dev/reference/methods/admin.workflows.triggers.types.permissions.set).
#[doc(alias = "admin.workflows.triggers.types.permissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsTriggersTypesPermissionsSetResponse {
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

/// Arguments for the Slack Web API method [`admin.workflows.unpublish`](https://docs.slack.dev/reference/methods/admin.workflows.unpublish): Unpublish workflows within the team or enterprise
///
/// Send it with [`SlackClient::admin_workflows_unpublish`].
#[doc(alias = "admin.workflows.unpublish")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminWorkflowsUnpublishRequest {
    /// Array of workflow IDs to unpublish
    pub workflow_ids: Vec<String>,
}

impl AdminWorkflowsUnpublishRequest {
    pub fn new(workflow_ids: Vec<String>) -> Self {
        Self { workflow_ids }
    }
}

impl SlackApiMethod for AdminWorkflowsUnpublishRequest {
    const METHOD: &'static str = "admin.workflows.unpublish";
    type Response = AdminWorkflowsUnpublishResponse;
}

/// Successful response of the Slack Web API method [`admin.workflows.unpublish`](https://docs.slack.dev/reference/methods/admin.workflows.unpublish).
#[doc(alias = "admin.workflows.unpublish")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminWorkflowsUnpublishResponse {
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
    /// Calls the Slack Web API method [`admin.workflows.collaborators.add`](https://docs.slack.dev/reference/methods/admin.workflows.collaborators.add): Add collaborators to workflows within the team or enterprise
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.workflows.collaborators.add")]
    pub async fn admin_workflows_collaborators_add(
        &self,
        request: &AdminWorkflowsCollaboratorsAddRequest,
    ) -> Result<AdminWorkflowsCollaboratorsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.workflows.collaborators.remove`](https://docs.slack.dev/reference/methods/admin.workflows.collaborators.remove): Remove collaborators from workflows within the team or enterprise
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.workflows.collaborators.remove")]
    pub async fn admin_workflows_collaborators_remove(
        &self,
        request: &AdminWorkflowsCollaboratorsRemoveRequest,
    ) -> Result<AdminWorkflowsCollaboratorsRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.workflows.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.workflows.permissions.lookup): Look up the permissions for a set of workflows
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.workflows.permissions.lookup")]
    pub async fn admin_workflows_permissions_lookup(
        &self,
        request: &AdminWorkflowsPermissionsLookupRequest,
    ) -> Result<AdminWorkflowsPermissionsLookupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.workflows.search`](https://docs.slack.dev/reference/methods/admin.workflows.search): Search workflows within the team or enterprise
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.workflows.search")]
    pub async fn admin_workflows_search(
        &self,
        request: &AdminWorkflowsSearchRequest,
    ) -> Result<AdminWorkflowsSearchResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.workflows.triggers.types.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.workflows.triggers.types.permissions.lookup): List the permissions for using each trigger type
    ///
    /// Required scopes:
    ///
    /// - user token: `client`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.workflows.triggers.types.permissions.lookup")]
    pub async fn admin_workflows_triggers_types_permissions_lookup(
        &self,
        request: &AdminWorkflowsTriggersTypesPermissionsLookupRequest,
    ) -> Result<AdminWorkflowsTriggersTypesPermissionsLookupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.workflows.triggers.types.permissions.set`](https://docs.slack.dev/reference/methods/admin.workflows.triggers.types.permissions.set): Set the permissions for using a trigger type
    ///
    /// Required scopes:
    ///
    /// - user token: `client`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.workflows.triggers.types.permissions.set")]
    pub async fn admin_workflows_triggers_types_permissions_set(
        &self,
        request: &AdminWorkflowsTriggersTypesPermissionsSetRequest,
    ) -> Result<AdminWorkflowsTriggersTypesPermissionsSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.workflows.unpublish`](https://docs.slack.dev/reference/methods/admin.workflows.unpublish): Unpublish workflows within the team or enterprise
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.workflows.unpublish")]
    pub async fn admin_workflows_unpublish(
        &self,
        request: &AdminWorkflowsUnpublishRequest,
    ) -> Result<AdminWorkflowsUnpublishResponse, SlackError> {
        self.call(request).await
    }
}
