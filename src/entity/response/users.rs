use crate::SlackApiResponseMetadata;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SlackApiUserConversionResponse {
    pub channels: Vec<Channel>,
    pub response_metadata: Option<SlackApiResponseMetadata>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Channel {
    context_team_id: String,
    created: u64,
    creator: String,
    id: String,
    is_archived: bool,
    is_channel: bool,
    is_ext_shared: bool,
    is_general: bool,
    is_group: bool,
    is_im: bool,
    is_mpim: bool,
    is_org_shared: bool,
    is_pending_ext_shared: bool,
    is_private: bool,
    is_shared: bool,
    name: String,
    name_normalized: String,
    // parent_conversation: Option<String>,//
    // pending_connected_team_ids: Array [],
    // pending_shared: Array [],
    // previous_names: Array [],
    purpose: Purpose,
    // "shared_team_ids": Array [
    // String,
    // ],
    topic: Topic,
    unlinked: u64,
    updated: u64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Purpose {
    creator: String,
    last_set: u64,
    value: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Topic {
    creator: String,
    last_set: u64,
    value: String,
}
