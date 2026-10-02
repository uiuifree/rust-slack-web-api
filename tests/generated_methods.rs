// このファイルは codegen/generate.py が生成する。手で編集しない

//! 全メソッドを、ドキュメントの成功例を返すモックサーバーに対して呼ぶ

use slack_web_api::api::*;
use slack_web_api::{CursorPaginated, NextCursor, ResponseMetadata, SlackClient};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup(name: &str, body: &str) -> (MockServer, SlackClient) {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/api/{name}")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body.to_owned(), "application/json"))
        .mount(&server)
        .await;
    let client = SlackClient::builder()
        .token("xoxb-test")
        .base_url(format!("{}/api", server.uri()))
        .build();
    (server, client)
}

/// 応答例の値が、型に読んで書き戻したあとも全部残っているか（null・空配列・空オブジェクトは除く）。
/// 数値と数字の文字列は同じとみなす（ゆるい読み取りで型が寄るため）
fn assert_preserved<T: serde::Serialize>(name: &str, original: &str, res: &T) {
    let original: serde_json::Value = serde_json::from_str(original).unwrap();
    let back = serde_json::to_value(res).unwrap();
    let mut missing = vec![];
    covers(&original, &back, String::new(), &mut missing);
    missing.retain(|p| p != ".ok");
    assert!(missing.is_empty(), "{name}: lost {missing:?}");
}

fn covers(a: &serde_json::Value, b: &serde_json::Value, at: String, missing: &mut Vec<String>) {
    use serde_json::Value::*;
    match (a, b) {
        (Null, _) => {}
        (Object(x), _) if x.is_empty() => {}
        (Array(x), _) if x.is_empty() => {}
        (Object(x), Object(y)) => {
            for (k, v) in x {
                match y.get(k) {
                    Some(w) => covers(v, w, format!("{at}.{k}"), missing),
                    None if v.is_null()
                        || v.as_array().is_some_and(|a| a.is_empty())
                        || v.as_object().is_some_and(|o| o.is_empty()) => {}
                    None => missing.push(format!("{at}.{k}")),
                }
            }
        }
        (Array(x), Array(y)) if x.len() == y.len() => {
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                covers(v, w, format!("{at}[{i}]"), missing);
            }
        }
        (x, y) if x == y => {}
        (x, y) if scalar_text(x).is_some() && scalar_text(x) == scalar_text(y) => {}
        _ => missing.push(at),
    }
}

fn scalar_text(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => n.as_f64().map(|f| f.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_analytics_get_file() {
    let req = AdminAnalyticsGetFileRequest::new("member")
        .date("2020-09-01")
        .metadata_only(true);
    let (_server, client) = setup("admin.analytics.getFile", "gzip-bytes").await;
    assert_eq!(
        &client.admin_analytics_get_file(&req).await.unwrap()[..],
        b"gzip-bytes"
    );
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_analytics_messages_activity() {
    let req = AdminAnalyticsMessagesActivityRequest::new("x")
        .oldest_ts("x")
        .latest_ts("x")
        .cursor("cGFnZTo0")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminAnalyticsMessagesActivityResponse::default().next_cursor(),
        None
    );
    let page = AdminAnalyticsMessagesActivityResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.analytics.messages.activity", r#"{"ok": true, "message_activities": [{"channel_id": "C123ABC456", "timestamp": "1234567890.123456", "unique_user_reactions_count": 15, "unique_user_shares_count": 8, "unique_user_views_count": 142, "unique_user_clicks_count": 23, "unique_views_client": {"desktop_views_count": 89, "mobile_views_count": 42, "web_views_count": 11}, "unique_stats_by_department": [{"department": "Engineering", "views": 45, "reactions": 8, "shares": 3, "clicks": 12}, {"department": "Product", "views": 32, "reactions": 4, "shares": 2, "clicks": 7}], "unique_stats_by_org": [{"team_id": "T123ABC456", "views": 98, "reactions": 12, "shares": 5, "clicks": 18}]}], "response_metadata": {"next_cursor": "abcd..."}}"#).await;
        let res = client
            .admin_analytics_messages_activity(&req)
            .await
            .expect("admin.analytics.messages.activity");
        assert_preserved(
            "admin.analytics.messages.activity",
            r#"{"ok": true, "message_activities": [{"channel_id": "C123ABC456", "timestamp": "1234567890.123456", "unique_user_reactions_count": 15, "unique_user_shares_count": 8, "unique_user_views_count": 142, "unique_user_clicks_count": 23, "unique_views_client": {"desktop_views_count": 89, "mobile_views_count": 42, "web_views_count": 11}, "unique_stats_by_department": [{"department": "Engineering", "views": 45, "reactions": 8, "shares": 3, "clicks": 12}, {"department": "Product", "views": 32, "reactions": 4, "shares": 2, "clicks": 7}], "unique_stats_by_org": [{"team_id": "T123ABC456", "views": 98, "reactions": 12, "shares": 5, "clicks": 18}]}], "response_metadata": {"next_cursor": "abcd..."}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_analytics_messages_metadata() {
    let req = AdminAnalyticsMessagesMetadataRequest::new("x")
        .oldest_ts("x")
        .latest_ts("x")
        .cursor("cGFnZTo0");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminAnalyticsMessagesMetadataResponse::default().next_cursor(),
        None
    );
    let page = AdminAnalyticsMessagesMetadataResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.analytics.messages.metadata", r#"{"ok": true, "messages": [{"type": "message", "ts": "1234567890.123456", "user_id": "U123ABC456", "text_character_count": 42, "subtype": "bot_message", "thread_ts": "1234567890.123456", "reactions": [{"name": "thumbsup", "count": 5}], "files": [{"id": "F123ABC456", "created": 1234567890, "timestamp": 1234567890, "mimetype": "image/png", "filetype": "png", "pretty_type": "PNG"}]}], "response_metadata": {"next_cursor": "abcd..."}}"#).await;
        let res = client
            .admin_analytics_messages_metadata(&req)
            .await
            .expect("admin.analytics.messages.metadata");
        assert_preserved(
            "admin.analytics.messages.metadata",
            r#"{"ok": true, "messages": [{"type": "message", "ts": "1234567890.123456", "user_id": "U123ABC456", "text_character_count": 42, "subtype": "bot_message", "thread_ts": "1234567890.123456", "reactions": [{"name": "thumbsup", "count": 5}], "files": [{"id": "F123ABC456", "created": 1234567890, "timestamp": 1234567890, "mimetype": "image/png", "filetype": "png", "pretty_type": "PNG"}]}], "response_metadata": {"next_cursor": "abcd..."}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_activities_list() {
    let req = AdminAppsActivitiesListRequest::new()
        .app_id("A12345")
        .team_id("T12345")
        .cursor("bG9nX2lkOjc5NjQ1NA==")
        .limit(1)
        .min_log_level("info")
        .log_event_type("test_log_event")
        .source("slack")
        .component_type("workflows")
        .component_id("Wf013SMGL4V9")
        .trace_id("Tr432f2")
        .min_date_created(1)
        .max_date_created(1)
        .sort_direction("asc");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminAppsActivitiesListResponse::default().next_cursor(),
        None
    );
    let page = AdminAppsActivitiesListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.apps.activities.list", r#"{"ok": true, "activities": [{"app_id": "A123456789", "level": "info", "event_type": "function_execution_started", "source": "slack", "component_type": "functions", "component_id": "Fn123", "payload": {"function_name": "Reverse", "function_type": "app"}, "created": 1650463798824317, "trace_id": "Tr123"}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .admin_apps_activities_list(&req)
            .await
            .expect("admin.apps.activities.list");
        assert_preserved(
            "admin.apps.activities.list",
            r#"{"ok": true, "activities": [{"app_id": "A123456789", "level": "info", "event_type": "function_execution_started", "source": "slack", "component_type": "functions", "component_id": "Fn123", "payload": {"function_name": "Reverse", "function_type": "app"}, "created": 1650463798824317, "trace_id": "Tr123"}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_approve() {
    let req = AdminAppsApproveRequest::new()
        .allow_child_auto_install(true)
        .app_id("A12345")
        .request_id("Ar12345")
        .team_id("T12345")
        .enterprise_id("E12345")
        .user_scopes("emoji:read,pins:read")
        .bot_scopes("emoji:read,pins:read");
    {
        let (_server, client) = setup("admin.apps.approve", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_approve(&req)
            .await
            .expect("admin.apps.approve");
        assert_preserved("admin.apps.approve", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_approved_list() {
    let req = AdminAppsApprovedListRequest::new()
        .limit(1)
        .cursor("5c3e53d5")
        .team_id("T0HFE6EBT")
        .enterprise_id("E0AS553RN")
        .certified(true);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminAppsApprovedListResponse::default().next_cursor(), None);
    let page = AdminAppsApprovedListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.apps.approved.list", r#"{"ok": true, "approved_apps": [{"app": {"id": "A0W7UKG8E", "name": "My Test App", "description": "test app", "help_url": "https://www.slack.com", "privacy_policy_url": "https://www.slack.com", "app_homepage_url": "https://www.slack.com", "app_directory_url": "https://myteam.enterprise.slack.com/apps/A0W7UKG8E-my-test-app", "is_app_directory_approved": false, "is_internal": false, "developer_type": "third_party", "socket_mode_enabled": false, "icons": {"image_32": "https://302674312496446w_2bd4ea1ad1f89a23c242_32.png", "image_36": "https://302674312496446w_2bd4ea1ad1f89a23c242_36.png", "image_48": "https://302674312496446w_2bd4ea1ad1f89a23c242_48.png", "image_64": "https://302674312496446w_2bd4ea1ad1f89a23c242_64.png", "image_72": "https://302674312496446w_2bd4ea1ad1f89a23c242_72.png", "image_96": "https://302674312496446w_2bd4ea1ad1f89a23c242_96.png", "image_128": "https://30267341249446w6_2bd4ea1ad1f89a23c242_128.png", "image_192": "https://30267431249446w6_2bd4ea1ad1f89a23c242_192.png", "image_512": "https://30267431249446w6_2bd4ea1ad1f89a23c242_512.png", "image_1024": "https://3026743124446w96_2bd4ea1ad1f89a23c242_1024.png", "image_original": "https://302674446w12496_2bd4ea1ad1f89a23c242_original.png"}, "additional_info": ""}, "scopes": [{"name": "bot", "description": "Add the ability for people to direct message or mention @my_test_app", "is_sensitive": true, "token_type": "bot"}], "date_updated": 1574296707, "last_resolved_by": {"actor_id": "W0G82F4FD", "actor_type": "user"}}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .admin_apps_approved_list(&req)
            .await
            .expect("admin.apps.approved.list");
        assert_preserved(
            "admin.apps.approved.list",
            r#"{"ok": true, "approved_apps": [{"app": {"id": "A0W7UKG8E", "name": "My Test App", "description": "test app", "help_url": "https://www.slack.com", "privacy_policy_url": "https://www.slack.com", "app_homepage_url": "https://www.slack.com", "app_directory_url": "https://myteam.enterprise.slack.com/apps/A0W7UKG8E-my-test-app", "is_app_directory_approved": false, "is_internal": false, "developer_type": "third_party", "socket_mode_enabled": false, "icons": {"image_32": "https://302674312496446w_2bd4ea1ad1f89a23c242_32.png", "image_36": "https://302674312496446w_2bd4ea1ad1f89a23c242_36.png", "image_48": "https://302674312496446w_2bd4ea1ad1f89a23c242_48.png", "image_64": "https://302674312496446w_2bd4ea1ad1f89a23c242_64.png", "image_72": "https://302674312496446w_2bd4ea1ad1f89a23c242_72.png", "image_96": "https://302674312496446w_2bd4ea1ad1f89a23c242_96.png", "image_128": "https://30267341249446w6_2bd4ea1ad1f89a23c242_128.png", "image_192": "https://30267431249446w6_2bd4ea1ad1f89a23c242_192.png", "image_512": "https://30267431249446w6_2bd4ea1ad1f89a23c242_512.png", "image_1024": "https://3026743124446w96_2bd4ea1ad1f89a23c242_1024.png", "image_original": "https://302674446w12496_2bd4ea1ad1f89a23c242_original.png"}, "additional_info": ""}, "scopes": [{"name": "bot", "description": "Add the ability for people to direct message or mention @my_test_app", "is_sensitive": true, "token_type": "bot"}], "date_updated": 1574296707, "last_resolved_by": {"actor_id": "W0G82F4FD", "actor_type": "user"}}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_clear_resolution() {
    let req = AdminAppsClearResolutionRequest::new("A12345")
        .team_id("T12345")
        .enterprise_id("E12345");
    {
        let (_server, client) = setup("admin.apps.clearResolution", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_clear_resolution(&req)
            .await
            .expect("admin.apps.clearResolution");
        assert_preserved("admin.apps.clearResolution", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_config_lookup() {
    let req = AdminAppsConfigLookupRequest::new()
        .app_ids(vec!["A1".to_string(), "A2".to_string()])
        .rich_link_preview_types(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.apps.config.lookup", r#"{"ok": true, "configs": [{"app_id": "A123", "workflow_auth_strategy": "end_user_only", "rich_link_preview_type": "limited_details", "domain_restrictions": {"emails": ["my-corp.com", "yourcorp.com"], "urls": ["mycorp.company.com", "mycorp2.company.com"]}}, {"app_id": "A456", "workflow_auth_strategy": "builder_choice", "rich_link_preview_type": "no_preview", "domain_restrictions": {"emails": [], "urls": []}}]}"#).await;
        let res = client
            .admin_apps_config_lookup(&req)
            .await
            .expect("admin.apps.config.lookup");
        assert_preserved(
            "admin.apps.config.lookup",
            r#"{"ok": true, "configs": [{"app_id": "A123", "workflow_auth_strategy": "end_user_only", "rich_link_preview_type": "limited_details", "domain_restrictions": {"emails": ["my-corp.com", "yourcorp.com"], "urls": ["mycorp.company.com", "mycorp2.company.com"]}}, {"app_id": "A456", "workflow_auth_strategy": "builder_choice", "rich_link_preview_type": "no_preview", "domain_restrictions": {"emails": [], "urls": []}}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_config_set() {
    let req = AdminAppsConfigSetRequest::new("A12345")
        .workflow_auth_strategy("x")
        .rich_link_preview_type("x")
        .domain_restrictions(serde_json::json!({"k": "v"}));
    {
        let (_server, client) = setup("admin.apps.config.set", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_config_set(&req)
            .await
            .expect("admin.apps.config.set");
        assert_preserved("admin.apps.config.set", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_mcp_servers_list() {
    let req = AdminAppsMcpServersListRequest::new()
        .limit(1)
        .cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminAppsMcpServersListResponse::default().next_cursor(),
        None
    );
    let page = AdminAppsMcpServersListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.apps.mcp.servers.list", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_mcp_servers_list(&req)
            .await
            .expect("admin.apps.mcp.servers.list");
        assert_preserved("admin.apps.mcp.servers.list", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_mcp_servers_permissions_list() {
    let req = AdminAppsMcpServersPermissionsListRequest::new("A0000000001");
    {
        let (_server, client) =
            setup("admin.apps.mcp.servers.permissions.list", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_mcp_servers_permissions_list(&req)
            .await
            .expect("admin.apps.mcp.servers.permissions.list");
        assert_preserved(
            "admin.apps.mcp.servers.permissions.list",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_mcp_servers_permissions_set() {
    let req = AdminAppsMcpServersPermissionsSetRequest::new("A0000000001", "Amcp0000000001", "x")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .usergroup_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) =
            setup("admin.apps.mcp.servers.permissions.set", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_mcp_servers_permissions_set(&req)
            .await
            .expect("admin.apps.mcp.servers.permissions.set");
        assert_preserved(
            "admin.apps.mcp.servers.permissions.set",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_permissions_add() {
    let req = AdminAppsPermissionsAddRequest::new("A0000000001")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .usergroup_ids(vec!["A1".to_string(), "A2".to_string()])
        .channel_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.apps.permissions.add", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_permissions_add(&req)
            .await
            .expect("admin.apps.permissions.add");
        assert_preserved("admin.apps.permissions.add", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_permissions_list() {
    let req = AdminAppsPermissionsListRequest::new("A0000000001");
    {
        let (_server, client) = setup("admin.apps.permissions.list", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_permissions_list(&req)
            .await
            .expect("admin.apps.permissions.list");
        assert_preserved("admin.apps.permissions.list", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_permissions_remove() {
    let req = AdminAppsPermissionsRemoveRequest::new("A0000000001")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .usergroup_ids(vec!["A1".to_string(), "A2".to_string()])
        .channel_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.apps.permissions.remove", r#"{"ok": true, "permission_type": "everyone", "channel_restriction_mode": "specific_channels", "channel_ids": ["C00000001"]}"#).await;
        let res = client
            .admin_apps_permissions_remove(&req)
            .await
            .expect("admin.apps.permissions.remove");
        assert_preserved(
            "admin.apps.permissions.remove",
            r#"{"ok": true, "permission_type": "everyone", "channel_restriction_mode": "specific_channels", "channel_ids": ["C00000001"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_permissions_set() {
    let req = AdminAppsPermissionsSetRequest::new("A0000000001", "x")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .usergroup_ids(vec!["A1".to_string(), "A2".to_string()])
        .channel_restriction_mode("x")
        .channel_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.apps.permissions.set", r#"{"ok": true, "permission_type": "everyone", "channel_restriction_mode": "specific_channels", "channel_ids": ["C00000001", "C00000002"]}"#).await;
        let res = client
            .admin_apps_permissions_set(&req)
            .await
            .expect("admin.apps.permissions.set");
        assert_preserved(
            "admin.apps.permissions.set",
            r#"{"ok": true, "permission_type": "everyone", "channel_restriction_mode": "specific_channels", "channel_ids": ["C00000001", "C00000002"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_requests_cancel() {
    let req = AdminAppsRequestsCancelRequest::new("Ar12345")
        .team_id("T12345")
        .enterprise_id("E12345");
    {
        let (_server, client) = setup("admin.apps.requests.cancel", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_requests_cancel(&req)
            .await
            .expect("admin.apps.requests.cancel");
        assert_preserved("admin.apps.requests.cancel", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_requests_list() {
    let req = AdminAppsRequestsListRequest::new()
        .limit(1)
        .cursor("5c3e53d5")
        .team_id("x")
        .enterprise_id("x")
        .certified(true);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminAppsRequestsListResponse::default().next_cursor(), None);
    let page = AdminAppsRequestsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.apps.requests.list", r#"{"ok": true, "app_requests": [{"id": "Ar0XJGFLMLS", "app": {"id": "A061BL8RQ0", "name": "Test App", "description": "", "help_url": "", "privacy_policy_url": "https://testapp.com/privacy", "app_homepage_url": "", "app_directory_url": "https://acmecorp.slack.com/apps/A061BL8RQ0-test-app", "is_app_directory_approved": true, "is_internal": true, "developer_type": "internal", "socket_mode_enabled": false, "icons": {"image_32": "/cdn/157658203/img/testapp/service_32.png", "image_36": "/cdn/157658203/img/testapp/service_36.png", "image_48": "/cdn/157658203/img/testapp/service_48.png", "image_64": "/cdn/157658203/img/testapp/service_64.png", "image_72": "/cdn/157658203/img/testapp/service_72.png", "image_96": "/cdn/157658203/img/testapp/service_96.png", "image_128": "/cdn/157258203/img/testapp/service_128.png", "image_192": "/cdn/157258203/img/testapp/service_192.png", "image_512": "/cdn/15758203/img/testapp/service_512.png", "image_1024": "/cdn/15258203/img/testapp/service_1024.png"}, "additional_info": ""}, "previous_resolution": null, "user": {"id": "W08RA9G5HR", "name": "Jane Doe", "email": "janedoe@example.com"}, "team": {"id": "T0M94LNUCR", "name": "Acme Corp", "domain": "acmecorp"}, "scopes": [{"name": "incoming-webhook", "description": "Post messages to specific channels in Slack", "is_sensitive": false, "token_type": "user", "is_optional": true, "is_approved": false}], "message": "Could you please install this app for me, it does everything I need.", "is_user_app_collaborator": false, "date_created": 1578956327}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .admin_apps_requests_list(&req)
            .await
            .expect("admin.apps.requests.list");
        assert_preserved(
            "admin.apps.requests.list",
            r#"{"ok": true, "app_requests": [{"id": "Ar0XJGFLMLS", "app": {"id": "A061BL8RQ0", "name": "Test App", "description": "", "help_url": "", "privacy_policy_url": "https://testapp.com/privacy", "app_homepage_url": "", "app_directory_url": "https://acmecorp.slack.com/apps/A061BL8RQ0-test-app", "is_app_directory_approved": true, "is_internal": true, "developer_type": "internal", "socket_mode_enabled": false, "icons": {"image_32": "/cdn/157658203/img/testapp/service_32.png", "image_36": "/cdn/157658203/img/testapp/service_36.png", "image_48": "/cdn/157658203/img/testapp/service_48.png", "image_64": "/cdn/157658203/img/testapp/service_64.png", "image_72": "/cdn/157658203/img/testapp/service_72.png", "image_96": "/cdn/157658203/img/testapp/service_96.png", "image_128": "/cdn/157258203/img/testapp/service_128.png", "image_192": "/cdn/157258203/img/testapp/service_192.png", "image_512": "/cdn/15758203/img/testapp/service_512.png", "image_1024": "/cdn/15258203/img/testapp/service_1024.png"}, "additional_info": ""}, "previous_resolution": null, "user": {"id": "W08RA9G5HR", "name": "Jane Doe", "email": "janedoe@example.com"}, "team": {"id": "T0M94LNUCR", "name": "Acme Corp", "domain": "acmecorp"}, "scopes": [{"name": "incoming-webhook", "description": "Post messages to specific channels in Slack", "is_sensitive": false, "token_type": "user", "is_optional": true, "is_approved": false}], "message": "Could you please install this app for me, it does everything I need.", "is_user_app_collaborator": false, "date_created": 1578956327}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_restrict() {
    let req = AdminAppsRestrictRequest::new()
        .app_id("A12345")
        .request_id("Ar12345")
        .team_id("T12345")
        .enterprise_id("E12345");
    {
        let (_server, client) = setup("admin.apps.restrict", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_restrict(&req)
            .await
            .expect("admin.apps.restrict");
        assert_preserved("admin.apps.restrict", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_restricted_list() {
    let req = AdminAppsRestrictedListRequest::new()
        .limit(1)
        .cursor("5c3e53d5")
        .team_id("T0HFE6EBT")
        .enterprise_id("E0AS553RN")
        .certified(true);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminAppsRestrictedListResponse::default().next_cursor(),
        None
    );
    let page = AdminAppsRestrictedListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.apps.restricted.list", r#"{"ok": true, "restricted_apps": [{"app": {"id": "A0FDLP8M2L", "name": "My Test App", "description": "A fun test app for Slack", "help_url": "https://example.com", "privacy_policy_url": "https://example.com", "app_homepage_url": "https://example.com", "app_directory_url": "https://myteam.enterprise.slack.com/apps/A0FDLP8M2L-my-test-app", "is_app_directory_approved": true, "is_internal": false, "developer_type": "third_party", "socket_mode_enabled": false, "icons": {"image_32": "https://143326534038rl8788_eb57dbc818daa4ba15d6_32.png", "image_36": "https://143326534038rl8788_eb57dbc818daa4ba15d6_36.png", "image_48": "https://143326534038rl8788_eb57dbc818daa4ba15d6_48.png", "image_64": "https://143326534038rl8788_eb57dbc818daa4ba15d6_64.png", "image_72": "https://143326534038rl8788_eb57dbc818daa4ba15d6_72.png", "image_96": "https://143326534038rl8788_eb57dbc818daa4ba15d6_96.png", "image_128": "https://4332653438rl87808_eb57dbc818daa4ba15d6_128.png", "image_192": "https://4332653438rl87808_eb57dbc818daa4ba15d6_192.png", "image_512": "https://4332653438rl87808_eb57dbc818daa4ba15d6_512.png", "image_1024": "https://1433265338rl878408_eb57dbc818daa4ba15d6_1024.png", "image_original": "https://143338rl8782653408_eb57dbc818daa4ba15d6_original.png"}, "additional_info": ""}, "scopes": [{"name": "files:write:user", "description": "Upload, edit, and delete files on the user‟s behalf", "is_sensitive": true, "token_type": "user"}], "date_updated": 1574296721, "last_resolved_by": {"actor_id": "W0G82LMFD", "actor_type": "user"}}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .admin_apps_restricted_list(&req)
            .await
            .expect("admin.apps.restricted.list");
        assert_preserved(
            "admin.apps.restricted.list",
            r#"{"ok": true, "restricted_apps": [{"app": {"id": "A0FDLP8M2L", "name": "My Test App", "description": "A fun test app for Slack", "help_url": "https://example.com", "privacy_policy_url": "https://example.com", "app_homepage_url": "https://example.com", "app_directory_url": "https://myteam.enterprise.slack.com/apps/A0FDLP8M2L-my-test-app", "is_app_directory_approved": true, "is_internal": false, "developer_type": "third_party", "socket_mode_enabled": false, "icons": {"image_32": "https://143326534038rl8788_eb57dbc818daa4ba15d6_32.png", "image_36": "https://143326534038rl8788_eb57dbc818daa4ba15d6_36.png", "image_48": "https://143326534038rl8788_eb57dbc818daa4ba15d6_48.png", "image_64": "https://143326534038rl8788_eb57dbc818daa4ba15d6_64.png", "image_72": "https://143326534038rl8788_eb57dbc818daa4ba15d6_72.png", "image_96": "https://143326534038rl8788_eb57dbc818daa4ba15d6_96.png", "image_128": "https://4332653438rl87808_eb57dbc818daa4ba15d6_128.png", "image_192": "https://4332653438rl87808_eb57dbc818daa4ba15d6_192.png", "image_512": "https://4332653438rl87808_eb57dbc818daa4ba15d6_512.png", "image_1024": "https://1433265338rl878408_eb57dbc818daa4ba15d6_1024.png", "image_original": "https://143338rl8782653408_eb57dbc818daa4ba15d6_original.png"}, "additional_info": ""}, "scopes": [{"name": "files:write:user", "description": "Upload, edit, and delete files on the user‟s behalf", "is_sensitive": true, "token_type": "user"}], "date_updated": 1574296721, "last_resolved_by": {"actor_id": "W0G82LMFD", "actor_type": "user"}}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_apps_uninstall() {
    let req = AdminAppsUninstallRequest::new("A12345")
        .team_ids("x")
        .enterprise_id("E12345");
    {
        let (_server, client) = setup("admin.apps.uninstall", r#"{"ok": true}"#).await;
        let res = client
            .admin_apps_uninstall(&req)
            .await
            .expect("admin.apps.uninstall");
        assert_preserved("admin.apps.uninstall", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_audit_anomaly_allow_get_item() {
    let req = AdminAuditAnomalyAllowGetItemRequest::new();
    {
        let (_server, client) = setup("admin.audit.anomaly.allow.getItem", r#"{"ok": true}"#).await;
        let res = client
            .admin_audit_anomaly_allow_get_item(&req)
            .await
            .expect("admin.audit.anomaly.allow.getItem");
        assert_preserved("admin.audit.anomaly.allow.getItem", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_audit_anomaly_allow_update_item() {
    let req = AdminAuditAnomalyAllowUpdateItemRequest::new()
        .trusted_cidr(vec![serde_json::json!({"k": "v"})])
        .trusted_asns(vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) =
            setup("admin.audit.anomaly.allow.updateItem", r#"{"ok": true}"#).await;
        let res = client
            .admin_audit_anomaly_allow_update_item(&req)
            .await
            .expect("admin.audit.anomaly.allow.updateItem");
        assert_preserved(
            "admin.audit.anomaly.allow.updateItem",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_auth_policy_assign_entities() {
    let req = AdminAuthPolicyAssignEntitiesRequest::new(
        "email_password",
        "x",
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.auth.policy.assignEntities", r#"{"ok": true}"#).await;
        let res = client
            .admin_auth_policy_assign_entities(&req)
            .await
            .expect("admin.auth.policy.assignEntities");
        assert_preserved("admin.auth.policy.assignEntities", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_auth_policy_get_entities() {
    let req = AdminAuthPolicyGetEntitiesRequest::new("email_password")
        .entity_type("x")
        .limit(1)
        .cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminAuthPolicyGetEntitiesResponse::default().next_cursor(),
        None
    );
    let page = AdminAuthPolicyGetEntitiesResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.auth.policy.getEntities", r#"{"ok": true, "entities": [{"entity_type": "USER", "entity_id": "U1234", "date_added": 1620836993}], "entity_total_count": 1}"#).await;
        let res = client
            .admin_auth_policy_get_entities(&req)
            .await
            .expect("admin.auth.policy.getEntities");
        assert_preserved(
            "admin.auth.policy.getEntities",
            r#"{"ok": true, "entities": [{"entity_type": "USER", "entity_id": "U1234", "date_added": 1620836993}], "entity_total_count": 1}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_auth_policy_remove_entities() {
    let req = AdminAuthPolicyRemoveEntitiesRequest::new(
        "email_password",
        "x",
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.auth.policy.removeEntities", r#"{"ok": true}"#).await;
        let res = client
            .admin_auth_policy_remove_entities(&req)
            .await
            .expect("admin.auth.policy.removeEntities");
        assert_preserved("admin.auth.policy.removeEntities", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_barriers_create() {
    let req = AdminBarriersCreateRequest::new(
        "x",
        vec!["A1".to_string(), "A2".to_string()],
        vec![serde_json::json!({"k": "v"})],
    );
    {
        let (_server, client) = setup("admin.barriers.create", r#"{"ok": true, "barrier": {"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}, {"id": "S03TNHF56UR", "name": "External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "date_update": 1660224825}}"#).await;
        let res = client
            .admin_barriers_create(&req)
            .await
            .expect("admin.barriers.create");
        assert_preserved(
            "admin.barriers.create",
            r#"{"ok": true, "barrier": {"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}, {"id": "S03TNHF56UR", "name": "External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "date_update": 1660224825}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_barriers_delete() {
    let req = AdminBarriersDeleteRequest::new("x");
    {
        let (_server, client) = setup("admin.barriers.delete", r#"{"ok": true}"#).await;
        let res = client
            .admin_barriers_delete(&req)
            .await
            .expect("admin.barriers.delete");
        assert_preserved("admin.barriers.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_barriers_list() {
    let req = AdminBarriersListRequest::new().limit(1).cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminBarriersListResponse::default().next_cursor(), None);
    let page = AdminBarriersListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.barriers.list", r#"{"ok": true, "barriers": [{"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHF56UR", "name": "External Contracting Team"}, {"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "date_update": 1660224825}]}"#).await;
        let res = client
            .admin_barriers_list(&req)
            .await
            .expect("admin.barriers.list");
        assert_preserved(
            "admin.barriers.list",
            r#"{"ok": true, "barriers": [{"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHF56UR", "name": "External Contracting Team"}, {"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "date_update": 1660224825}]}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("admin.barriers.list", r#"{"ok": true, "barriers": [{"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHF56UR", "name": "External Contracting Team"}, {"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "response_metadata": {"next_cursor": "dGVhbTpDMDYxRkE1UEI="}, "date_update": 1660224825}]}"#).await;
        let res = client
            .admin_barriers_list(&req)
            .await
            .expect("admin.barriers.list");
        assert_preserved(
            "admin.barriers.list",
            r#"{"ok": true, "barriers": [{"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHF56UR", "name": "External Contracting Team"}, {"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "response_metadata": {"next_cursor": "dGVhbTpDMDYxRkE1UEI="}, "date_update": 1660224825}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_barriers_update() {
    let req = AdminBarriersUpdateRequest::new(
        "x",
        "x",
        vec!["A1".to_string(), "A2".to_string()],
        vec![serde_json::json!({"k": "v"})],
    );
    {
        let (_server, client) = setup("admin.barriers.update", r#"{"ok": true, "barrier": {"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "date_update": 1660224825}}"#).await;
        let res = client
            .admin_barriers_update(&req)
            .await
            .expect("admin.barriers.update");
        assert_preserved(
            "admin.barriers.update",
            r#"{"ok": true, "barrier": {"id": "Ba03T70KB2H3", "enterprise_id": "E03055H6DAS", "primary_usergroup": {"id": "S03TZK4A9H6", "name": "Company That Pays Contracting Teams"}, "barriered_from_usergroups": [{"id": "S03TNHGAUGZ", "name": "Another External Contracting Team"}], "restricted_subjects": ["im", "mpim", "call"], "date_update": 1660224825}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_archive() {
    let req = AdminConversationsArchiveRequest::new("C12345");
    {
        let (_server, client) = setup("admin.conversations.archive", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_archive(&req)
            .await
            .expect("admin.conversations.archive");
        assert_preserved("admin.conversations.archive", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_bulk_archive() {
    let req = AdminConversationsBulkArchiveRequest::new(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.conversations.bulkArchive", r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#).await;
        let res = client
            .admin_conversations_bulk_archive(&req)
            .await
            .expect("admin.conversations.bulkArchive");
        assert_preserved(
            "admin.conversations.bulkArchive",
            r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_bulk_delete() {
    let req = AdminConversationsBulkDeleteRequest::new(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.conversations.bulkDelete", r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#).await;
        let res = client
            .admin_conversations_bulk_delete(&req)
            .await
            .expect("admin.conversations.bulkDelete");
        assert_preserved(
            "admin.conversations.bulkDelete",
            r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_bulk_move() {
    let req = AdminConversationsBulkMoveRequest::new("x", vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.conversations.bulkMove", r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#).await;
        let res = client
            .admin_conversations_bulk_move(&req)
            .await
            .expect("admin.conversations.bulkMove");
        assert_preserved(
            "admin.conversations.bulkMove",
            r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_bulk_set_exclude_from_slack_ai() {
    let req = AdminConversationsBulkSetExcludeFromSlackAiRequest::new(
        vec!["A1".to_string(), "A2".to_string()],
        true,
    );
    {
        let (_server, client) = setup("admin.conversations.bulkSetExcludeFromSlackAi", r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#).await;
        let res = client
            .admin_conversations_bulk_set_exclude_from_slack_ai(&req)
            .await
            .expect("admin.conversations.bulkSetExcludeFromSlackAi");
        assert_preserved(
            "admin.conversations.bulkSetExcludeFromSlackAi",
            r#"{"ok": true, "bulk_action_id": "Ab123456", "not_added": [{"channel_id": "C12346", "error": "invalid_channel"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_bulk_set_properties() {
    let req = AdminConversationsBulkSetPropertiesRequest::new(
        vec!["A1".to_string(), "A2".to_string()],
        "{\"exclude_from_slack_ai\": true }",
    );
    {
        let (_server, client) =
            setup("admin.conversations.bulkSetProperties", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_bulk_set_properties(&req)
            .await
            .expect("admin.conversations.bulkSetProperties");
        assert_preserved(
            "admin.conversations.bulkSetProperties",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_convert_to_private() {
    let req =
        AdminConversationsConvertToPrivateRequest::new("C12345").name("new_private_channel_name");
    {
        let (_server, client) =
            setup("admin.conversations.convertToPrivate", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_convert_to_private(&req)
            .await
            .expect("admin.conversations.convertToPrivate");
        assert_preserved(
            "admin.conversations.convertToPrivate",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_convert_to_public() {
    let req = AdminConversationsConvertToPublicRequest::new("C12345");
    {
        let (_server, client) =
            setup("admin.conversations.convertToPublic", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_convert_to_public(&req)
            .await
            .expect("admin.conversations.convertToPublic");
        assert_preserved(
            "admin.conversations.convertToPublic",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_create() {
    let req = AdminConversationsCreateRequest::new("mychannel", true)
        .description("It's a good channel, Bront.")
        .org_wide(true)
        .team_id("x");
    {
        let (_server, client) = setup(
            "admin.conversations.create",
            r#"{"ok": true, "channel_id": "C12345"}"#,
        )
        .await;
        let res = client
            .admin_conversations_create(&req)
            .await
            .expect("admin.conversations.create");
        assert_preserved(
            "admin.conversations.create",
            r#"{"ok": true, "channel_id": "C12345"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_create_for_objects() {
    let req =
        AdminConversationsCreateForObjectsRequest::new("0019000000DmehKAAR", "00DGC00000024hsuWY")
            .invite_object_team(true);
    {
        let (_server, client) = setup(
            "admin.conversations.createForObjects",
            r#"{"ok": true, "channel_id": "C12345"}"#,
        )
        .await;
        let res = client
            .admin_conversations_create_for_objects(&req)
            .await
            .expect("admin.conversations.createForObjects");
        assert_preserved(
            "admin.conversations.createForObjects",
            r#"{"ok": true, "channel_id": "C12345"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_delete() {
    let req = AdminConversationsDeleteRequest::new("C12345");
    {
        let (_server, client) = setup("admin.conversations.delete", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_delete(&req)
            .await
            .expect("admin.conversations.delete");
        assert_preserved("admin.conversations.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_disconnect_shared() {
    let req = AdminConversationsDisconnectSharedRequest::new("C12345")
        .leaving_team_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) =
            setup("admin.conversations.disconnectShared", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_disconnect_shared(&req)
            .await
            .expect("admin.conversations.disconnectShared");
        assert_preserved(
            "admin.conversations.disconnectShared",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_ekm_list_original_connected_channel_info() {
    let req = AdminConversationsEkmListOriginalConnectedChannelInfoRequest::new()
        .channel_ids("x")
        .team_ids("x")
        .limit(1)
        .cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminConversationsEkmListOriginalConnectedChannelInfoResponse::default().next_cursor(),
        None
    );
    let page = AdminConversationsEkmListOriginalConnectedChannelInfoResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.conversations.ekm.listOriginalConnectedChannelInfo", r#"{"ok": true, "channels": [{"id": "string", "internal_team_ids": "array", "original_connected_host_id": "string", "original_connected_channel_id": "string"}]}"#).await;
        let res = client
            .admin_conversations_ekm_list_original_connected_channel_info(&req)
            .await
            .expect("admin.conversations.ekm.listOriginalConnectedChannelInfo");
        assert_preserved(
            "admin.conversations.ekm.listOriginalConnectedChannelInfo",
            r#"{"ok": true, "channels": [{"id": "string", "internal_team_ids": "array", "original_connected_host_id": "string", "original_connected_channel_id": "string"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_get_conversation_prefs() {
    let req = AdminConversationsGetConversationPrefsRequest::new("C12345");
    {
        let (_server, client) = setup("admin.conversations.getConversationPrefs", r#"{"ok": true, "prefs": {"who_can_post": {"type": "admin", "user": "U1234"}, "can_thread": {"type": "admin, owner", "user": "U1234,U5678"}}}"#).await;
        let res = client
            .admin_conversations_get_conversation_prefs(&req)
            .await
            .expect("admin.conversations.getConversationPrefs");
        assert_preserved(
            "admin.conversations.getConversationPrefs",
            r#"{"ok": true, "prefs": {"who_can_post": {"type": "admin", "user": "U1234"}, "can_thread": {"type": "admin, owner", "user": "U1234,U5678"}}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("admin.conversations.getConversationPrefs", r#"{"ok": true, "prefs": {"who_can_post": {"type": ["admin"], "user": ["ABCD4567E"]}, "can_thread": {"type": ["ra"], "user": []}, "enable_at_channel": {"enabled": false}, "enable_at_here": {"enabled": false}}}"#).await;
        let res = client
            .admin_conversations_get_conversation_prefs(&req)
            .await
            .expect("admin.conversations.getConversationPrefs");
        assert_preserved(
            "admin.conversations.getConversationPrefs",
            r#"{"ok": true, "prefs": {"who_can_post": {"type": ["admin"], "user": ["ABCD4567E"]}, "can_thread": {"type": ["ra"], "user": []}, "enable_at_channel": {"enabled": false}, "enable_at_here": {"enabled": false}}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_get_custom_retention() {
    let req = AdminConversationsGetCustomRetentionRequest::new("C12345678");
    {
        let (_server, client) = setup(
            "admin.conversations.getCustomRetention",
            r#"{"ok": true, "is_policy_enabled": true, "duration_days": 70}"#,
        )
        .await;
        let res = client
            .admin_conversations_get_custom_retention(&req)
            .await
            .expect("admin.conversations.getCustomRetention");
        assert_preserved(
            "admin.conversations.getCustomRetention",
            r#"{"ok": true, "is_policy_enabled": true, "duration_days": 70}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_get_teams() {
    let req = AdminConversationsGetTeamsRequest::new("C12345")
        .cursor("5c3e53d5")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminConversationsGetTeamsResponse::default().next_cursor(),
        None
    );
    let page = AdminConversationsGetTeamsResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup(
            "admin.conversations.getTeams",
            r#"{"ok": true, "team_ids": ["T1234", "T5679"]}"#,
        )
        .await;
        let res = client
            .admin_conversations_get_teams(&req)
            .await
            .expect("admin.conversations.getTeams");
        assert_preserved(
            "admin.conversations.getTeams",
            r#"{"ok": true, "team_ids": ["T1234", "T5679"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_invite() {
    let req =
        AdminConversationsInviteRequest::new(vec!["A1".to_string(), "A2".to_string()], "C12345");
    {
        let (_server, client) = setup("admin.conversations.invite", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_invite(&req)
            .await
            .expect("admin.conversations.invite");
        assert_preserved("admin.conversations.invite", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_link_objects() {
    let req =
        AdminConversationsLinkObjectsRequest::new("x", "0019000000DmehKAAR", "00DGC00000024hsuWY");
    {
        let (_server, client) = setup("admin.conversations.linkObjects", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_link_objects(&req)
            .await
            .expect("admin.conversations.linkObjects");
        assert_preserved("admin.conversations.linkObjects", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_lookup() {
    let req = AdminConversationsLookupRequest::new(vec!["A1".to_string(), "A2".to_string()], 1)
        .max_member_count(1)
        .cursor("x")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminConversationsLookupResponse::default().next_cursor(),
        None
    );
    let page = AdminConversationsLookupResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup(
            "admin.conversations.lookup",
            r#"{"ok": true, "channels": ["encoded_id_1", "encoded_id_2"]}"#,
        )
        .await;
        let res = client
            .admin_conversations_lookup(&req)
            .await
            .expect("admin.conversations.lookup");
        assert_preserved(
            "admin.conversations.lookup",
            r#"{"ok": true, "channels": ["encoded_id_1", "encoded_id_2"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_remove_custom_retention() {
    let req = AdminConversationsRemoveCustomRetentionRequest::new("C12345678");
    {
        let (_server, client) = setup(
            "admin.conversations.removeCustomRetention",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .admin_conversations_remove_custom_retention(&req)
            .await
            .expect("admin.conversations.removeCustomRetention");
        assert_preserved(
            "admin.conversations.removeCustomRetention",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_rename() {
    let req = AdminConversationsRenameRequest::new("C12345", "x");
    {
        let (_server, client) = setup("admin.conversations.rename", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_rename(&req)
            .await
            .expect("admin.conversations.rename");
        assert_preserved("admin.conversations.rename", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_restrict_access_add_group() {
    let req = AdminConversationsRestrictAccessAddGroupRequest::new("x", "x").team_id("x");
    {
        let (_server, client) = setup(
            "admin.conversations.restrictAccess.addGroup",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .admin_conversations_restrict_access_add_group(&req)
            .await
            .expect("admin.conversations.restrictAccess.addGroup");
        assert_preserved(
            "admin.conversations.restrictAccess.addGroup",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_restrict_access_list_groups() {
    let req = AdminConversationsRestrictAccessListGroupsRequest::new("x").team_id("x");
    {
        let (_server, client) = setup(
            "admin.conversations.restrictAccess.listGroups",
            r#"{"ok": true, "group_ids": ["YOUR_GROUP_ID"]}"#,
        )
        .await;
        let res = client
            .admin_conversations_restrict_access_list_groups(&req)
            .await
            .expect("admin.conversations.restrictAccess.listGroups");
        assert_preserved(
            "admin.conversations.restrictAccess.listGroups",
            r#"{"ok": true, "group_ids": ["YOUR_GROUP_ID"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_restrict_access_remove_group() {
    let req = AdminConversationsRestrictAccessRemoveGroupRequest::new("x", "x", "x");
    {
        let (_server, client) = setup(
            "admin.conversations.restrictAccess.removeGroup",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .admin_conversations_restrict_access_remove_group(&req)
            .await
            .expect("admin.conversations.restrictAccess.removeGroup");
        assert_preserved(
            "admin.conversations.restrictAccess.removeGroup",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_search() {
    let req = AdminConversationsSearchRequest::new()
        .team_ids(vec!["A1".to_string(), "A2".to_string()])
        .connected_team_ids(vec!["A1".to_string(), "A2".to_string()])
        .query("announcement")
        .limit(1)
        .cursor("dXNlcjpVMEc5V0ZYTlo=")
        .search_channel_types(vec!["A1".to_string(), "A2".to_string()])
        .sort("name")
        .sort_dir("asc")
        .total_count_only(true);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminConversationsSearchResponse::default().next_cursor(),
        None
    );
    let page = AdminConversationsSearchResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    let top = AdminConversationsSearchResponse {
        next_cursor: Some("t".into()),
        ..Default::default()
    };
    assert_eq!(top.next_cursor(), Some("t"));
    {
        let (_server, client) = setup("admin.conversations.search", r#"{"ok": true, "conversations": [{"id": "GSEV0B5PY", "name": "privacy-channel", "purpose": "Group messaging with: @rita @nwhere @meanie", "member_count": -1, "created": 1578423973, "creator_id": "WPQ65MVKK", "is_private": true, "is_archived": true, "is_general": false, "last_activity_ts": 1583198954000200, "is_ext_shared": false, "is_global_shared": true, "is_org_default": false, "is_org_mandatory": false, "is_org_shared": true, "is_frozen": false, "connected_team_ids": [], "internal_team_ids_count": 4, "internal_team_ids_sample_team": "T013F30DBAB", "pending_connected_team_ids": [], "is_pending_ext_shared": false}, {"id": "C013JDPD6CR", "name": "proj-decomposed-monolith", "purpose": "", "member_count": 1, "created": 1588786531, "creator_id": "WPQ65MVKK", "is_private": false, "is_archived": false, "is_general": false, "last_activity_ts": 1589854024000200, "is_ext_shared": false, "is_global_shared": false, "is_org_default": false, "is_org_mandatory": false, "is_org_shared": true, "is_frozen": false, "connected_team_ids": [], "internal_team_ids_count": 1, "internal_team_ids_sample_team": "TPQ67R81F", "pending_connected_team_ids": [], "is_pending_ext_shared": false}], "next_cursor": "aWQ6Mw==", "total_count": 14823}"#).await;
        let res = client
            .admin_conversations_search(&req)
            .await
            .expect("admin.conversations.search");
        assert_preserved(
            "admin.conversations.search",
            r#"{"ok": true, "conversations": [{"id": "GSEV0B5PY", "name": "privacy-channel", "purpose": "Group messaging with: @rita @nwhere @meanie", "member_count": -1, "created": 1578423973, "creator_id": "WPQ65MVKK", "is_private": true, "is_archived": true, "is_general": false, "last_activity_ts": 1583198954000200, "is_ext_shared": false, "is_global_shared": true, "is_org_default": false, "is_org_mandatory": false, "is_org_shared": true, "is_frozen": false, "connected_team_ids": [], "internal_team_ids_count": 4, "internal_team_ids_sample_team": "T013F30DBAB", "pending_connected_team_ids": [], "is_pending_ext_shared": false}, {"id": "C013JDPD6CR", "name": "proj-decomposed-monolith", "purpose": "", "member_count": 1, "created": 1588786531, "creator_id": "WPQ65MVKK", "is_private": false, "is_archived": false, "is_general": false, "last_activity_ts": 1589854024000200, "is_ext_shared": false, "is_global_shared": false, "is_org_default": false, "is_org_mandatory": false, "is_org_shared": true, "is_frozen": false, "connected_team_ids": [], "internal_team_ids_count": 1, "internal_team_ids_sample_team": "TPQ67R81F", "pending_connected_team_ids": [], "is_pending_ext_shared": false}], "next_cursor": "aWQ6Mw==", "total_count": 14823}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_set_conversation_prefs() {
    let req = AdminConversationsSetConversationPrefsRequest::new(
        "C1234",
        "{'who_can_post':'type:admin,user:U1234'}",
    );
    {
        let (_server, client) = setup(
            "admin.conversations.setConversationPrefs",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .admin_conversations_set_conversation_prefs(&req)
            .await
            .expect("admin.conversations.setConversationPrefs");
        assert_preserved(
            "admin.conversations.setConversationPrefs",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_set_custom_retention() {
    let req = AdminConversationsSetCustomRetentionRequest::new("C12345678", 1);
    {
        let (_server, client) =
            setup("admin.conversations.setCustomRetention", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_set_custom_retention(&req)
            .await
            .expect("admin.conversations.setCustomRetention");
        assert_preserved(
            "admin.conversations.setCustomRetention",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_set_teams() {
    let req = AdminConversationsSetTeamsRequest::new("x")
        .team_id("x")
        .target_team_ids(vec!["A1".to_string(), "A2".to_string()])
        .org_channel(true);
    {
        let (_server, client) = setup("admin.conversations.setTeams", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_set_teams(&req)
            .await
            .expect("admin.conversations.setTeams");
        assert_preserved("admin.conversations.setTeams", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_unarchive() {
    let req = AdminConversationsUnarchiveRequest::new("C12345");
    {
        let (_server, client) = setup("admin.conversations.unarchive", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_unarchive(&req)
            .await
            .expect("admin.conversations.unarchive");
        assert_preserved("admin.conversations.unarchive", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_conversations_unlink_objects() {
    let req = AdminConversationsUnlinkObjectsRequest::new("x", "x");
    {
        let (_server, client) = setup("admin.conversations.unlinkObjects", r#"{"ok": true}"#).await;
        let res = client
            .admin_conversations_unlink_objects(&req)
            .await
            .expect("admin.conversations.unlinkObjects");
        assert_preserved("admin.conversations.unlinkObjects", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_emoji_add() {
    let req = AdminEmojiAddRequest::new("x").url("x");
    {
        let (_server, client) = setup("admin.emoji.add", r#"{"ok": true}"#).await;
        let res = client.admin_emoji_add(&req).await.expect("admin.emoji.add");
        assert_preserved("admin.emoji.add", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_emoji_add_alias() {
    let req = AdminEmojiAddAliasRequest::new("x", "x");
    {
        let (_server, client) = setup("admin.emoji.addAlias", r#"{"ok": true}"#).await;
        let res = client
            .admin_emoji_add_alias(&req)
            .await
            .expect("admin.emoji.addAlias");
        assert_preserved("admin.emoji.addAlias", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_emoji_list() {
    let req = AdminEmojiListRequest::new().cursor("5c3e53d5").limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminEmojiListResponse::default().next_cursor(), None);
    let page = AdminEmojiListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.emoji.list", r#"{"ok": true, "emoji": {"workflow": {"url": "https://emoji.slack-edge.com/TM315QLU8/workflow/530de66adccc59c5.png", "date_created": 1591720632, "uploaded_by": "WLWLQDAL9"}, "welcome": {"url": "https://emoji.slack-edge.com/TM315QLU8/welcome/763d3659699d2ef7.gif", "date_created": 1593383451, "uploaded_by": "WPU7MCTFH"}, "person": {"url": "https://emoji.slack-edge.com/TM315QLU8/person/81295a4f69d8b122.png", "date_created": 1593383817, "uploaded_by": "WPU7MCTFH"}, "people": {"url": "https://emoji.slack-edge.com/TM315QLU8/people/0b40796ab677b47f.png", "date_created": 1593383822, "uploaded_by": "WPU7MCTFH"}, "slackbot": {"url": "https://emoji.slack-edge.com/TM315QLU8/slackbot/561d6e545263d92b.png", "date_created": 1593383989, "uploaded_by": "WPU7MCTFH"}, "plus1": {"url": "https://emoji.slack-edge.com/TM315QLU8/plus1/42b92e57a79eb27e.png", "date_created": 1593724572, "uploaded_by": "WPU7MCTFH"}, "bc": {"url": "https://emoji.slack-edge.com/TM315QLU8/bc/fb3dfdea697528b9.png", "date_created": 1594854289, "uploaded_by": "WPU7MCTFH"}, "wf": {"url": "https://emoji.slack-edge.com/TM315QLU8/wf/04dad3aa28b57cd3.png", "date_created": 1594854443, "uploaded_by": "WPU7MCTFH"}, "kb": {"url": "https://emoji.slack-edge.com/TM315QLU8/kb/bab417c375703f7b.png", "date_created": 1598467537, "uploaded_by": "WPU7MCTFH"}, "ignore": {"url": "https://emoji.slack-edge.com/TM315QLU8/ignore/9506cda43addbad8.png", "date_created": 1598467835, "uploaded_by": "WPU7MCTFH"}}, "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .admin_emoji_list(&req)
            .await
            .expect("admin.emoji.list");
        assert_preserved(
            "admin.emoji.list",
            r#"{"ok": true, "emoji": {"workflow": {"url": "https://emoji.slack-edge.com/TM315QLU8/workflow/530de66adccc59c5.png", "date_created": 1591720632, "uploaded_by": "WLWLQDAL9"}, "welcome": {"url": "https://emoji.slack-edge.com/TM315QLU8/welcome/763d3659699d2ef7.gif", "date_created": 1593383451, "uploaded_by": "WPU7MCTFH"}, "person": {"url": "https://emoji.slack-edge.com/TM315QLU8/person/81295a4f69d8b122.png", "date_created": 1593383817, "uploaded_by": "WPU7MCTFH"}, "people": {"url": "https://emoji.slack-edge.com/TM315QLU8/people/0b40796ab677b47f.png", "date_created": 1593383822, "uploaded_by": "WPU7MCTFH"}, "slackbot": {"url": "https://emoji.slack-edge.com/TM315QLU8/slackbot/561d6e545263d92b.png", "date_created": 1593383989, "uploaded_by": "WPU7MCTFH"}, "plus1": {"url": "https://emoji.slack-edge.com/TM315QLU8/plus1/42b92e57a79eb27e.png", "date_created": 1593724572, "uploaded_by": "WPU7MCTFH"}, "bc": {"url": "https://emoji.slack-edge.com/TM315QLU8/bc/fb3dfdea697528b9.png", "date_created": 1594854289, "uploaded_by": "WPU7MCTFH"}, "wf": {"url": "https://emoji.slack-edge.com/TM315QLU8/wf/04dad3aa28b57cd3.png", "date_created": 1594854443, "uploaded_by": "WPU7MCTFH"}, "kb": {"url": "https://emoji.slack-edge.com/TM315QLU8/kb/bab417c375703f7b.png", "date_created": 1598467537, "uploaded_by": "WPU7MCTFH"}, "ignore": {"url": "https://emoji.slack-edge.com/TM315QLU8/ignore/9506cda43addbad8.png", "date_created": 1598467835, "uploaded_by": "WPU7MCTFH"}}, "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_emoji_remove() {
    let req = AdminEmojiRemoveRequest::new("x");
    {
        let (_server, client) = setup("admin.emoji.remove", r#"{"ok": true}"#).await;
        let res = client
            .admin_emoji_remove(&req)
            .await
            .expect("admin.emoji.remove");
        assert_preserved("admin.emoji.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_emoji_rename() {
    let req = AdminEmojiRenameRequest::new("x", "x");
    {
        let (_server, client) = setup("admin.emoji.rename", r#"{"ok": true}"#).await;
        let res = client
            .admin_emoji_rename(&req)
            .await
            .expect("admin.emoji.rename");
        assert_preserved("admin.emoji.rename", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_functions_list() {
    let req = AdminFunctionsListRequest::new(vec!["A1".to_string(), "A2".to_string()])
        .team_id("T00000001")
        .include_non_distributed_functions(true)
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminFunctionsListResponse::default().next_cursor(), None);
    let page = AdminFunctionsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.functions.list", r#"{"ok": true, "functions": [{"id": "123ABC456DE", "callback_id": "sample_function", "title": "Sample function", "description": "A sample function.", "type": "app", "input_parameters": [{"type": "string", "name": "message", "description": "Message to be posted.", "title": "Message", "is_required": true}, {"type": "slack#/reference/objects/user-object_id", "name": "user", "description": "The user invoking the workflow.", "title": "User", "is_required": false}], "output_parameters": [{"type": "string", "name": "updatedMsg", "description": "Updated message to be posted.", "title": "Updated Msg", "is_required": true}], "app_id": "789FGH1011IJ", "date_created": 1692283027, "date_updated": 1692725035, "date_deleted": 0}], "response_metadata": {"next_cursor": "aWQ6MTE3MDk1NTIzNDAxOQ=="}}"#).await;
        let res = client
            .admin_functions_list(&req)
            .await
            .expect("admin.functions.list");
        assert_preserved(
            "admin.functions.list",
            r#"{"ok": true, "functions": [{"id": "123ABC456DE", "callback_id": "sample_function", "title": "Sample function", "description": "A sample function.", "type": "app", "input_parameters": [{"type": "string", "name": "message", "description": "Message to be posted.", "title": "Message", "is_required": true}, {"type": "slack#/reference/objects/user-object_id", "name": "user", "description": "The user invoking the workflow.", "title": "User", "is_required": false}], "output_parameters": [{"type": "string", "name": "updatedMsg", "description": "Updated message to be posted.", "title": "Updated Msg", "is_required": true}], "app_id": "789FGH1011IJ", "date_created": 1692283027, "date_updated": 1692725035, "date_deleted": 0}], "response_metadata": {"next_cursor": "aWQ6MTE3MDk1NTIzNDAxOQ=="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_functions_permissions_lookup() {
    let req = AdminFunctionsPermissionsLookupRequest::new(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup(
            "admin.functions.permissions.lookup",
            r#"{"ok": true, "errors": {}}"#,
        )
        .await;
        let res = client
            .admin_functions_permissions_lookup(&req)
            .await
            .expect("admin.functions.permissions.lookup");
        assert_preserved(
            "admin.functions.permissions.lookup",
            r#"{"ok": true, "errors": {}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_functions_permissions_set() {
    let req = AdminFunctionsPermissionsSetRequest::new("x")
        .visibility("x")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .permissions(vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) = setup("admin.functions.permissions.set", r#"{"ok": true}"#).await;
        let res = client
            .admin_functions_permissions_set(&req)
            .await
            .expect("admin.functions.permissions.set");
        assert_preserved("admin.functions.permissions.set", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_invite_requests_approve() {
    let req = AdminInviteRequestsApproveRequest::new("Ir1234").team_id("x");
    {
        let (_server, client) = setup("admin.inviteRequests.approve", r#"{"ok": true}"#).await;
        let res = client
            .admin_invite_requests_approve(&req)
            .await
            .expect("admin.inviteRequests.approve");
        assert_preserved("admin.inviteRequests.approve", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_invite_requests_approved_list() {
    let req = AdminInviteRequestsApprovedListRequest::new()
        .team_id("x")
        .cursor("5cweb43")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminInviteRequestsApprovedListResponse::default().next_cursor(),
        None
    );
    let page = AdminInviteRequestsApprovedListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) =
            setup("admin.inviteRequests.approved.list", r#"{"ok": true}"#).await;
        let res = client
            .admin_invite_requests_approved_list(&req)
            .await
            .expect("admin.inviteRequests.approved.list");
        assert_preserved(
            "admin.inviteRequests.approved.list",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_invite_requests_denied_list() {
    let req = AdminInviteRequestsDeniedListRequest::new()
        .team_id("T0U9RERW4")
        .cursor("5cweb43")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminInviteRequestsDeniedListResponse::default().next_cursor(),
        None
    );
    let page = AdminInviteRequestsDeniedListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.inviteRequests.denied.list", r#"{"ok": true, "denied_requests": [{"invite_request": {"id": "Ir0P0Z6SKD", "email": "example1@slack-corp.com", "date_created": 1569863763, "requester_ids": ["W0GBAEJKD"], "channel_ids": ["C0GU730DQ", "C0U9WES0Y"], "is_restricted": false, "is_ultra_restricted": false, "real_name": "Example", "date_expire": null, "request_reason": null}, "denied_by": {"actor_type": "app", "actor_id": "A00"}}, {"invite_request": {"id": "Ir0GL738RY", "email": "example2@slack-corp.com", "date_created": 1564016487, "requester_ids": ["W0GAAFJKD"], "channel_ids": ["G06LHQYCS", "G0BMCEY2W", "C0GLJ30DQ", "C0U9RQS0Y"], "is_restricted": false, "is_ultra_restricted": false, "real_name": "seesijkdswsss", "date_expire": null, "request_reason": null}, "denied_by": {"actor_type": "user", "actor_id": "W0GAAUJKD"}}], "response_metadata": {"next_cursor": "ZGF0ZV9jcmVhdGU6MTU2MTc0Nzc2Ng=="}}"#).await;
        let res = client
            .admin_invite_requests_denied_list(&req)
            .await
            .expect("admin.inviteRequests.denied.list");
        assert_preserved(
            "admin.inviteRequests.denied.list",
            r#"{"ok": true, "denied_requests": [{"invite_request": {"id": "Ir0P0Z6SKD", "email": "example1@slack-corp.com", "date_created": 1569863763, "requester_ids": ["W0GBAEJKD"], "channel_ids": ["C0GU730DQ", "C0U9WES0Y"], "is_restricted": false, "is_ultra_restricted": false, "real_name": "Example", "date_expire": null, "request_reason": null}, "denied_by": {"actor_type": "app", "actor_id": "A00"}}, {"invite_request": {"id": "Ir0GL738RY", "email": "example2@slack-corp.com", "date_created": 1564016487, "requester_ids": ["W0GAAFJKD"], "channel_ids": ["G06LHQYCS", "G0BMCEY2W", "C0GLJ30DQ", "C0U9RQS0Y"], "is_restricted": false, "is_ultra_restricted": false, "real_name": "seesijkdswsss", "date_expire": null, "request_reason": null}, "denied_by": {"actor_type": "user", "actor_id": "W0GAAUJKD"}}], "response_metadata": {"next_cursor": "ZGF0ZV9jcmVhdGU6MTU2MTc0Nzc2Ng=="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_invite_requests_deny() {
    let req = AdminInviteRequestsDenyRequest::new("Ir1234").team_id("x");
    {
        let (_server, client) = setup("admin.inviteRequests.deny", r#"{"ok": true}"#).await;
        let res = client
            .admin_invite_requests_deny(&req)
            .await
            .expect("admin.inviteRequests.deny");
        assert_preserved("admin.inviteRequests.deny", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_invite_requests_list() {
    let req = AdminInviteRequestsListRequest::new()
        .team_id("x")
        .cursor("5cweb43")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminInviteRequestsListResponse::default().next_cursor(),
        None
    );
    let page = AdminInviteRequestsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.inviteRequests.list", r#"{"ok": true, "invite_requests": [{"id": "Ir558ULC1J", "email": "foobar@example.com", "date_created": 1619700970, "requester_ids": ["U558ULC10"], "channel_ids": ["C558ULC19", "C558ULC0W"], "invite_type": "full_member", "real_name": null, "date_expire": null, "request_reason": null}], "response_metadata": {"next_cursor": "ZGF0ZV9jcmVhdGU6MTYxOTcwMDk3MA=="}}"#).await;
        let res = client
            .admin_invite_requests_list(&req)
            .await
            .expect("admin.inviteRequests.list");
        assert_preserved(
            "admin.inviteRequests.list",
            r#"{"ok": true, "invite_requests": [{"id": "Ir558ULC1J", "email": "foobar@example.com", "date_created": 1619700970, "requester_ids": ["U558ULC10"], "channel_ids": ["C558ULC19", "C558ULC0W"], "invite_type": "full_member", "real_name": null, "date_expire": null, "request_reason": null}], "response_metadata": {"next_cursor": "ZGF0ZV9jcmVhdGU6MTYxOTcwMDk3MA=="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_roles_add_assignments() {
    let req = AdminRolesAddAssignmentsRequest::new(
        "R0001",
        vec!["A1".to_string(), "A2".to_string()],
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.roles.addAssignments", r#"{"ok": true}"#).await;
        let res = client
            .admin_roles_add_assignments(&req)
            .await
            .expect("admin.roles.addAssignments");
        assert_preserved("admin.roles.addAssignments", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_roles_list_assignments() {
    let req = AdminRolesListAssignmentsRequest::new()
        .cursor("5c3e53d5")
        .limit(1)
        .role_ids(vec!["A1".to_string(), "A2".to_string()])
        .entity_ids(vec!["A1".to_string(), "A2".to_string()])
        .sort_dir("DESC");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AdminRolesListAssignmentsResponse::default().next_cursor(),
        None
    );
    let page = AdminRolesListAssignmentsResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.roles.listAssignments", r#"{"ok": true, "role_assignments": [{"role_id": "Rl0C", "entity_id": "T123ABC456", "user_id": "U123ABC456", "date_create": 1677038902}, {"role_id": "Rl0C", "entity_id": "T123ABC456", "user_id": "U123ABC456", "date_create": 1677038902}, {"role_id": "Rl0A", "entity_id": "C123ABC456", "user_id": "U123ABC456", "date_create": 1666624374}, {"role_id": "Rl03", "entity_id": "E123ABC456", "user_id": "U123ABC456", "date_create": 1663617026}, {"role_id": "Rl01", "entity_id": "E123ABC456", "user_id": "U123ABC456", "date_create": 1643231331}], "response_metadata": {"next_cursor": "dXNlcl9pZDozMDA1NjIwNTc0MjYyO2VudGl0eV90eXBlOjE7ZW50aXR5X2lkOjMwMDUxODcyMTczNjY7ZGF0ZV9jcmVhdGU6MTY0MzIzMTMzMQ=="}}"#).await;
        let res = client
            .admin_roles_list_assignments(&req)
            .await
            .expect("admin.roles.listAssignments");
        assert_preserved(
            "admin.roles.listAssignments",
            r#"{"ok": true, "role_assignments": [{"role_id": "Rl0C", "entity_id": "T123ABC456", "user_id": "U123ABC456", "date_create": 1677038902}, {"role_id": "Rl0C", "entity_id": "T123ABC456", "user_id": "U123ABC456", "date_create": 1677038902}, {"role_id": "Rl0A", "entity_id": "C123ABC456", "user_id": "U123ABC456", "date_create": 1666624374}, {"role_id": "Rl03", "entity_id": "E123ABC456", "user_id": "U123ABC456", "date_create": 1663617026}, {"role_id": "Rl01", "entity_id": "E123ABC456", "user_id": "U123ABC456", "date_create": 1643231331}], "response_metadata": {"next_cursor": "dXNlcl9pZDozMDA1NjIwNTc0MjYyO2VudGl0eV90eXBlOjE7ZW50aXR5X2lkOjMwMDUxODcyMTczNjY7ZGF0ZV9jcmVhdGU6MTY0MzIzMTMzMQ=="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_roles_remove_assignments() {
    let req = AdminRolesRemoveAssignmentsRequest::new(
        "R0001",
        vec!["A1".to_string(), "A2".to_string()],
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.roles.removeAssignments", r#"{"ok": true}"#).await;
        let res = client
            .admin_roles_remove_assignments(&req)
            .await
            .expect("admin.roles.removeAssignments");
        assert_preserved("admin.roles.removeAssignments", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_admins_list() {
    let req = AdminTeamsAdminsListRequest::new("x")
        .limit(1)
        .cursor("dXNlcjpVMEc5V0ZYTlo=");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminTeamsAdminsListResponse::default().next_cursor(), None);
    let page = AdminTeamsAdminsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup(
            "admin.teams.admins.list",
            r#"{"ok": true, "admin_ids": ["U1234"]}"#,
        )
        .await;
        let res = client
            .admin_teams_admins_list(&req)
            .await
            .expect("admin.teams.admins.list");
        assert_preserved(
            "admin.teams.admins.list",
            r#"{"ok": true, "admin_ids": ["U1234"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_create() {
    let req = AdminTeamsCreateRequest::new("x", "x")
        .team_description("x")
        .team_discoverability("x");
    {
        let (_server, client) =
            setup("admin.teams.create", r#"{"ok": true, "team": "T12345"}"#).await;
        let res = client
            .admin_teams_create(&req)
            .await
            .expect("admin.teams.create");
        assert_preserved(
            "admin.teams.create",
            r#"{"ok": true, "team": "T12345"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_list() {
    let req = AdminTeamsListRequest::new().limit(1).cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminTeamsListResponse::default().next_cursor(), None);
    let page = AdminTeamsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.teams.list", r#"{"ok": true, "teams": [{"id": "T1234", "name": "My Team", "discoverability": "hidden", "primary_owner": {"user_id": "W1234", "email": "bront@slack.com"}, "team_url": "https://subarachnoid.slack.com/"}]}"#).await;
        let res = client
            .admin_teams_list(&req)
            .await
            .expect("admin.teams.list");
        assert_preserved(
            "admin.teams.list",
            r#"{"ok": true, "teams": [{"id": "T1234", "name": "My Team", "discoverability": "hidden", "primary_owner": {"user_id": "W1234", "email": "bront@slack.com"}, "team_url": "https://subarachnoid.slack.com/"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_owners_list() {
    let req = AdminTeamsOwnersListRequest::new("x")
        .limit(1)
        .cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminTeamsOwnersListResponse::default().next_cursor(), None);
    let page = AdminTeamsOwnersListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup(
            "admin.teams.owners.list",
            r#"{"ok": true, "owner_ids": ["U1234"]}"#,
        )
        .await;
        let res = client
            .admin_teams_owners_list(&req)
            .await
            .expect("admin.teams.owners.list");
        assert_preserved(
            "admin.teams.owners.list",
            r#"{"ok": true, "owner_ids": ["U1234"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_settings_info() {
    let req = AdminTeamsSettingsInfoRequest::new("x");
    {
        let (_server, client) = setup("admin.teams.settings.info", r#"{"ok": true, "team": {"id": "string", "name": "string", "domain": "string", "email_domain": "string", "icon": "array", "enterprise_id": "string", "enterprise_name": "string", "default_channels": "array"}}"#).await;
        let res = client
            .admin_teams_settings_info(&req)
            .await
            .expect("admin.teams.settings.info");
        assert_preserved(
            "admin.teams.settings.info",
            r#"{"ok": true, "team": {"id": "string", "name": "string", "domain": "string", "email_domain": "string", "icon": "array", "enterprise_id": "string", "enterprise_name": "string", "default_channels": "array"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_settings_set_default_channels() {
    let req = AdminTeamsSettingsSetDefaultChannelsRequest::new(
        "x",
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) =
            setup("admin.teams.settings.setDefaultChannels", r#"{"ok": true}"#).await;
        let res = client
            .admin_teams_settings_set_default_channels(&req)
            .await
            .expect("admin.teams.settings.setDefaultChannels");
        assert_preserved(
            "admin.teams.settings.setDefaultChannels",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_settings_set_description() {
    let req = AdminTeamsSettingsSetDescriptionRequest::new("x", "x");
    {
        let (_server, client) =
            setup("admin.teams.settings.setDescription", r#"{"ok": true}"#).await;
        let res = client
            .admin_teams_settings_set_description(&req)
            .await
            .expect("admin.teams.settings.setDescription");
        assert_preserved(
            "admin.teams.settings.setDescription",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_settings_set_discoverability() {
    let req = AdminTeamsSettingsSetDiscoverabilityRequest::new("x", "x");
    {
        let (_server, client) =
            setup("admin.teams.settings.setDiscoverability", r#"{"ok": true}"#).await;
        let res = client
            .admin_teams_settings_set_discoverability(&req)
            .await
            .expect("admin.teams.settings.setDiscoverability");
        assert_preserved(
            "admin.teams.settings.setDiscoverability",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_settings_set_icon() {
    let req = AdminTeamsSettingsSetIconRequest::new("http://mysite.com/icon.jpeg", "x");
    {
        let (_server, client) = setup("admin.teams.settings.setIcon", r#"{"ok": true}"#).await;
        let res = client
            .admin_teams_settings_set_icon(&req)
            .await
            .expect("admin.teams.settings.setIcon");
        assert_preserved("admin.teams.settings.setIcon", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_teams_settings_set_name() {
    let req = AdminTeamsSettingsSetNameRequest::new("x", "x");
    {
        let (_server, client) = setup("admin.teams.settings.setName", r#"{"ok": true}"#).await;
        let res = client
            .admin_teams_settings_set_name(&req)
            .await
            .expect("admin.teams.settings.setName");
        assert_preserved("admin.teams.settings.setName", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_add_channels() {
    let req = AdminUsergroupsAddChannelsRequest::new(
        "S00000000",
        vec!["A1".to_string(), "A2".to_string()],
    )
    .team_id("T00000000");
    {
        let (_server, client) = setup("admin.usergroups.addChannels", r#"{"ok": true}"#).await;
        let res = client
            .admin_usergroups_add_channels(&req)
            .await
            .expect("admin.usergroups.addChannels");
        assert_preserved("admin.usergroups.addChannels", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_add_teams() {
    let req =
        AdminUsergroupsAddTeamsRequest::new("S12345678", vec!["A1".to_string(), "A2".to_string()])
            .auto_provision(true);
    {
        let (_server, client) = setup("admin.usergroups.addTeams", r#"{"ok": true}"#).await;
        let res = client
            .admin_usergroups_add_teams(&req)
            .await
            .expect("admin.usergroups.addTeams");
        assert_preserved("admin.usergroups.addTeams", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_add_users() {
    let req =
        AdminUsergroupsAddUsersRequest::new("S0604QSJC", vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup(
            "admin.usergroups.addUsers",
            r#"{"ok": true, "invalid_users": [{"user_id": "U060RNRCZ", "reason": "guest_user"}]}"#,
        )
        .await;
        let res = client
            .admin_usergroups_add_users(&req)
            .await
            .expect("admin.usergroups.addUsers");
        assert_preserved(
            "admin.usergroups.addUsers",
            r#"{"ok": true, "invalid_users": [{"user_id": "U060RNRCZ", "reason": "guest_user"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_create() {
    let req = AdminUsergroupsCreateRequest::new("x")
        .handle("x")
        .purpose("x")
        .is_visible(true);
    {
        let (_server, client) = setup("admin.usergroups.create", r#"{"ok": true, "subteam": {"id": "S0604QSJC", "team_id": "E060RNRCZ", "enterprise_id": "E060RNRCZ", "is_subteam": true, "is_usergroup": true, "name": "Marketing Team", "description": "", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446746793, "date_delete": 0, "auto_type": null, "auto_provision": false, "enterprise_subteam_id": "", "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "is_section": false, "is_editing_restricted": true, "is_membership_locked": true, "is_idp_group": false, "is_visible": true, "is_org_level": true, "user_count": 0, "channel_count": 0}}"#).await;
        let res = client
            .admin_usergroups_create(&req)
            .await
            .expect("admin.usergroups.create");
        assert_preserved(
            "admin.usergroups.create",
            r#"{"ok": true, "subteam": {"id": "S0604QSJC", "team_id": "E060RNRCZ", "enterprise_id": "E060RNRCZ", "is_subteam": true, "is_usergroup": true, "name": "Marketing Team", "description": "", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446746793, "date_delete": 0, "auto_type": null, "auto_provision": false, "enterprise_subteam_id": "", "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "is_section": false, "is_editing_restricted": true, "is_membership_locked": true, "is_idp_group": false, "is_visible": true, "is_org_level": true, "user_count": 0, "channel_count": 0}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_fetch() {
    let req = AdminUsergroupsFetchRequest::new("S0604QSJC");
    {
        let (_server, client) = setup("admin.usergroups.fetch", r#"{"ok": true, "subteam": {"id": "S0604QSJC", "team_id": "E060RNRCZ", "enterprise_id": "E060RNRCZ", "is_subteam": true, "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446746793, "date_delete": 0, "auto_type": null, "auto_provision": false, "enterprise_subteam_id": "", "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "is_section": false, "is_editing_restricted": true, "is_membership_locked": true, "is_idp_group": false, "is_visible": true, "is_org_level": true, "teams": ["T060RNRCZ"], "user_count": 2, "channel_count": 1}}"#).await;
        let res = client
            .admin_usergroups_fetch(&req)
            .await
            .expect("admin.usergroups.fetch");
        assert_preserved(
            "admin.usergroups.fetch",
            r#"{"ok": true, "subteam": {"id": "S0604QSJC", "team_id": "E060RNRCZ", "enterprise_id": "E060RNRCZ", "is_subteam": true, "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446746793, "date_delete": 0, "auto_type": null, "auto_provision": false, "enterprise_subteam_id": "", "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "is_section": false, "is_editing_restricted": true, "is_membership_locked": true, "is_idp_group": false, "is_visible": true, "is_org_level": true, "teams": ["T060RNRCZ"], "user_count": 2, "channel_count": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_list_channels() {
    let req = AdminUsergroupsListChannelsRequest::new("S00000000")
        .team_id("T00000000")
        .include_num_members(true);
    {
        let (_server, client) = setup("admin.usergroups.listChannels", r#"{"ok": true, "channels": [{"id": "C024BE91L", "name": "fun", "team_id": "T024BE911", "num_members": 34}, {"id": "C024BE91K", "name": "more fun", "team_id": "T024BE912"}, {"id": "C024BE91M", "name": "public-channel", "team_id": "T024BE911", "is_redacted": true, "num_members": 34}, {"id": "C024BE91N", "name": "some more fun", "team_id": "T024BE921"}]}"#).await;
        let res = client
            .admin_usergroups_list_channels(&req)
            .await
            .expect("admin.usergroups.listChannels");
        assert_preserved(
            "admin.usergroups.listChannels",
            r#"{"ok": true, "channels": [{"id": "C024BE91L", "name": "fun", "team_id": "T024BE911", "num_members": 34}, {"id": "C024BE91K", "name": "more fun", "team_id": "T024BE912"}, {"id": "C024BE91M", "name": "public-channel", "team_id": "T024BE911", "is_redacted": true, "num_members": 34}, {"id": "C024BE91N", "name": "some more fun", "team_id": "T024BE921"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_remove_channels() {
    let req = AdminUsergroupsRemoveChannelsRequest::new(
        "S00000000",
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.usergroups.removeChannels", r#"{"ok": true}"#).await;
        let res = client
            .admin_usergroups_remove_channels(&req)
            .await
            .expect("admin.usergroups.removeChannels");
        assert_preserved("admin.usergroups.removeChannels", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_remove_teams() {
    let req = AdminUsergroupsRemoveTeamsRequest::new(
        "S12345678",
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.usergroups.removeTeams", r#"{"ok": true}"#).await;
        let res = client
            .admin_usergroups_remove_teams(&req)
            .await
            .expect("admin.usergroups.removeTeams");
        assert_preserved("admin.usergroups.removeTeams", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_remove_users() {
    let req = AdminUsergroupsRemoveUsersRequest::new(
        "S0604QSJC",
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.usergroups.removeUsers", r#"{"ok": true}"#).await;
        let res = client
            .admin_usergroups_remove_users(&req)
            .await
            .expect("admin.usergroups.removeUsers");
        assert_preserved("admin.usergroups.removeUsers", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_update() {
    let req = AdminUsergroupsUpdateRequest::new("S0604QSJC")
        .name("x")
        .handle("x")
        .description("x")
        .is_visible(true);
    {
        let (_server, client) = setup("admin.usergroups.update", r#"{"ok": true, "subteam": {"id": "S0604QSJC", "team_id": "E060RNRCZ", "enterprise_id": "E060RNRCZ", "is_subteam": true, "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446748371, "date_delete": 0, "auto_type": null, "auto_provision": false, "enterprise_subteam_id": "", "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "is_section": false, "is_editing_restricted": true, "is_membership_locked": true, "is_idp_group": false, "is_visible": true, "is_org_level": true, "teams": ["T060RNRCZ"], "user_count": 2, "channel_count": 1}}"#).await;
        let res = client
            .admin_usergroups_update(&req)
            .await
            .expect("admin.usergroups.update");
        assert_preserved(
            "admin.usergroups.update",
            r#"{"ok": true, "subteam": {"id": "S0604QSJC", "team_id": "E060RNRCZ", "enterprise_id": "E060RNRCZ", "is_subteam": true, "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446748371, "date_delete": 0, "auto_type": null, "auto_provision": false, "enterprise_subteam_id": "", "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "is_section": false, "is_editing_restricted": true, "is_membership_locked": true, "is_idp_group": false, "is_visible": true, "is_org_level": true, "teams": ["T060RNRCZ"], "user_count": 2, "channel_count": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_usergroups_upload_users() {
    let req = AdminUsergroupsUploadUsersRequest::new("S0604QSJC").file("x");
    {
        let (_server, client) = setup("admin.usergroups.uploadUsers", r#"{"ok": true, "successful_user_count": 2, "invalid_users": [{"user_id": "U0G9QF9C6", "reason": "guest_user"}]}"#).await;
        let res = client
            .admin_usergroups_upload_users(&req)
            .await
            .expect("admin.usergroups.uploadUsers");
        assert_preserved(
            "admin.usergroups.uploadUsers",
            r#"{"ok": true, "successful_user_count": 2, "invalid_users": [{"user_id": "U0G9QF9C6", "reason": "guest_user"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_assign() {
    let req = AdminUsersAssignRequest::new("x", "x")
        .is_restricted(true)
        .is_ultra_restricted(true)
        .channel_ids("C123,C3456");
    {
        let (_server, client) = setup("admin.users.assign", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_assign(&req)
            .await
            .expect("admin.users.assign");
        assert_preserved("admin.users.assign", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_get_expiration() {
    let req = AdminUsersGetExpirationRequest::new("U123ABC456").target_team("T123ABC456");
    {
        let (_server, client) = setup("admin.users.getExpiration", r#"{"ok": true, "user": {"id": "U123ABC456", "email": "deactivate_user2@email.com", "is_restricted": false, "is_ultra_restricted": true, "expiration_ts": 0}}"#).await;
        let res = client
            .admin_users_get_expiration(&req)
            .await
            .expect("admin.users.getExpiration");
        assert_preserved(
            "admin.users.getExpiration",
            r#"{"ok": true, "user": {"id": "U123ABC456", "email": "deactivate_user2@email.com", "is_restricted": false, "is_ultra_restricted": true, "expiration_ts": 0}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_invite() {
    let req = AdminUsersInviteRequest::new("x", "joe@email.com", "C1A2B3C4D,C26Z25Y24")
        .custom_message("Come and join our team!")
        .real_name("{\"full_name\":\"Joe Smith\"}")
        .resend(true)
        .is_restricted(true)
        .is_ultra_restricted(true)
        .guest_expiration_ts("0123456789.012345")
        .email_password_policy_enabled(true);
    {
        let (_server, client) = setup("admin.users.invite", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_invite(&req)
            .await
            .expect("admin.users.invite");
        assert_preserved("admin.users.invite", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_list() {
    let req = AdminUsersListRequest::new()
        .team_id("x")
        .cursor("5c3e53d5")
        .is_active(true)
        .include_deactivated_user_workspaces(true)
        .only_guests(true)
        .include_admins(true)
        .include_owners(true)
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminUsersListResponse::default().next_cursor(), None);
    let page = AdminUsersListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.users.list", r#"{"ok": true, "users": [{"id": "W0L3P31SP", "email": "john.doe@slack.com", "is_admin": false, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "username": "john_doe", "full_name": "John Doe", "is_active": true, "date_created": 1566922090, "deactivated_ts": 1678435283, "expiration_ts": 0, "workspaces": ["T123"], "has_2fa": false, "has_sso": false}]}"#).await;
        let res = client
            .admin_users_list(&req)
            .await
            .expect("admin.users.list");
        assert_preserved(
            "admin.users.list",
            r#"{"ok": true, "users": [{"id": "W0L3P31SP", "email": "john.doe@slack.com", "is_admin": false, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "username": "john_doe", "full_name": "John Doe", "is_active": true, "date_created": 1566922090, "deactivated_ts": 1678435283, "expiration_ts": 0, "workspaces": ["T123"], "has_2fa": false, "has_sso": false}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_remove() {
    let req = AdminUsersRemoveRequest::new("x", "W12345678");
    {
        let (_server, client) = setup("admin.users.remove", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_remove(&req)
            .await
            .expect("admin.users.remove");
        assert_preserved("admin.users.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_session_clear_settings() {
    let req = AdminUsersSessionClearSettingsRequest::new(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.users.session.clearSettings", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_session_clear_settings(&req)
            .await
            .expect("admin.users.session.clearSettings");
        assert_preserved("admin.users.session.clearSettings", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_session_get_settings() {
    let req = AdminUsersSessionGetSettingsRequest::new(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.users.session.getSettings", r#"{"ok": true, "session_settings": [{"user_id": "U1234", "desktop_app_browser_quit": true, "duration": 315569520}], "no_settings_applied": []}"#).await;
        let res = client
            .admin_users_session_get_settings(&req)
            .await
            .expect("admin.users.session.getSettings");
        assert_preserved(
            "admin.users.session.getSettings",
            r#"{"ok": true, "session_settings": [{"user_id": "U1234", "desktop_app_browser_quit": true, "duration": 315569520}], "no_settings_applied": []}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_session_invalidate() {
    let req = AdminUsersSessionInvalidateRequest::new("U12345", 1);
    {
        let (_server, client) = setup("admin.users.session.invalidate", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_session_invalidate(&req)
            .await
            .expect("admin.users.session.invalidate");
        assert_preserved("admin.users.session.invalidate", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_session_list() {
    let req = AdminUsersSessionListRequest::new()
        .team_id("T1234")
        .user_id("U1234")
        .limit(1)
        .cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminUsersSessionListResponse::default().next_cursor(), None);
    let page = AdminUsersSessionListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.users.session.list", r#"{"ok": true, "active_sessions": [{"user_id": "U012S9M77JP", "team_id": "E011E2SBBFC", "session_id": 1112275520242, "recent": {"device_hardware": "Intel", "os": "OS X", "os_version": "10.15.7", "slack_client_version": "91.0.4472.77", "ip": "24.6.145.138"}, "created": {"device_hardware": "Intel", "os": "OS X", "os_version": "10.15.7", "slack_client_version": "91.0.4472.77", "ip": "24.6.145.138"}}]}"#).await;
        let res = client
            .admin_users_session_list(&req)
            .await
            .expect("admin.users.session.list");
        assert_preserved(
            "admin.users.session.list",
            r#"{"ok": true, "active_sessions": [{"user_id": "U012S9M77JP", "team_id": "E011E2SBBFC", "session_id": 1112275520242, "recent": {"device_hardware": "Intel", "os": "OS X", "os_version": "10.15.7", "slack_client_version": "91.0.4472.77", "ip": "24.6.145.138"}, "created": {"device_hardware": "Intel", "os": "OS X", "os_version": "10.15.7", "slack_client_version": "91.0.4472.77", "ip": "24.6.145.138"}}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_session_reset() {
    let req = AdminUsersSessionResetRequest::new("W12345678")
        .mobile_only(true)
        .web_only(true);
    {
        let (_server, client) = setup("admin.users.session.reset", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_session_reset(&req)
            .await
            .expect("admin.users.session.reset");
        assert_preserved("admin.users.session.reset", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_session_reset_bulk() {
    let req = AdminUsersSessionResetBulkRequest::new(vec!["A1".to_string(), "A2".to_string()])
        .mobile_only(true)
        .web_only(true);
    {
        let (_server, client) = setup("admin.users.session.resetBulk", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_session_reset_bulk(&req)
            .await
            .expect("admin.users.session.resetBulk");
        assert_preserved("admin.users.session.resetBulk", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_session_set_settings() {
    let req = AdminUsersSessionSetSettingsRequest::new(vec!["A1".to_string(), "A2".to_string()])
        .duration(1)
        .desktop_app_browser_quit(true);
    {
        let (_server, client) = setup("admin.users.session.setSettings", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_session_set_settings(&req)
            .await
            .expect("admin.users.session.setSettings");
        assert_preserved("admin.users.session.setSettings", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_set_admin() {
    let req = AdminUsersSetAdminRequest::new("T12345678", "W12345678");
    {
        let (_server, client) = setup("admin.users.setAdmin", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_set_admin(&req)
            .await
            .expect("admin.users.setAdmin");
        assert_preserved("admin.users.setAdmin", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_set_expiration() {
    let req = AdminUsersSetExpirationRequest::new("W12345678", 1).team_id("x");
    {
        let (_server, client) = setup("admin.users.setExpiration", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_set_expiration(&req)
            .await
            .expect("admin.users.setExpiration");
        assert_preserved("admin.users.setExpiration", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_set_owner() {
    let req = AdminUsersSetOwnerRequest::new("T12345678", "W12345678");
    {
        let (_server, client) = setup("admin.users.setOwner", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_set_owner(&req)
            .await
            .expect("admin.users.setOwner");
        assert_preserved("admin.users.setOwner", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_set_regular() {
    let req = AdminUsersSetRegularRequest::new("T12345678", "W12345678");
    {
        let (_server, client) = setup("admin.users.setRegular", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_set_regular(&req)
            .await
            .expect("admin.users.setRegular");
        assert_preserved("admin.users.setRegular", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_users_unsupported_versions_export() {
    let req = AdminUsersUnsupportedVersionsExportRequest::new()
        .date_sessions_started(1)
        .date_end_of_support(1);
    {
        let (_server, client) =
            setup("admin.users.unsupportedVersions.export", r#"{"ok": true}"#).await;
        let res = client
            .admin_users_unsupported_versions_export(&req)
            .await
            .expect("admin.users.unsupportedVersions.export");
        assert_preserved(
            "admin.users.unsupportedVersions.export",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_workflows_collaborators_add() {
    let req = AdminWorkflowsCollaboratorsAddRequest::new(
        vec!["A1".to_string(), "A2".to_string()],
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("admin.workflows.collaborators.add", r#"{"ok": true}"#).await;
        let res = client
            .admin_workflows_collaborators_add(&req)
            .await
            .expect("admin.workflows.collaborators.add");
        assert_preserved("admin.workflows.collaborators.add", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_workflows_collaborators_remove() {
    let req = AdminWorkflowsCollaboratorsRemoveRequest::new(
        vec!["A1".to_string(), "A2".to_string()],
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) =
            setup("admin.workflows.collaborators.remove", r#"{"ok": true}"#).await;
        let res = client
            .admin_workflows_collaborators_remove(&req)
            .await
            .expect("admin.workflows.collaborators.remove");
        assert_preserved(
            "admin.workflows.collaborators.remove",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_workflows_permissions_lookup() {
    let req = AdminWorkflowsPermissionsLookupRequest::new(vec!["A1".to_string(), "A2".to_string()])
        .max_workflow_triggers(1);
    {
        let (_server, client) =
            setup("admin.workflows.permissions.lookup", r#"{"ok": true}"#).await;
        let res = client
            .admin_workflows_permissions_lookup(&req)
            .await
            .expect("admin.workflows.permissions.lookup");
        assert_preserved(
            "admin.workflows.permissions.lookup",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_workflows_search() {
    let req = AdminWorkflowsSearchRequest::new()
        .query("Time off")
        .app_id("A12345")
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1)
        .no_collaborators(true)
        .collaborator_ids(vec!["A1".to_string(), "A2".to_string()])
        .num_trigger_ids(1)
        .is_sales_elevate(true)
        .source("x")
        .sort("x")
        .sort_dir("x")
        .trigger_type_id("x")
        .publish_status("x")
        .step_function_ids(vec!["A1".to_string(), "A2".to_string()]);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AdminWorkflowsSearchResponse::default().next_cursor(), None);
    let page = AdminWorkflowsSearchResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("admin.workflows.search", r#"{"ok": true, "total_found": 2, "workflows": [{"id": "Wf014FQ97ZT5", "workflow_function_id": "Fn014EPW7SBU", "callback_id": "untitled_workflow", "title": "Hello there", "description": "A brand new workflow", "input_parameters": {"Ft014FQ980RZ__user_id": {"type": "slack#/reference/objects/user-object_id", "name": "Ft014FQ980RZ__user_id", "description": "User who reacted to the message", "title": "User who reacted to the message", "is_required": false}, "Ft014FQ980RZ__message_context": {"type": "slack#/types/message_context", "name": "Ft014FQ980RZ__message_context", "description": "Reference to the message that was reacted to", "title": "Reference to the message that was reacted to", "is_required": true}}, "steps": [{"id": "a1468ed7-82a2-4d3a-8598-d67194a10148", "function_id": "Fn010P", "inputs": {"message": {"value": [{"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"text": "Hello ", "type": "text"}, {"id": "{{inputs.Ft014FQ980RZ__user_id}}", "type": "workflowtoken", "property": "", "data_type": "slack#/reference/objects/user-object_id"}]}]}], "locked": false}, "message_context": {"value": "{{inputs.Ft014FQ980RZ__message_context}}", "locked": false}, "reply_broadcast": {"value": "false", "locked": false}}}], "collaborators": ["U014FM2DQF5"], "icons": {"image_96": "https://slack-pantry.dev.slack.com/11d89af/img/apps/workflows_96.png", "image_192": "https://slack-pantry.dev.slack.com/11d89af/img/apps/workflows_192.png"}, "is_published": true, "last_updated_by": "U014FM2DQF5", "unpublished_change_count": 0, "app_id": "A014EPW7S3U", "source": "workflow_builder", "billing_type": "simple", "date_updated": 1715661162, "is_billable": false, "creation_source_type": 1, "creation_source_id": "Wt06DWT50W57", "last_published_version_id": "Wfv014FQ9NHCK", "last_published_date": "1674675746", "trigger_ids": ["Ft050QQ638NS"], "is_sales_home_workflow": false, "is_sales_elevate": false, "trigger_types": [{"id": "Ftt0102", "type": "event", "subtype": "slack#/events/reaction_added"}]}, {"id": "Wf014HH0GN9G", "workflow_function_id": "Fn014HH0GN82", "callback_id": "give_kudos_workflow", "title": "Give kudos", "description": "Acknowledge the impact someone had on you", "input_parameters": {"interactivity": {"type": "slack#/types/interactivity", "name": "interactivity", "title": "Interactivity", "is_required": true}}, "steps": [{"id": "0", "function_id": "Fn010N", "inputs": {"title": {"value": "Give someone kudos", "locked": false}, "fields": {"value": {"elements": [{"name": "doer_of_good_deeds", "type": "slack#/reference/objects/user-object_id", "title": "Whose deeds are deemed worthy of a kudo?", "description": "Recognizing such deeds is dazzlingly desirable of you!"}, {"name": "kudo_channel", "type": "slack#/reference/objects/channel-object_id", "title": "Where should this message be shared?"}, {"long": true, "name": "kudo_message", "type": "string", "title": "What would you like to say?"}, {"enum": ["Appreciation for someone 🫂", "Celebrating a victory 🏆", "Thankful for great teamwork ⚽️", "Amazed at awesome work ☄️", "Excited for the future 🎉", "No vibes, just plants 🪴"], "name": "kudo_vibe", "type": "string", "title": "What is this kudo's \"vibe\"?", "description": "What sorts of energy is given off?"}], "required": ["doer_of_good_deeds", "kudo_channel", "kudo_message"]}, "locked": false}, "description": {"value": "Continue the positive energy through your written word", "locked": false}, "submit_label": {"value": "Share", "locked": false}, "interactivity": {"value": "{{inputs.interactivity}}", "locked": false}}}, {"id": "1", "function_id": "Fn014JHDUP3M", "inputs": {"vibe": {"value": "{{steps.0.fields.kudo_vibe}}", "locked": false}}}, {"id": "2", "function_id": "Fn0102", "inputs": {"message": {"value": "*Hey <@{{steps.0.fields.doer_of_good_deeds}}>!* Someone wanted to share some kind words with you :otter:\n> {{steps.0.fields.kudo_message}}\n{{steps.1.URL}}", "locked": false}, "channel_id": {"value": "{{steps.0.fields.kudo_channel}}", "locked": false}}}], "collaborators": ["U014ELP4Z9Q"], "icons": {"image_96": "https://example.com/avatars/2023-03-01/1153578567186_dd65b1ee58919c5d8665_96.png", "image_192": "https://example.com/avatars/2023-03-01/1153578567186_dd65b1ee58919c5d8665_192.png"}, "is_published": true, "last_updated_by": "U014ELP4Z9Q", "unpublished_change_count": 0, "app_id": "A014HH0GN7L", "source": "app", "billing_type": "complex", "date_updated": 1715661162, "is_billable": true, "creation_source_type": 1, "creation_source_id": "Wt06DWT50W57", "last_published_version_id": "Wfv014JHDUP4K", "last_published_date": "1677682235", "trigger_ids": ["Ft0481M8V85R"], "is_sales_home_workflow": false, "is_sales_elevate": false, "trigger_types": [{"id": "Ftt0101", "type": "shortcut"}]}], "response_metadata": {"next_cursor": "aWQ6MTE1MzU3ODU2NjMyMg=="}}"#).await;
        let res = client
            .admin_workflows_search(&req)
            .await
            .expect("admin.workflows.search");
        assert_preserved(
            "admin.workflows.search",
            r#"{"ok": true, "total_found": 2, "workflows": [{"id": "Wf014FQ97ZT5", "workflow_function_id": "Fn014EPW7SBU", "callback_id": "untitled_workflow", "title": "Hello there", "description": "A brand new workflow", "input_parameters": {"Ft014FQ980RZ__user_id": {"type": "slack#/reference/objects/user-object_id", "name": "Ft014FQ980RZ__user_id", "description": "User who reacted to the message", "title": "User who reacted to the message", "is_required": false}, "Ft014FQ980RZ__message_context": {"type": "slack#/types/message_context", "name": "Ft014FQ980RZ__message_context", "description": "Reference to the message that was reacted to", "title": "Reference to the message that was reacted to", "is_required": true}}, "steps": [{"id": "a1468ed7-82a2-4d3a-8598-d67194a10148", "function_id": "Fn010P", "inputs": {"message": {"value": [{"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"text": "Hello ", "type": "text"}, {"id": "{{inputs.Ft014FQ980RZ__user_id}}", "type": "workflowtoken", "property": "", "data_type": "slack#/reference/objects/user-object_id"}]}]}], "locked": false}, "message_context": {"value": "{{inputs.Ft014FQ980RZ__message_context}}", "locked": false}, "reply_broadcast": {"value": "false", "locked": false}}}], "collaborators": ["U014FM2DQF5"], "icons": {"image_96": "https://slack-pantry.dev.slack.com/11d89af/img/apps/workflows_96.png", "image_192": "https://slack-pantry.dev.slack.com/11d89af/img/apps/workflows_192.png"}, "is_published": true, "last_updated_by": "U014FM2DQF5", "unpublished_change_count": 0, "app_id": "A014EPW7S3U", "source": "workflow_builder", "billing_type": "simple", "date_updated": 1715661162, "is_billable": false, "creation_source_type": 1, "creation_source_id": "Wt06DWT50W57", "last_published_version_id": "Wfv014FQ9NHCK", "last_published_date": "1674675746", "trigger_ids": ["Ft050QQ638NS"], "is_sales_home_workflow": false, "is_sales_elevate": false, "trigger_types": [{"id": "Ftt0102", "type": "event", "subtype": "slack#/events/reaction_added"}]}, {"id": "Wf014HH0GN9G", "workflow_function_id": "Fn014HH0GN82", "callback_id": "give_kudos_workflow", "title": "Give kudos", "description": "Acknowledge the impact someone had on you", "input_parameters": {"interactivity": {"type": "slack#/types/interactivity", "name": "interactivity", "title": "Interactivity", "is_required": true}}, "steps": [{"id": "0", "function_id": "Fn010N", "inputs": {"title": {"value": "Give someone kudos", "locked": false}, "fields": {"value": {"elements": [{"name": "doer_of_good_deeds", "type": "slack#/reference/objects/user-object_id", "title": "Whose deeds are deemed worthy of a kudo?", "description": "Recognizing such deeds is dazzlingly desirable of you!"}, {"name": "kudo_channel", "type": "slack#/reference/objects/channel-object_id", "title": "Where should this message be shared?"}, {"long": true, "name": "kudo_message", "type": "string", "title": "What would you like to say?"}, {"enum": ["Appreciation for someone 🫂", "Celebrating a victory 🏆", "Thankful for great teamwork ⚽️", "Amazed at awesome work ☄️", "Excited for the future 🎉", "No vibes, just plants 🪴"], "name": "kudo_vibe", "type": "string", "title": "What is this kudo's \"vibe\"?", "description": "What sorts of energy is given off?"}], "required": ["doer_of_good_deeds", "kudo_channel", "kudo_message"]}, "locked": false}, "description": {"value": "Continue the positive energy through your written word", "locked": false}, "submit_label": {"value": "Share", "locked": false}, "interactivity": {"value": "{{inputs.interactivity}}", "locked": false}}}, {"id": "1", "function_id": "Fn014JHDUP3M", "inputs": {"vibe": {"value": "{{steps.0.fields.kudo_vibe}}", "locked": false}}}, {"id": "2", "function_id": "Fn0102", "inputs": {"message": {"value": "*Hey <@{{steps.0.fields.doer_of_good_deeds}}>!* Someone wanted to share some kind words with you :otter:\n> {{steps.0.fields.kudo_message}}\n{{steps.1.URL}}", "locked": false}, "channel_id": {"value": "{{steps.0.fields.kudo_channel}}", "locked": false}}}], "collaborators": ["U014ELP4Z9Q"], "icons": {"image_96": "https://example.com/avatars/2023-03-01/1153578567186_dd65b1ee58919c5d8665_96.png", "image_192": "https://example.com/avatars/2023-03-01/1153578567186_dd65b1ee58919c5d8665_192.png"}, "is_published": true, "last_updated_by": "U014ELP4Z9Q", "unpublished_change_count": 0, "app_id": "A014HH0GN7L", "source": "app", "billing_type": "complex", "date_updated": 1715661162, "is_billable": true, "creation_source_type": 1, "creation_source_id": "Wt06DWT50W57", "last_published_version_id": "Wfv014JHDUP4K", "last_published_date": "1677682235", "trigger_ids": ["Ft0481M8V85R"], "is_sales_home_workflow": false, "is_sales_elevate": false, "trigger_types": [{"id": "Ftt0101", "type": "shortcut"}]}], "response_metadata": {"next_cursor": "aWQ6MTE1MzU3ODU2NjMyMg=="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_workflows_triggers_types_permissions_lookup() {
    let req = AdminWorkflowsTriggersTypesPermissionsLookupRequest::new(vec![
        "A1".to_string(),
        "A2".to_string(),
    ]);
    {
        let (_server, client) = setup(
            "admin.workflows.triggers.types.permissions.lookup",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .admin_workflows_triggers_types_permissions_lookup(&req)
            .await
            .expect("admin.workflows.triggers.types.permissions.lookup");
        assert_preserved(
            "admin.workflows.triggers.types.permissions.lookup",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_workflows_triggers_types_permissions_set() {
    let req = AdminWorkflowsTriggersTypesPermissionsSetRequest::new("['FTT01', 'FTT02', 'FTT03']")
        .visibility("x")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .permissions(serde_json::json!({"k": "v"}));
    {
        let (_server, client) = setup(
            "admin.workflows.triggers.types.permissions.set",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .admin_workflows_triggers_types_permissions_set(&req)
            .await
            .expect("admin.workflows.triggers.types.permissions.set");
        assert_preserved(
            "admin.workflows.triggers.types.permissions.set",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn admin_workflows_unpublish() {
    let req = AdminWorkflowsUnpublishRequest::new(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("admin.workflows.unpublish", r#"{"ok": true}"#).await;
        let res = client
            .admin_workflows_unpublish(&req)
            .await
            .expect("admin.workflows.unpublish");
        assert_preserved("admin.workflows.unpublish", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn agents_sessions_rename() {
    let req = AgentsSessionsRenameRequest::new("x")
        .channel_id("x")
        .thread_ts("1234567890.123456");
    {
        let (_server, client) = setup(
            "agents.sessions.rename",
            r#"{"ok": true, "title": "Bora Bora trip prep"}"#,
        )
        .await;
        let res = client
            .agents_sessions_rename(&req)
            .await
            .expect("agents.sessions.rename");
        assert_preserved(
            "agents.sessions.rename",
            r#"{"ok": true, "title": "Bora Bora trip prep"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn agents_sessions_set_status() {
    let req = AgentsSessionsSetStatusRequest::new("x")
        .channel_id("x")
        .thread_ts("1234567890.123456")
        .title("x")
        .initiator_user_id("x")
        .icon_emoji("x")
        .icon_url("x")
        .username("x");
    {
        let (_server, client) = setup("agents.sessions.setStatus", r#"{"ok": true, "status": "processing", "agent_status": "processing", "title": "Scuba diving research"}"#).await;
        let res = client
            .agents_sessions_set_status(&req)
            .await
            .expect("agents.sessions.setStatus");
        assert_preserved(
            "agents.sessions.setStatus",
            r#"{"ok": true, "status": "processing", "agent_status": "processing", "title": "Scuba diving research"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn api_test() {
    let req = ApiTestRequest::new().error("my_error");
    {
        let (_server, client) = setup("api.test", r#"{"ok": true}"#).await;
        let res = client.api_test(&req).await.expect("api.test");
        assert_preserved("api.test", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("api.test", r#"{"ok": true, "args": {"foo": "bar"}}"#).await;
        let res = client.api_test(&req).await.expect("api.test");
        assert_preserved("api.test", r#"{"ok": true, "args": {"foo": "bar"}}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_activities_list() {
    let req = AppsActivitiesListRequest::new("A12345")
        .team_id("T12345")
        .cursor("bG9nX2lkOjc5NjQ1NA==")
        .limit(1)
        .min_log_level("info")
        .log_event_type("test_log_event")
        .source("slack")
        .component_type("workflows")
        .component_id("Wf013SMGL4V9")
        .trace_id("Tr432f2")
        .min_date_created(1)
        .max_date_created(1)
        .sort_direction("asc");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AppsActivitiesListResponse::default().next_cursor(), None);
    let page = AppsActivitiesListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("apps.activities.list", r#"{"ok": true, "activities": [{"level": "info", "event_type": "function_execution_started", "source": "slack", "component_type": "functions", "component_id": "Fn123", "payload": {"function_name": "Reverse", "function_type": "app"}, "created": 1650463798824317, "trace_id": "Tr123"}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .apps_activities_list(&req)
            .await
            .expect("apps.activities.list");
        assert_preserved(
            "apps.activities.list",
            r#"{"ok": true, "activities": [{"level": "info", "event_type": "function_execution_started", "source": "slack", "component_type": "functions", "component_id": "Fn123", "payload": {"function_name": "Reverse", "function_type": "app"}, "created": 1650463798824317, "trace_id": "Tr123"}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_auth_external_delete() {
    let req = AppsAuthExternalDeleteRequest::new()
        .app_id("A12345")
        .provider_key("x")
        .external_token_id("x");
    {
        let (_server, client) = setup("apps.auth.external.delete", r#"{"ok": true}"#).await;
        let res = client
            .apps_auth_external_delete(&req)
            .await
            .expect("apps.auth.external.delete");
        assert_preserved("apps.auth.external.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_auth_external_get() {
    let req = AppsAuthExternalGetRequest::new("Et12345ABCDE").force_refresh(true);
    {
        let (_server, client) = setup(
            "apps.auth.external.get",
            r#"{"ok": true, "external_token": "00D-EXAMPLE-TOKEN"}"#,
        )
        .await;
        let res = client
            .apps_auth_external_get(&req)
            .await
            .expect("apps.auth.external.get");
        assert_preserved(
            "apps.auth.external.get",
            r#"{"ok": true, "external_token": "00D-EXAMPLE-TOKEN"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_connections_open() {
    let req = AppsConnectionsOpenRequest::new();
    {
        let (_server, client) = setup("apps.connections.open", r#"{"ok": true, "url": "wss://wss-somethiing.slack.com/link/?ticket=12348&app_id=5678"}"#).await;
        let res = client
            .apps_connections_open(&req)
            .await
            .expect("apps.connections.open");
        assert_preserved(
            "apps.connections.open",
            r#"{"ok": true, "url": "wss://wss-somethiing.slack.com/link/?ticket=12348&app_id=5678"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_bulk_delete() {
    let req = AppsDatastoreBulkDeleteRequest::new("x", vec!["A1".to_string(), "A2".to_string()])
        .app_id("x");
    {
        let (_server, client) = setup(
            "apps.datastore.bulkDelete",
            r#"{"ok": true, "failed_items": []}"#,
        )
        .await;
        let res = client
            .apps_datastore_bulk_delete(&req)
            .await
            .expect("apps.datastore.bulkDelete");
        assert_preserved(
            "apps.datastore.bulkDelete",
            r#"{"ok": true, "failed_items": []}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_bulk_get() {
    let req =
        AppsDatastoreBulkGetRequest::new("x", vec!["A1".to_string(), "A2".to_string()]).app_id("x");
    {
        let (_server, client) = setup("apps.datastore.bulkGet", r#"{"ok": true, "datastore": "delicious_meals", "items": [{"id": "12462", "meal": "Shawarma"}]}"#).await;
        let res = client
            .apps_datastore_bulk_get(&req)
            .await
            .expect("apps.datastore.bulkGet");
        assert_preserved(
            "apps.datastore.bulkGet",
            r#"{"ok": true, "datastore": "delicious_meals", "items": [{"id": "12462", "meal": "Shawarma"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_bulk_put() {
    let req =
        AppsDatastoreBulkPutRequest::new("x", vec![serde_json::json!({"k": "v"})]).app_id("x");
    {
        let (_server, client) = setup(
            "apps.datastore.bulkPut",
            r#"{"ok": true, "datastore": "delicious_meals", "failed_items": []}"#,
        )
        .await;
        let res = client
            .apps_datastore_bulk_put(&req)
            .await
            .expect("apps.datastore.bulkPut");
        assert_preserved(
            "apps.datastore.bulkPut",
            r#"{"ok": true, "datastore": "delicious_meals", "failed_items": []}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_count() {
    let req = AppsDatastoreCountRequest::new("x")
        .expression("#artist = :artist_name")
        .expression_attributes(serde_json::json!({"k": "v"}))
        .expression_values(serde_json::json!({"k": "v"}))
        .app_id("x");
    {
        let (_server, client) = setup(
            "apps.datastore.count",
            r#"{"ok": true, "datastore": "good_tunes", "count": 2}"#,
        )
        .await;
        let res = client
            .apps_datastore_count(&req)
            .await
            .expect("apps.datastore.count");
        assert_preserved(
            "apps.datastore.count",
            r#"{"ok": true, "datastore": "good_tunes", "count": 2}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_delete() {
    let req = AppsDatastoreDeleteRequest::new("x", "x").app_id("x");
    {
        let (_server, client) = setup("apps.datastore.delete", r#"{"ok": true}"#).await;
        let res = client
            .apps_datastore_delete(&req)
            .await
            .expect("apps.datastore.delete");
        assert_preserved("apps.datastore.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_get() {
    let req = AppsDatastoreGetRequest::new("x", "x").app_id("x");
    {
        let (_server, client) = setup("apps.datastore.get", r#"{"ok": true, "datastore": "good_tunes", "item": {"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}}"#).await;
        let res = client
            .apps_datastore_get(&req)
            .await
            .expect("apps.datastore.get");
        assert_preserved(
            "apps.datastore.get",
            r#"{"ok": true, "datastore": "good_tunes", "item": {"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_put() {
    let req = AppsDatastorePutRequest::new("x", serde_json::json!({"k": "v"})).app_id("x");
    {
        let (_server, client) = setup("apps.datastore.put", r#"{"ok": true, "datastore": "good_tunes", "item": {"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}}"#).await;
        let res = client
            .apps_datastore_put(&req)
            .await
            .expect("apps.datastore.put");
        assert_preserved(
            "apps.datastore.put",
            r#"{"ok": true, "datastore": "good_tunes", "item": {"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_query() {
    let req = AppsDatastoreQueryRequest::new("x")
        .expression("#artist = :artist_name")
        .expression_attributes(serde_json::json!({"k": "v"}))
        .expression_values(serde_json::json!({"k": "v"}))
        .app_id("x")
        .cursor("5c3e53d5")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AppsDatastoreQueryResponse::default().next_cursor(), None);
    let page = AppsDatastoreQueryResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("apps.datastore.query", r#"{"ok": true, "datastore": "good_tunes", "items": [{"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}, {"artist": "Fred Rogers", "song": "Won't You Be My Neighbor?", "id": "5"}]}"#).await;
        let res = client
            .apps_datastore_query(&req)
            .await
            .expect("apps.datastore.query");
        assert_preserved(
            "apps.datastore.query",
            r#"{"ok": true, "datastore": "good_tunes", "items": [{"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}, {"artist": "Fred Rogers", "song": "Won't You Be My Neighbor?", "id": "5"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_datastore_update() {
    let req = AppsDatastoreUpdateRequest::new("x", serde_json::json!({"k": "v"})).app_id("x");
    {
        let (_server, client) = setup("apps.datastore.update", r#"{"ok": true, "datastore": "good_tunes", "item": {"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}}"#).await;
        let res = client
            .apps_datastore_update(&req)
            .await
            .expect("apps.datastore.update");
        assert_preserved(
            "apps.datastore.update",
            r#"{"ok": true, "datastore": "good_tunes", "item": {"artist": "Whitney Houston", "song": "I Will Always Love You", "id": "4"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_event_authorizations_list() {
    let req = AppsEventAuthorizationsListRequest::new("x")
        .cursor("x")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AppsEventAuthorizationsListResponse::default().next_cursor(),
        None
    );
    let page = AppsEventAuthorizationsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("apps.event.authorizations.list", r#"{"ok": true, "authorizations": [{"enterprise_id": "string", "team_id": "string", "user_id": "string", "is_bot": "string"}, {"enterprise_id": "string2", "team_id": "string2", "user_id": "string2", "is_bot": "string2"}]}"#).await;
        let res = client
            .apps_event_authorizations_list(&req)
            .await
            .expect("apps.event.authorizations.list");
        assert_preserved(
            "apps.event.authorizations.list",
            r#"{"ok": true, "authorizations": [{"enterprise_id": "string", "team_id": "string", "user_id": "string", "is_bot": "string"}, {"enterprise_id": "string2", "team_id": "string2", "user_id": "string2", "is_bot": "string2"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_icon_set() {
    let req = AppsIconSetRequest::new("x").file("x").url("x");
    {
        let (_server, client) = setup("apps.icon.set", r#"{"ok": true}"#).await;
        let res = client.apps_icon_set(&req).await.expect("apps.icon.set");
        assert_preserved("apps.icon.set", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_managed_permissions_set() {
    let req = AppsManagedPermissionsSetRequest::new("x", "everyone");
    {
        let (_server, client) = setup("apps.managed.permissions.set", r#"{"ok": true}"#).await;
        let res = client
            .apps_managed_permissions_set(&req)
            .await
            .expect("apps.managed.permissions.set");
        assert_preserved("apps.managed.permissions.set", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_manifest_create() {
    let req = AppsManifestCreateRequest::new("x").team_id("x");
    {
        let (_server, client) = setup("apps.manifest.create", r#"{"ok": true, "app_id": "A012ABCD0A0", "credentials": {"client_id": "...", "client_secret": "...", "verification_token": "...", "signing_secret": "..."}, "oauth_authorize_url": "https://slack.com/oauth/v2/authorize?client_id=...&scope=commands,workflow.steps:execute"}"#).await;
        let res = client
            .apps_manifest_create(&req)
            .await
            .expect("apps.manifest.create");
        assert_preserved(
            "apps.manifest.create",
            r#"{"ok": true, "app_id": "A012ABCD0A0", "credentials": {"client_id": "...", "client_secret": "...", "verification_token": "...", "signing_secret": "..."}, "oauth_authorize_url": "https://slack.com/oauth/v2/authorize?client_id=...&scope=commands,workflow.steps:execute"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_manifest_delete() {
    let req = AppsManifestDeleteRequest::new("x");
    {
        let (_server, client) = setup("apps.manifest.delete", r#"{"ok": true}"#).await;
        let res = client
            .apps_manifest_delete(&req)
            .await
            .expect("apps.manifest.delete");
        assert_preserved("apps.manifest.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_manifest_export() {
    let req = AppsManifestExportRequest::new("x");
    {
        let (_server, client) = setup("apps.manifest.export", r##"{"ok": true, "manifest": {"_metadata": {"major_version": 1, "minor_version": 1}, "display_information": {"name": "Zork", "description": "You are likely to be eaten by a grue.", "background_color": "#0000AA", "long_description": "Play the Infocom classic text adventure and find your way to the end of the maze. ZORK is a game of adventure, danger, and low cunning. In it you will explore some of the most amazing territory ever seen by mortals. No workspace should be without one!"}, "features": {"app_home": {"home_tab_enabled": true, "messages_tab_enabled": false, "messages_tab_read_only_enabled": false}, "bot_user": {"display_name": "zork", "always_online": true}, "slash_commands": [{"command": "/zork", "description": "You are standing in an open field west of a white house, with a boarded front door. There is a small mailbox here.", "usage_hint": "/zork open mailbox", "should_escape": false}], "workflow_steps": [{"name": "Example step", "callback_id": "tutorial_example_step"}]}, "oauth_config": {"redirect_urls": ["https://example.com/slack/auth"], "scopes": {"bot": ["commands", "workflow.steps:execute"]}}, "settings": {"event_subscriptions": {"bot_events": ["workflow_step_execute"]}, "interactivity": {"is_enabled": true}, "org_deploy_enabled": false, "socket_mode_enabled": true, "is_hosted": false, "token_rotation_enabled": false}}}"##).await;
        let res = client
            .apps_manifest_export(&req)
            .await
            .expect("apps.manifest.export");
        assert_preserved(
            "apps.manifest.export",
            r##"{"ok": true, "manifest": {"_metadata": {"major_version": 1, "minor_version": 1}, "display_information": {"name": "Zork", "description": "You are likely to be eaten by a grue.", "background_color": "#0000AA", "long_description": "Play the Infocom classic text adventure and find your way to the end of the maze. ZORK is a game of adventure, danger, and low cunning. In it you will explore some of the most amazing territory ever seen by mortals. No workspace should be without one!"}, "features": {"app_home": {"home_tab_enabled": true, "messages_tab_enabled": false, "messages_tab_read_only_enabled": false}, "bot_user": {"display_name": "zork", "always_online": true}, "slash_commands": [{"command": "/zork", "description": "You are standing in an open field west of a white house, with a boarded front door. There is a small mailbox here.", "usage_hint": "/zork open mailbox", "should_escape": false}], "workflow_steps": [{"name": "Example step", "callback_id": "tutorial_example_step"}]}, "oauth_config": {"redirect_urls": ["https://example.com/slack/auth"], "scopes": {"bot": ["commands", "workflow.steps:execute"]}}, "settings": {"event_subscriptions": {"bot_events": ["workflow_step_execute"]}, "interactivity": {"is_enabled": true}, "org_deploy_enabled": false, "socket_mode_enabled": true, "is_hosted": false, "token_rotation_enabled": false}}}"##,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_manifest_update() {
    let req = AppsManifestUpdateRequest::new("x", "x");
    {
        let (_server, client) = setup(
            "apps.manifest.update",
            r#"{"ok": true, "app_id": "A012ABCD0A0", "permissions_updated": false}"#,
        )
        .await;
        let res = client
            .apps_manifest_update(&req)
            .await
            .expect("apps.manifest.update");
        assert_preserved(
            "apps.manifest.update",
            r#"{"ok": true, "app_id": "A012ABCD0A0", "permissions_updated": false}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_manifest_validate() {
    let req = AppsManifestValidateRequest::new("x").app_id("x");
    {
        let (_server, client) =
            setup("apps.manifest.validate", r#"{"ok": true, "errors": []}"#).await;
        let res = client
            .apps_manifest_validate(&req)
            .await
            .expect("apps.manifest.validate");
        assert_preserved(
            "apps.manifest.validate",
            r#"{"ok": true, "errors": []}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_uninstall() {
    let req = AppsUninstallRequest::new(
        "56579136444.26251006572",
        "f25b5ceaf8a3c2a2c4f52bb4f0b0499e",
    );
    {
        let (_server, client) = setup("apps.uninstall", r#"{"ok": true}"#).await;
        let res = client.apps_uninstall(&req).await.expect("apps.uninstall");
        assert_preserved("apps.uninstall", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn apps_user_connection_update() {
    let req = AppsUserConnectionUpdateRequest::new("U12345678", "connected");
    {
        let (_server, client) = setup("apps.user.connection.update", r#"{"ok": true}"#).await;
        let res = client
            .apps_user_connection_update(&req)
            .await
            .expect("apps.user.connection.update");
        assert_preserved("apps.user.connection.update", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn assistant_search_context() {
    let req = AssistantSearchContextRequest::new("What is project gizmo?")
        .action_token("12345.98765.abcd2358fdea")
        .channel_types(vec!["A1".to_string(), "A2".to_string()])
        .content_types(vec!["A1".to_string(), "A2".to_string()])
        .include_bots(true)
        .include_deleted_users(true)
        .before(1)
        .after(1)
        .include_context_messages(true)
        .context_channel_id("x")
        .cursor("asf91j9jfd")
        .limit(1)
        .sort("x")
        .sort_dir("x")
        .include_message_blocks(true)
        .highlight(true)
        .term_clauses(vec![serde_json::json!({"k": "v"})])
        .modifiers("has:pin before:yesterday")
        .include_archived_channels(true)
        .disable_semantic_search(true);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        AssistantSearchContextResponse::default().next_cursor(),
        None
    );
    let page = AssistantSearchContextResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("assistant.search.context", r#"{"ok": true, "results": {"messages": [{"author_name": "Jennifer Hynes", "author_user_id": "U0123456", "team_id": "T0123456", "channel_id": "C0123456", "channel_name": "proj-gizmo", "message_ts": "123456.7890", "content": "Hey team, we'll be kicking off our mobile UX revamp for the Gizmo App...", "is_author_bot": false, "permalink": "https://mycompany.slack.com/archives/C012345ABC/p123456789", "blocks": [{"type": "rich_text", "block_id": "0c2PW", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Hey team, we'll be kicking off our mobile UX revamp for the Gizmo App..."}]}]}], "context_messages": {"before": [{"text": "What are we discussing in today's sync?", "user_id:": "U098765", "ts": "123456.7777", "blocks": [{"type": "rich_text", "block_id": "0c5KQ", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "What are we discussing in today's sync?"}]}]}]}], "after": [{"text": "Woohoo! Exciting news!", "user_id:": "U555930", "ts": "123456.9999", "blocks": [{"type": "rich_text", "block_id": "0P6G5", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Woohoo! Exciting news!"}]}]}]}]}}], "files": [{"uploader_user_id": "U0123456", "author_user_id": "U0123456", "author_name": "Jennifer Hynes", "team_id": "T0123456", "file_id": "F0123456", "date_created": 1733260762, "date_updated": 1733260763, "title": "Project tracker", "file_type": "application/vnd.slack-list", "permalink": "https://mycompany.slack.com/lists/T0123456/F0123456", "content": "Project tracker"}], "channels": [{"team_id": "T0123456", "creator_user_id": "U0123456", "creator_name": "Jennifer Hynes", "date_created": 1746570052, "date_updated": 1746570052, "name": "project-gizmo", "topic": "Launch date: Q4 2025", "purpose": "Discuss project-related topics on the new Gizmo app update", "permalink": "https://slack.com/archives/C123456"}]}, "response_metadata": {"next_cursor": "Q1VSUkVOVF9QQUdFOjI="}}"#).await;
        let res = client
            .assistant_search_context(&req)
            .await
            .expect("assistant.search.context");
        assert_preserved(
            "assistant.search.context",
            r#"{"ok": true, "results": {"messages": [{"author_name": "Jennifer Hynes", "author_user_id": "U0123456", "team_id": "T0123456", "channel_id": "C0123456", "channel_name": "proj-gizmo", "message_ts": "123456.7890", "content": "Hey team, we'll be kicking off our mobile UX revamp for the Gizmo App...", "is_author_bot": false, "permalink": "https://mycompany.slack.com/archives/C012345ABC/p123456789", "blocks": [{"type": "rich_text", "block_id": "0c2PW", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Hey team, we'll be kicking off our mobile UX revamp for the Gizmo App..."}]}]}], "context_messages": {"before": [{"text": "What are we discussing in today's sync?", "user_id:": "U098765", "ts": "123456.7777", "blocks": [{"type": "rich_text", "block_id": "0c5KQ", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "What are we discussing in today's sync?"}]}]}]}], "after": [{"text": "Woohoo! Exciting news!", "user_id:": "U555930", "ts": "123456.9999", "blocks": [{"type": "rich_text", "block_id": "0P6G5", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Woohoo! Exciting news!"}]}]}]}]}}], "files": [{"uploader_user_id": "U0123456", "author_user_id": "U0123456", "author_name": "Jennifer Hynes", "team_id": "T0123456", "file_id": "F0123456", "date_created": 1733260762, "date_updated": 1733260763, "title": "Project tracker", "file_type": "application/vnd.slack-list", "permalink": "https://mycompany.slack.com/lists/T0123456/F0123456", "content": "Project tracker"}], "channels": [{"team_id": "T0123456", "creator_user_id": "U0123456", "creator_name": "Jennifer Hynes", "date_created": 1746570052, "date_updated": 1746570052, "name": "project-gizmo", "topic": "Launch date: Q4 2025", "purpose": "Discuss project-related topics on the new Gizmo app update", "permalink": "https://slack.com/archives/C123456"}]}, "response_metadata": {"next_cursor": "Q1VSUkVOVF9QQUdFOjI="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn assistant_search_info() {
    let req = AssistantSearchInfoRequest::new();
    {
        let (_server, client) = setup(
            "assistant.search.info",
            r#"{"ok": true, "is_ai_search_enabled": true}"#,
        )
        .await;
        let res = client
            .assistant_search_info(&req)
            .await
            .expect("assistant.search.info");
        assert_preserved(
            "assistant.search.info",
            r#"{"ok": true, "is_ai_search_enabled": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn assistant_threads_set_status() {
    let req = AssistantThreadsSetStatusRequest::new("x", "x", "x")
        .loading_messages(vec![serde_json::json!({"k": "v"})])
        .icon_emoji(":chart_with_upwards_trend:")
        .icon_url("http://lorempixel.com/48/48")
        .username("My Bot");
    {
        let (_server, client) = setup("assistant.threads.setStatus", r#"{"ok": true}"#).await;
        let res = client
            .assistant_threads_set_status(&req)
            .await
            .expect("assistant.threads.setStatus");
        assert_preserved("assistant.threads.setStatus", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn assistant_threads_set_suggested_prompts() {
    let req = AssistantThreadsSetSuggestedPromptsRequest::new("x", "x")
        .thread_ts("x")
        .title("x");
    {
        let (_server, client) =
            setup("assistant.threads.setSuggestedPrompts", r#"{"ok": true}"#).await;
        let res = client
            .assistant_threads_set_suggested_prompts(&req)
            .await
            .expect("assistant.threads.setSuggestedPrompts");
        assert_preserved(
            "assistant.threads.setSuggestedPrompts",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn assistant_threads_set_title() {
    let req = AssistantThreadsSetTitleRequest::new("x", "x", "x");
    {
        let (_server, client) = setup("assistant.threads.setTitle", r#"{"ok": true}"#).await;
        let res = client
            .assistant_threads_set_title(&req)
            .await
            .expect("assistant.threads.setTitle");
        assert_preserved("assistant.threads.setTitle", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn auth_revoke() {
    let req = AuthRevokeRequest::new().test(true);
    {
        let (_server, client) = setup("auth.revoke", r#"{"ok": true, "revoked": true}"#).await;
        let res = client.auth_revoke(&req).await.expect("auth.revoke");
        assert_preserved("auth.revoke", r#"{"ok": true, "revoked": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn auth_teams_list() {
    let req = AuthTeamsListRequest::new()
        .limit(1)
        .cursor("5c3e53d5")
        .include_icon(true);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(AuthTeamsListResponse::default().next_cursor(), None);
    let page = AuthTeamsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("auth.teams.list", r#"{"ok": true, "teams": [{"name": "Shinichi's workspace", "id": "T12345678"}, {"name": "Migi's workspace", "id": "T12345679"}], "response_metadata": {"next_cursor": "dXNlcl9pZDo5MTQyOTI5Mzkz"}}"#).await;
        let res = client.auth_teams_list(&req).await.expect("auth.teams.list");
        assert_preserved(
            "auth.teams.list",
            r#"{"ok": true, "teams": [{"name": "Shinichi's workspace", "id": "T12345678"}, {"name": "Migi's workspace", "id": "T12345679"}], "response_metadata": {"next_cursor": "dXNlcl9pZDo5MTQyOTI5Mzkz"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn auth_test() {
    let req = AuthTestRequest::new();
    {
        let (_server, client) = setup("auth.test", r#"{"ok": true, "url": "https://subarachnoid.slack.com/", "team": "Subarachnoid Workspace", "user": "grace", "team_id": "T12345678", "user_id": "W12345678"}"#).await;
        let res = client.auth_test(&req).await.expect("auth.test");
        assert_preserved(
            "auth.test",
            r#"{"ok": true, "url": "https://subarachnoid.slack.com/", "team": "Subarachnoid Workspace", "user": "grace", "team_id": "T12345678", "user_id": "W12345678"}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("auth.test", r#"{"ok": true, "url": "https://subarachnoid.slack.com/", "team": "Subarachnoid Workspace", "user": "bot", "team_id": "T0G9PQBBK", "user_id": "W23456789", "bot_id": "BZYBOTHED"}"#).await;
        let res = client.auth_test(&req).await.expect("auth.test");
        assert_preserved(
            "auth.test",
            r#"{"ok": true, "url": "https://subarachnoid.slack.com/", "team": "Subarachnoid Workspace", "user": "bot", "team_id": "T0G9PQBBK", "user_id": "W23456789", "bot_id": "BZYBOTHED"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn blocks_validate() {
    let req = BlocksValidateRequest::new()
        .blocks(vec![slack_web_api::blocks::Block::from(slack_web_api::blocks::DividerBlock::new())])
        .message("{\"blocks\": [{\"type\": \"section\", \"text\": {\"type\": \"plain_text\", \"text\": \"Hello world\"}}]}")
        .view(slack_web_api::blocks::View::modal(slack_web_api::blocks::TextObject::plain("t"), vec![]));
    {
        let (_server, client) = setup("blocks.validate", r#"{"ok": true}"#).await;
        let res = client.blocks_validate(&req).await.expect("blocks.validate");
        assert_preserved("blocks.validate", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn bookmarks_add() {
    let req = BookmarksAddRequest::new("x", "x")
        .channel_id("x")
        .link("x")
        .emoji("x")
        .entity_id("x")
        .access_level("x")
        .parent_id("x");
    {
        let (_server, client) = setup("bookmarks.add", r#"{"ok": true, "bookmark": {"id": "Bk033XFJ9BTJ", "channel_id": "C1RQ000", "title": "bookmark-1", "link": "https://google.com", "emoji": ":clap:", "icon_url": "https://www.google.com/favicon.ico", "type": "link", "entity_id": null, "date_created": 1644956055, "date_updated": 0, "rank": "g", "last_updated_by_user_id": "U0334B6G6G5", "last_updated_by_team_id": "T018DF03GHY", "shortcut_id": null, "app_id": null}}"#).await;
        let res = client.bookmarks_add(&req).await.expect("bookmarks.add");
        assert_preserved(
            "bookmarks.add",
            r#"{"ok": true, "bookmark": {"id": "Bk033XFJ9BTJ", "channel_id": "C1RQ000", "title": "bookmark-1", "link": "https://google.com", "emoji": ":clap:", "icon_url": "https://www.google.com/favicon.ico", "type": "link", "entity_id": null, "date_created": 1644956055, "date_updated": 0, "rank": "g", "last_updated_by_user_id": "U0334B6G6G5", "last_updated_by_team_id": "T018DF03GHY", "shortcut_id": null, "app_id": null}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn bookmarks_edit() {
    let req = BookmarksEditRequest::new()
        .channel_id("x")
        .bookmark_id("x")
        .title("x")
        .link("x")
        .emoji("x");
    {
        let (_server, client) = setup("bookmarks.edit", r#"{"ok": true, "bookmark": {"id": "Bk033XFJ9BTJ", "channel_id": "C1RQ000", "title": "bookmark-1", "link": "https://google.com", "emoji": ":clap:", "icon_url": "https://www.google.com/favicon.ico", "type": "link", "entity_id": null, "date_created": 1644956055, "date_updated": 0, "rank": "g", "last_updated_by_user_id": "U0334B6G6G5", "last_updated_by_team_id": "T018DF03GHY", "shortcut_id": null, "app_id": null}}"#).await;
        let res = client.bookmarks_edit(&req).await.expect("bookmarks.edit");
        assert_preserved(
            "bookmarks.edit",
            r#"{"ok": true, "bookmark": {"id": "Bk033XFJ9BTJ", "channel_id": "C1RQ000", "title": "bookmark-1", "link": "https://google.com", "emoji": ":clap:", "icon_url": "https://www.google.com/favicon.ico", "type": "link", "entity_id": null, "date_created": 1644956055, "date_updated": 0, "rank": "g", "last_updated_by_user_id": "U0334B6G6G5", "last_updated_by_team_id": "T018DF03GHY", "shortcut_id": null, "app_id": null}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn bookmarks_list() {
    let req = BookmarksListRequest::new().channel_id("x");
    {
        let (_server, client) = setup("bookmarks.list", r#"{"ok": true, "bookmarks": [{"id": "Bk123ABC4DEF", "channel_id": "C1RQ000", "title": "bookmark-1", "link": "https://google.com", "emoji": ":clap:", "icon_url": "https://www.google.com/favicon.ico", "type": "link", "entity_id": null, "date_created": 1644956055, "date_updated": 0, "rank": "g", "last_updated_by_user_id": "U0123A1B1C1", "last_updated_by_team_id": "T012AB34CDE", "shortcut_id": null, "app_id": null}]}"#).await;
        let res = client.bookmarks_list(&req).await.expect("bookmarks.list");
        assert_preserved(
            "bookmarks.list",
            r#"{"ok": true, "bookmarks": [{"id": "Bk123ABC4DEF", "channel_id": "C1RQ000", "title": "bookmark-1", "link": "https://google.com", "emoji": ":clap:", "icon_url": "https://www.google.com/favicon.ico", "type": "link", "entity_id": null, "date_created": 1644956055, "date_updated": 0, "rank": "g", "last_updated_by_user_id": "U0123A1B1C1", "last_updated_by_team_id": "T012AB34CDE", "shortcut_id": null, "app_id": null}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn bookmarks_remove() {
    let req = BookmarksRemoveRequest::new()
        .channel_id("x")
        .bookmark_id("x")
        .quip_section_id("x");
    {
        let (_server, client) = setup("bookmarks.remove", r#"{"ok": true}"#).await;
        let res = client
            .bookmarks_remove(&req)
            .await
            .expect("bookmarks.remove");
        assert_preserved("bookmarks.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn bots_info() {
    let req = BotsInfoRequest::new().bot("B12345678").team_id("x");
    {
        let (_server, client) = setup("bots.info", r#"{"ok": true, "bot": {"id": "B123456", "deleted": false, "name": "beforebot", "updated": 1449272004, "app_id": "A123456", "user_id": "U123456", "icons": {"image_36": "https://...", "image_48": "https://...", "image_72": "https://..."}}}"#).await;
        let res = client.bots_info(&req).await.expect("bots.info");
        assert_preserved(
            "bots.info",
            r#"{"ok": true, "bot": {"id": "B123456", "deleted": false, "name": "beforebot", "updated": 1449272004, "app_id": "A123456", "user_id": "U123456", "icons": {"image_36": "https://...", "image_48": "https://...", "image_72": "https://..."}}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn calls_add() {
    let req = CallsAddRequest::new(
        "025169F6-E37A-4E62-BB54-7F93A0FC4C1F",
        "https://example.com/calls/1234567890",
    )
    .external_display_id("705-292-868")
    .desktop_app_join_url("callapp://join/1234567890")
    .date_start(1)
    .title("Kimpossible sync up")
    .created_by("U1H77")
    .users(vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) = setup("calls.add", r#"{"ok": true, "call": {"id": "R0E69JAIF", "date_start": 1562002086, "external_unique_id": "025169F6-E37A-4E62-BB54-7F93A0FC4C1F", "join_url": "https://example.com/calls/1234567890", "desktop_app_join_url": "callapp://join/1234567890", "external_display_id": "705-292-868", "title": "Kimpossible sync up", "users": [{"slack_id": "U0MQG83FD"}, {"external_id": "54321678", "display_name": "Kim Possible", "avatar_url": "https://callmebeepme.com/users/avatar1234.jpg"}]}}"#).await;
        let res = client.calls_add(&req).await.expect("calls.add");
        assert_preserved(
            "calls.add",
            r#"{"ok": true, "call": {"id": "R0E69JAIF", "date_start": 1562002086, "external_unique_id": "025169F6-E37A-4E62-BB54-7F93A0FC4C1F", "join_url": "https://example.com/calls/1234567890", "desktop_app_join_url": "callapp://join/1234567890", "external_display_id": "705-292-868", "title": "Kimpossible sync up", "users": [{"slack_id": "U0MQG83FD"}, {"external_id": "54321678", "display_name": "Kim Possible", "avatar_url": "https://callmebeepme.com/users/avatar1234.jpg"}]}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn calls_end() {
    let req = CallsEndRequest::new("R0E69JAIF").duration(1);
    {
        let (_server, client) = setup("calls.end", r#"{"ok": true}"#).await;
        let res = client.calls_end(&req).await.expect("calls.end");
        assert_preserved("calls.end", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) =
            setup("calls.end", r#"{"ok": true, "error": "inactive_call"}"#).await;
        let res = client.calls_end(&req).await.expect("calls.end");
        assert_preserved(
            "calls.end",
            r#"{"ok": true, "error": "inactive_call"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn calls_info() {
    let req = CallsInfoRequest::new("R0E69JAIF");
    {
        let (_server, client) = setup("calls.info", r#"{"ok": true, "call": {"id": "R0E69JAIF", "date_start": 1562002086, "external_unique_id": "025169F6-E37A-4E62-BB54-7F93A0FC4C1F", "join_url": "https://callmebeepme.com/calls/1234567890", "desktop_app_join_url": "callapp://join/1234567890", "external_display_id": "705-292-868", "title": "Kimpossible sync up", "users": [{"slack_id": "U0MQG83FD"}, {"external_id": "54321678", "display_name": "Kim Possible", "avatar_url": "https://callmebeepme.com/users/avatar1234.jpg"}]}}"#).await;
        let res = client.calls_info(&req).await.expect("calls.info");
        assert_preserved(
            "calls.info",
            r#"{"ok": true, "call": {"id": "R0E69JAIF", "date_start": 1562002086, "external_unique_id": "025169F6-E37A-4E62-BB54-7F93A0FC4C1F", "join_url": "https://callmebeepme.com/calls/1234567890", "desktop_app_join_url": "callapp://join/1234567890", "external_display_id": "705-292-868", "title": "Kimpossible sync up", "users": [{"slack_id": "U0MQG83FD"}, {"external_id": "54321678", "display_name": "Kim Possible", "avatar_url": "https://callmebeepme.com/users/avatar1234.jpg"}]}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn calls_participants_add() {
    let req = CallsParticipantsAddRequest::new("R0E69JAIF", vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) = setup("calls.participants.add", r#"{"ok": true}"#).await;
        let res = client
            .calls_participants_add(&req)
            .await
            .expect("calls.participants.add");
        assert_preserved("calls.participants.add", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup(
            "calls.participants.add",
            r#"{"ok": true, "error": "bad_users"}"#,
        )
        .await;
        let res = client
            .calls_participants_add(&req)
            .await
            .expect("calls.participants.add");
        assert_preserved(
            "calls.participants.add",
            r#"{"ok": true, "error": "bad_users"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn calls_participants_remove() {
    let req = CallsParticipantsRemoveRequest::new("R0E69JAIF", vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) = setup("calls.participants.remove", r#"{"ok": true}"#).await;
        let res = client
            .calls_participants_remove(&req)
            .await
            .expect("calls.participants.remove");
        assert_preserved("calls.participants.remove", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup(
            "calls.participants.remove",
            r#"{"ok": true, "error": "bad_users"}"#,
        )
        .await;
        let res = client
            .calls_participants_remove(&req)
            .await
            .expect("calls.participants.remove");
        assert_preserved(
            "calls.participants.remove",
            r#"{"ok": true, "error": "bad_users"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn calls_update() {
    let req = CallsUpdateRequest::new("R0E69JAIF")
        .title("Kimpossible sync up call")
        .join_url("https://example.com/calls/0987654321")
        .desktop_app_join_url("callapp://join/0987654321");
    {
        let (_server, client) = setup("calls.update", r#"{"ok": true, "call": {"id": "R0E69JAIF", "date_start": 1562002086, "external_unique_id": "025169F6-E37A-4E62-BB54-7F93A0FC4C1F", "join_url": "https://callmebeepme.com/calls/0987654321", "desktop_app_join_url": "callapp://join/0987654321", "external_display_id": "705-292-868", "title": "Kimpossible sync up", "users": [{"slack_id": "U0MQG83FD"}, {"external_id": "54321678", "display_name": "Kim Possible", "avatar_url": "https://callmebeepme.com/users/avatar1234.jpg"}]}}"#).await;
        let res = client.calls_update(&req).await.expect("calls.update");
        assert_preserved(
            "calls.update",
            r#"{"ok": true, "call": {"id": "R0E69JAIF", "date_start": 1562002086, "external_unique_id": "025169F6-E37A-4E62-BB54-7F93A0FC4C1F", "join_url": "https://callmebeepme.com/calls/0987654321", "desktop_app_join_url": "callapp://join/0987654321", "external_display_id": "705-292-868", "title": "Kimpossible sync up", "users": [{"slack_id": "U0MQG83FD"}, {"external_id": "54321678", "display_name": "Kim Possible", "avatar_url": "https://callmebeepme.com/users/avatar1234.jpg"}]}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn canvases_access_delete() {
    let req = CanvasesAccessDeleteRequest::new("F1234ABCD")
        .channel_ids(vec!["A1".to_string(), "A2".to_string()])
        .user_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("canvases.access.delete", r#"{"ok": true}"#).await;
        let res = client
            .canvases_access_delete(&req)
            .await
            .expect("canvases.access.delete");
        assert_preserved("canvases.access.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn canvases_access_set() {
    let req = CanvasesAccessSetRequest::new("F1234ABCD", "x")
        .channel_ids(vec!["A1".to_string(), "A2".to_string()])
        .user_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("canvases.access.set", r#"{"ok": true}"#).await;
        let res = client
            .canvases_access_set(&req)
            .await
            .expect("canvases.access.set");
        assert_preserved("canvases.access.set", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn canvases_create() {
    let req = CanvasesCreateRequest::new()
        .title("Your Brilliant Title")
        .document_content("{\"type\": \"markdown\", \"markdown\": \"> standalone canvas!\"}")
        .channel_id("x");
    {
        let (_server, client) = setup(
            "canvases.create",
            r#"{"ok": true, "canvas_id": "F1234ABCD"}"#,
        )
        .await;
        let res = client.canvases_create(&req).await.expect("canvases.create");
        assert_preserved(
            "canvases.create",
            r#"{"ok": true, "canvas_id": "F1234ABCD"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn canvases_delete() {
    let req = CanvasesDeleteRequest::new("F1234ABCD");
    {
        let (_server, client) = setup("canvases.delete", r#"{"ok": true}"#).await;
        let res = client.canvases_delete(&req).await.expect("canvases.delete");
        assert_preserved("canvases.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn canvases_edit() {
    let req = CanvasesEditRequest::new("F1234ABCD", vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) = setup("canvases.edit", r#"{"ok": true}"#).await;
        let res = client.canvases_edit(&req).await.expect("canvases.edit");
        assert_preserved("canvases.edit", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn canvases_get_content() {
    let req = CanvasesGetContentRequest::new("F1234ABCD").content_type("x");
    {
        let (_server, client) = setup("canvases.getContent", r##"{"ok": true, "content": "# Project plan\n\n- [ ] Draft spec\n- [x] Kickoff meeting\n"}"##).await;
        let res = client
            .canvases_get_content(&req)
            .await
            .expect("canvases.getContent");
        assert_preserved(
            "canvases.getContent",
            r##"{"ok": true, "content": "# Project plan\n\n- [ ] Draft spec\n- [x] Kickoff meeting\n"}"##,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn canvases_sections_lookup() {
    let req = CanvasesSectionsLookupRequest::new(
        "F1234ABCD",
        "{\"section_types\": [\"any_header\"], \"contains_text\": \"CAN Report\"}",
    );
    {
        let (_server, client) = setup(
            "canvases.sections.lookup",
            r#"{"ok": true, "sections": [{"id": "temp:C:eBa219af721c664422cb90a52fac"}]}"#,
        )
        .await;
        let res = client
            .canvases_sections_lookup(&req)
            .await
            .expect("canvases.sections.lookup");
        assert_preserved(
            "canvases.sections.lookup",
            r#"{"ok": true, "sections": [{"id": "temp:C:eBa219af721c664422cb90a52fac"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_append_stream() {
    let req = ChatAppendStreamRequest::new("x", "x")
        .chunks(vec![serde_json::json!({"k": "v"})])
        .markdown_text("**This is bold text**");
    {
        let (_server, client) = setup(
            "chat.appendStream",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247"}"#,
        )
        .await;
        let res = client
            .chat_append_stream(&req)
            .await
            .expect("chat.appendStream");
        assert_preserved(
            "chat.appendStream",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_delete() {
    let req = ChatDeleteRequest::new("x", "\"1405894322.002768\"").as_user(true);
    {
        let (_server, client) = setup(
            "chat.delete",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1401383885.000061"}"#,
        )
        .await;
        let res = client.chat_delete(&req).await.expect("chat.delete");
        assert_preserved(
            "chat.delete",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1401383885.000061"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_delete_scheduled_message() {
    let req = ChatDeleteScheduledMessageRequest::new("C123456789", "Q1234ABCD").as_user(true);
    {
        let (_server, client) = setup("chat.deleteScheduledMessage", r#"{"ok": true}"#).await;
        let res = client
            .chat_delete_scheduled_message(&req)
            .await
            .expect("chat.deleteScheduledMessage");
        assert_preserved("chat.deleteScheduledMessage", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_get_permalink() {
    let req = ChatGetPermalinkRequest::new("x", "x");
    {
        let (_server, client) = setup("chat.getPermalink", r#"{"ok": true, "channel": "C123ABC456", "permalink": "https://ghostbusters.slack.com/archives/C1H9RESGA/p135854651500008"}"#).await;
        let res = client
            .chat_get_permalink(&req)
            .await
            .expect("chat.getPermalink");
        assert_preserved(
            "chat.getPermalink",
            r#"{"ok": true, "channel": "C123ABC456", "permalink": "https://ghostbusters.slack.com/archives/C1H9RESGA/p135854651500008"}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("chat.getPermalink", r#"{"ok": true, "channel": "C123ABC456", "permalink": "https://ghostbusters.slack.com/archives/C1H9RESGL/p135854651700023?thread_ts=1358546515.000008&cid=C1H9RESGL"}"#).await;
        let res = client
            .chat_get_permalink(&req)
            .await
            .expect("chat.getPermalink");
        assert_preserved(
            "chat.getPermalink",
            r#"{"ok": true, "channel": "C123ABC456", "permalink": "https://ghostbusters.slack.com/archives/C1H9RESGL/p135854651700023?thread_ts=1358546515.000008&cid=C1H9RESGL"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_me_message() {
    let req = ChatMeMessageRequest::new("x", "Hello world");
    {
        let (_server, client) = setup(
            "chat.meMessage",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1417671948.000006"}"#,
        )
        .await;
        let res = client.chat_me_message(&req).await.expect("chat.meMessage");
        assert_preserved(
            "chat.meMessage",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1417671948.000006"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_post_ephemeral() {
    let req = ChatPostEphemeralRequest::new("x", "U0BPQUNTA")
        .as_user(true)
        .attachments(vec![slack_web_api::blocks::Attachment::new()])
        .blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .icon_emoji(":chart_with_upwards_trend:")
        .icon_url("http://lorempixel.com/48/48")
        .link_names(true)
        .markdown_text("**This is bold text**")
        .metadata(slack_web_api::blocks::MessageMetadata::new(
            "e",
            serde_json::json!({}),
        ))
        .parse("full")
        .text("Hello world")
        .thread_ts("x")
        .username("My Bot");
    {
        let (_server, client) = setup(
            "chat.postEphemeral",
            r#"{"ok": true, "message_ts": "1502210682.580145"}"#,
        )
        .await;
        let res = client
            .chat_post_ephemeral(&req)
            .await
            .expect("chat.postEphemeral");
        assert_preserved(
            "chat.postEphemeral",
            r#"{"ok": true, "message_ts": "1502210682.580145"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_post_message() {
    let req = ChatPostMessageRequest::new("x")
        .as_user(true)
        .attachments(vec![slack_web_api::blocks::Attachment::new()])
        .blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .current_draft_last_updated_ts("1524523204.000192")
        .icon_emoji(":chart_with_upwards_trend:")
        .icon_url("http://lorempixel.com/48/48")
        .link_names(true)
        .markdown_text("**This is bold text**")
        .metadata(slack_web_api::blocks::MessageMetadata::new(
            "e",
            serde_json::json!({}),
        ))
        .mrkdwn(true)
        .parse("full")
        .reply_broadcast(true)
        .text("Hello world")
        .thread_ts("x")
        .unfurl_links(true)
        .unfurl_media(true)
        .username("My Bot")
        .unfurl_app_links(true);
    {
        let (_server, client) = setup("chat.postMessage", r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247", "message": {"text": "Here's a message for you", "username": "ecto1", "bot_id": "B123ABC456", "attachments": [{"text": "This is an attachment", "id": 1, "fallback": "This is an attachment's fallback"}], "type": "message", "subtype": "bot_message", "ts": "1503435956.000247"}}"#).await;
        let res = client
            .chat_post_message(&req)
            .await
            .expect("chat.postMessage");
        assert_preserved(
            "chat.postMessage",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247", "message": {"text": "Here's a message for you", "username": "ecto1", "bot_id": "B123ABC456", "attachments": [{"text": "This is an attachment", "id": 1, "fallback": "This is an attachment's fallback"}], "type": "message", "subtype": "bot_message", "ts": "1503435956.000247"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_schedule_message() {
    let req = ChatScheduleMessageRequest::new("x", 1)
        .as_user(true)
        .attachments(vec![slack_web_api::blocks::Attachment::new()])
        .blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .link_names(true)
        .markdown_text("**This is bold text**")
        .parse("full")
        .reply_broadcast(true)
        .text("Hello world")
        .thread_ts("x")
        .unfurl_links(true)
        .unfurl_media(true)
        .metadata(slack_web_api::blocks::MessageMetadata::new(
            "e",
            serde_json::json!({}),
        ));
    {
        let (_server, client) = setup("chat.scheduleMessage", r#"{"ok": true, "channel": "C123ABC456", "scheduled_message_id": "Q1298393284", "post_at": "1562180400", "message": {"text": "Here's a message for you in the future", "username": "ecto1", "bot_id": "B123ABC456", "attachments": [{"text": "This is an attachment", "id": 1, "fallback": "This is an attachment's fallback"}], "type": "delayed_message", "subtype": "bot_message"}}"#).await;
        let res = client
            .chat_schedule_message(&req)
            .await
            .expect("chat.scheduleMessage");
        assert_preserved(
            "chat.scheduleMessage",
            r#"{"ok": true, "channel": "C123ABC456", "scheduled_message_id": "Q1298393284", "post_at": "1562180400", "message": {"text": "Here's a message for you in the future", "username": "ecto1", "bot_id": "B123ABC456", "attachments": [{"text": "This is an attachment", "id": 1, "fallback": "This is an attachment's fallback"}], "type": "delayed_message", "subtype": "bot_message"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_scheduled_messages_list() {
    let req = ChatScheduledMessagesListRequest::new()
        .channel("C123456789")
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .latest("1562137200")
        .limit(1)
        .oldest("1562137200")
        .team_id("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        ChatScheduledMessagesListResponse::default().next_cursor(),
        None
    );
    let page = ChatScheduledMessagesListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("chat.scheduledMessages.list", r#"{"ok": true, "scheduled_messages": [{"id": 1298393284, "channel_id": "C1H9RESGL", "post_at": 1551991428, "date_created": 1551891734, "text": "Here's a message for you in the future"}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .chat_scheduled_messages_list(&req)
            .await
            .expect("chat.scheduledMessages.list");
        assert_preserved(
            "chat.scheduledMessages.list",
            r#"{"ok": true, "scheduled_messages": [{"id": 1298393284, "channel_id": "C1H9RESGL", "post_at": 1551991428, "date_created": 1551891734, "text": "Here's a message for you in the future"}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_start_stream() {
    let req = ChatStartStreamRequest::new("x")
        .chunks(vec![serde_json::json!({"k": "v"})])
        .markdown_text("**This is bold text**")
        .thread_ts("1721609600.123456")
        .recipient_user_id("x")
        .recipient_team_id("T0123456789")
        .task_display_mode("plan")
        .icon_emoji(":chart_with_upwards_trend:")
        .icon_url("http://lorempixel.com/48/48")
        .username("My Bot");
    {
        let (_server, client) = setup(
            "chat.startStream",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247"}"#,
        )
        .await;
        let res = client
            .chat_start_stream(&req)
            .await
            .expect("chat.startStream");
        assert_preserved(
            "chat.startStream",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_stop_stream() {
    let req = ChatStopStreamRequest::new("x", "x")
        .chunks(vec![serde_json::json!({"k": "v"})])
        .markdown_text("**This is bold text**")
        .blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .metadata(slack_web_api::blocks::MessageMetadata::new(
            "e",
            serde_json::json!({}),
        ))
        .session_status("processing");
    {
        let (_server, client) = setup("chat.stopStream", r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247", "message": {"text": "Here's the final streamed message content", "bot_id": "B123ABC456", "ts": "1503435956.000247", "type": "message", "subtype": "bot_message"}}"#).await;
        let res = client
            .chat_stop_stream(&req)
            .await
            .expect("chat.stopStream");
        assert_preserved(
            "chat.stopStream",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1503435956.000247", "message": {"text": "Here's the final streamed message content", "bot_id": "B123ABC456", "ts": "1503435956.000247", "type": "message", "subtype": "bot_message"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_unfurl() {
    let req = ChatUnfurlRequest::new()
        .channel("x")
        .ts("x")
        .unfurls("x")
        .user_auth_message("x")
        .user_auth_required(true)
        .user_auth_url("https://example.com/onboarding?user_id=xxx")
        .user_auth_blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .unfurl_id("Uxxxxxxx-909b5454-75f8-4ac4-b325-1b40e230bbd8")
        .source("composer");
    {
        let (_server, client) = setup("chat.unfurl", r#"{"ok": true}"#).await;
        let res = client.chat_unfurl(&req).await.expect("chat.unfurl");
        assert_preserved("chat.unfurl", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn chat_update() {
    let req = ChatUpdateRequest::new("x", "\"1405894322.002768\"")
        .as_user(true)
        .attachments(vec![slack_web_api::blocks::Attachment::new()])
        .unfurled_attachments("[{\"pretext\": \"pre-hello\", \"text\": \"text-world\"}]")
        .blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .markdown_text("**This is bold text**")
        .metadata(slack_web_api::blocks::MessageMetadata::new(
            "e",
            serde_json::json!({}),
        ))
        .link_names(true)
        .parse("none")
        .text("Hello world")
        .reply_broadcast(true)
        .file_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("chat.update", r#"{"ok": true, "channel": "C123ABC456", "ts": "1401383885.000061", "text": "Updated text you carefully authored", "message": {"text": "Updated text you carefully authored", "user": "U34567890"}}"#).await;
        let res = client.chat_update(&req).await.expect("chat.update");
        assert_preserved(
            "chat.update",
            r#"{"ok": true, "channel": "C123ABC456", "ts": "1401383885.000061", "text": "Updated text you carefully authored", "message": {"text": "Updated text you carefully authored", "user": "U34567890"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_accept_shared_invite() {
    let req = ConversationsAcceptSharedInviteRequest::new("puppies-r-us")
        .is_private(true)
        .free_trial_accepted(true)
        .invite_id("x")
        .channel_id("x")
        .team_id("x");
    {
        let (_server, client) = setup("conversations.acceptSharedInvite", r#"{"ok": true, "implicit_approval": true, "channel_id": "C0001111", "invite_id": "I00043221"}"#).await;
        let res = client
            .conversations_accept_shared_invite(&req)
            .await
            .expect("conversations.acceptSharedInvite");
        assert_preserved(
            "conversations.acceptSharedInvite",
            r#"{"ok": true, "implicit_approval": true, "channel_id": "C0001111", "invite_id": "I00043221"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_approve_shared_invite() {
    let req = ConversationsApproveSharedInviteRequest::new("x").target_team("x");
    {
        let (_server, client) = setup("conversations.approveSharedInvite", r#"{"ok": true}"#).await;
        let res = client
            .conversations_approve_shared_invite(&req)
            .await
            .expect("conversations.approveSharedInvite");
        assert_preserved("conversations.approveSharedInvite", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_archive() {
    let req = ConversationsArchiveRequest::new("x");
    {
        let (_server, client) = setup("conversations.archive", r#"{"ok": true}"#).await;
        let res = client
            .conversations_archive(&req)
            .await
            .expect("conversations.archive");
        assert_preserved("conversations.archive", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_canvases_create() {
    let req = ConversationsCanvasesCreateRequest::new("x")
        .document_content("{\"type\": \"markdown\", \"markdown\": \"> channel canvas!\"}")
        .title("The Coolest Title Ever");
    {
        let (_server, client) = setup(
            "conversations.canvases.create",
            r#"{"ok": true, "canvas_id": "F1234ABCD"}"#,
        )
        .await;
        let res = client
            .conversations_canvases_create(&req)
            .await
            .expect("conversations.canvases.create");
        assert_preserved(
            "conversations.canvases.create",
            r#"{"ok": true, "canvas_id": "F1234ABCD"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_close() {
    let req = ConversationsCloseRequest::new("x");
    {
        let (_server, client) = setup("conversations.close", r#"{"ok": true}"#).await;
        let res = client
            .conversations_close(&req)
            .await
            .expect("conversations.close");
        assert_preserved("conversations.close", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup(
            "conversations.close",
            r#"{"ok": true, "no_op": true, "already_closed": true}"#,
        )
        .await;
        let res = client
            .conversations_close(&req)
            .await
            .expect("conversations.close");
        assert_preserved(
            "conversations.close",
            r#"{"ok": true, "no_op": true, "already_closed": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_create() {
    let req = ConversationsCreateRequest::new("mychannel")
        .is_private(true)
        .team_id("x");
    {
        let (_server, client) = setup("conversations.create", r#"{"ok": true, "channel": {"id": "C0EAQDV4Z", "name": "endeavor", "is_channel": true, "is_group": false, "is_im": false, "created": 1504554479, "creator": "U0123456", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "endeavor", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "last_read": "0000000000.000000", "latest": null, "unread_count": 0, "unread_count_display": 0, "topic": {"value": "", "creator": "", "last_set": 0}, "properties": {"canvas": {"file_id": "F123ABC456", "is_empty": true, "quip_thread_id": "JAB1CDefGhI"}}, "purpose": {"value": "", "creator": "", "last_set": 0}, "previous_names": [], "priority": 0}}"#).await;
        let res = client
            .conversations_create(&req)
            .await
            .expect("conversations.create");
        assert_preserved(
            "conversations.create",
            r#"{"ok": true, "channel": {"id": "C0EAQDV4Z", "name": "endeavor", "is_channel": true, "is_group": false, "is_im": false, "created": 1504554479, "creator": "U0123456", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "endeavor", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "last_read": "0000000000.000000", "latest": null, "unread_count": 0, "unread_count_display": 0, "topic": {"value": "", "creator": "", "last_set": 0}, "properties": {"canvas": {"file_id": "F123ABC456", "is_empty": true, "quip_thread_id": "JAB1CDefGhI"}}, "purpose": {"value": "", "creator": "", "last_set": 0}, "previous_names": [], "priority": 0}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_decline_shared_invite() {
    let req = ConversationsDeclineSharedInviteRequest::new("x").target_team("x");
    {
        let (_server, client) = setup("conversations.declineSharedInvite", r#"{"ok": true}"#).await;
        let res = client
            .conversations_decline_shared_invite(&req)
            .await
            .expect("conversations.declineSharedInvite");
        assert_preserved("conversations.declineSharedInvite", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_external_invite_permissions_set() {
    let req =
        ConversationsExternalInvitePermissionsSetRequest::new("C123456", "T726G27TT", "upgrade");
    {
        let (_server, client) = setup(
            "conversations.externalInvitePermissions.set",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .conversations_external_invite_permissions_set(&req)
            .await
            .expect("conversations.externalInvitePermissions.set");
        assert_preserved(
            "conversations.externalInvitePermissions.set",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_history() {
    let req = ConversationsHistoryRequest::new("x")
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .include_all_metadata(true)
        .inclusive(true)
        .latest("x")
        .limit(1)
        .oldest("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(ConversationsHistoryResponse::default().next_cursor(), None);
    let page = ConversationsHistoryResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("conversations.history", r#"{"ok": true, "messages": [{"type": "message", "user": "U123ABC456", "text": "I find you punny and would like to smell your nose letter", "ts": "1512085950.000216"}, {"type": "message", "user": "U222BBB222", "text": "What, you want to smell my shoes better?", "ts": "1512104434.000490"}], "has_more": true, "pin_count": 0, "response_metadata": {"next_cursor": "bmV4dF90czoxNTEyMDg1ODYxMDAwNTQz"}}"#).await;
        let res = client
            .conversations_history(&req)
            .await
            .expect("conversations.history");
        assert_preserved(
            "conversations.history",
            r#"{"ok": true, "messages": [{"type": "message", "user": "U123ABC456", "text": "I find you punny and would like to smell your nose letter", "ts": "1512085950.000216"}, {"type": "message", "user": "U222BBB222", "text": "What, you want to smell my shoes better?", "ts": "1512104434.000490"}], "has_more": true, "pin_count": 0, "response_metadata": {"next_cursor": "bmV4dF90czoxNTEyMDg1ODYxMDAwNTQz"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("conversations.history", r#"{"ok": true, "messages": [{"type": "message", "user": "U123ABC456", "text": "I find you punny and would like to smell your nose letter", "ts": "1512085950.000216"}, {"type": "message", "user": "U222BBB222", "text": "Isn't this whether dreadful? <https://badpuns.example.com/puns/123>", "attachments": [{"service_name": "Leg end nary a laugh, Ink.", "text": "This is likely a pun about the weather.", "fallback": "We're withholding a pun from you", "thumb_url": "https://badpuns.example.com/puns/123.png", "thumb_width": 1920, "thumb_height": 700, "id": 1}], "ts": "1512085950.218404"}], "has_more": true, "pin_count": 0, "response_metadata": {"next_cursor": "bmV4dF90czoxNTEyMTU0NDA5MDAwMjU2"}}"#).await;
        let res = client
            .conversations_history(&req)
            .await
            .expect("conversations.history");
        assert_preserved(
            "conversations.history",
            r#"{"ok": true, "messages": [{"type": "message", "user": "U123ABC456", "text": "I find you punny and would like to smell your nose letter", "ts": "1512085950.000216"}, {"type": "message", "user": "U222BBB222", "text": "Isn't this whether dreadful? <https://badpuns.example.com/puns/123>", "attachments": [{"service_name": "Leg end nary a laugh, Ink.", "text": "This is likely a pun about the weather.", "fallback": "We're withholding a pun from you", "thumb_url": "https://badpuns.example.com/puns/123.png", "thumb_width": 1920, "thumb_height": 700, "id": 1}], "ts": "1512085950.218404"}], "has_more": true, "pin_count": 0, "response_metadata": {"next_cursor": "bmV4dF90czoxNTEyMTU0NDA5MDAwMjU2"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("conversations.history", r#"{"ok": true, "latest": "1512085950.000216", "messages": [{"type": "message", "user": "U123ABC456", "text": "I find you punny and would like to smell your nose letter", "ts": "1512085950.000216"}], "has_more": true, "pin_count": 0, "response_metadata": {"next_cursor": "bmV4dF90czoxNTEyMzU2NTI2MDAwMTMw"}}"#).await;
        let res = client
            .conversations_history(&req)
            .await
            .expect("conversations.history");
        assert_preserved(
            "conversations.history",
            r#"{"ok": true, "latest": "1512085950.000216", "messages": [{"type": "message", "user": "U123ABC456", "text": "I find you punny and would like to smell your nose letter", "ts": "1512085950.000216"}], "has_more": true, "pin_count": 0, "response_metadata": {"next_cursor": "bmV4dF90czoxNTEyMzU2NTI2MDAwMTMw"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_info() {
    let req = ConversationsInfoRequest::new("x")
        .include_locale(true)
        .include_num_members(true);
    {
        let (_server, client) = setup("conversations.info", r#"{"ok": true, "channel": {"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "is_mpim": false, "is_private": false, "created": 1654868334, "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_frozen": false, "is_org_shared": false, "is_pending_ext_shared": false, "pending_shared": [], "context_team_id": "T123ABC456", "updated": 1723130875818, "parent_conversation": null, "creator": "U123ABC456", "is_ext_shared": false, "shared_team_ids": ["T123ABC456"], "pending_connected_team_ids": [], "topic": {"value": "For public discussion of generalities", "creator": "W012A3BCD", "last_set": 1449709364}, "purpose": {"value": "This part of the workspace is for fun. Make fun here.", "creator": "W012A3BCD", "last_set": 1449709364}, "properties": {"tabs": [{"id": "workflows", "label": "", "type": "workflows"}, {"id": "files", "label": "", "type": "files"}, {"id": "bookmarks", "label": "", "type": "bookmarks"}]}, "previous_names": []}}"#).await;
        let res = client
            .conversations_info(&req)
            .await
            .expect("conversations.info");
        assert_preserved(
            "conversations.info",
            r#"{"ok": true, "channel": {"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "is_mpim": false, "is_private": false, "created": 1654868334, "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_frozen": false, "is_org_shared": false, "is_pending_ext_shared": false, "pending_shared": [], "context_team_id": "T123ABC456", "updated": 1723130875818, "parent_conversation": null, "creator": "U123ABC456", "is_ext_shared": false, "shared_team_ids": ["T123ABC456"], "pending_connected_team_ids": [], "topic": {"value": "For public discussion of generalities", "creator": "W012A3BCD", "last_set": 1449709364}, "purpose": {"value": "This part of the workspace is for fun. Make fun here.", "creator": "W012A3BCD", "last_set": 1449709364}, "properties": {"tabs": [{"id": "workflows", "label": "", "type": "workflows"}, {"id": "files", "label": "", "type": "files"}, {"id": "bookmarks", "label": "", "type": "bookmarks"}]}, "previous_names": []}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("conversations.info", r#"{"ok": true, "channel": {"id": "C012AB3CD", "created": 1507235627, "is_im": true, "is_org_shared": false, "user": "U27FFLNF4", "last_read": "1513718191.000038", "latest": {"type": "message", "user": "U5R3PALPN", "text": "Psssst!", "ts": "1513718191.000038"}, "unread_count": 0, "unread_count_display": 0, "is_open": true, "locale": "en-US", "priority": 0.043016851216706}}"#).await;
        let res = client
            .conversations_info(&req)
            .await
            .expect("conversations.info");
        assert_preserved(
            "conversations.info",
            r#"{"ok": true, "channel": {"id": "C012AB3CD", "created": 1507235627, "is_im": true, "is_org_shared": false, "user": "U27FFLNF4", "last_read": "1513718191.000038", "latest": {"type": "message", "user": "U5R3PALPN", "text": "Psssst!", "ts": "1513718191.000038"}, "unread_count": 0, "unread_count_display": 0, "is_open": true, "locale": "en-US", "priority": 0.043016851216706}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("conversations.info", r#"{"ok": true, "channel": {"id": "C012AB3CD", "created": 1507235627, "is_im": true, "is_org_shared": false, "user": "U27FFLNF4", "last_read": "1513718191.000038", "latest": {"type": "message", "user": "U5R3PALPN", "text": "Psssst!", "ts": "1513718191.000038"}, "unread_count": 0, "unread_count_display": 0, "is_open": true, "locale": "en-US", "priority": 0.043016851216706, "num_members": 2}}"#).await;
        let res = client
            .conversations_info(&req)
            .await
            .expect("conversations.info");
        assert_preserved(
            "conversations.info",
            r#"{"ok": true, "channel": {"id": "C012AB3CD", "created": 1507235627, "is_im": true, "is_org_shared": false, "user": "U27FFLNF4", "last_read": "1513718191.000038", "latest": {"type": "message", "user": "U5R3PALPN", "text": "Psssst!", "ts": "1513718191.000038"}, "unread_count": 0, "unread_count_display": 0, "is_open": true, "locale": "en-US", "priority": 0.043016851216706, "num_members": 2}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_invite() {
    let req = ConversationsInviteRequest::new("x", "x").force(true);
    {
        let (_server, client) = setup("conversations.invite", r#"{"ok": true, "channel": {"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "W012A3BCD", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_read_only": false, "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "last_read": "1502126650.228446", "topic": {"value": "For public discussion of generalities", "creator": "W012A3BCD", "last_set": 1449709364}, "purpose": {"value": "This part of the workspace is for fun. Make fun here.", "creator": "W012A3BCD", "last_set": 1449709364}, "previous_names": ["specifics", "abstractions", "etc"]}}"#).await;
        let res = client
            .conversations_invite(&req)
            .await
            .expect("conversations.invite");
        assert_preserved(
            "conversations.invite",
            r#"{"ok": true, "channel": {"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "W012A3BCD", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_read_only": false, "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "last_read": "1502126650.228446", "topic": {"value": "For public discussion of generalities", "creator": "W012A3BCD", "last_set": 1449709364}, "purpose": {"value": "This part of the workspace is for fun. Make fun here.", "creator": "W012A3BCD", "last_set": 1449709364}, "previous_names": ["specifics", "abstractions", "etc"]}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_invite_shared() {
    let req = ConversationsInviteSharedRequest::new("x")
        .emails(vec!["A1".to_string(), "A2".to_string()])
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .external_limited(true);
    {
        let (_server, client) = setup(
            "conversations.inviteShared",
            r#"{"ok": true, "invite_id": "I02UKAJ6RJA", "is_legacy_shared_channel": false}"#,
        )
        .await;
        let res = client
            .conversations_invite_shared(&req)
            .await
            .expect("conversations.inviteShared");
        assert_preserved(
            "conversations.inviteShared",
            r#"{"ok": true, "invite_id": "I02UKAJ6RJA", "is_legacy_shared_channel": false}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup(
            "conversations.inviteShared",
            r#"{"ok": true, "invite_id": "I02UKAJ6RJA", "is_legacy_shared_channel": false}"#,
        )
        .await;
        let res = client
            .conversations_invite_shared(&req)
            .await
            .expect("conversations.inviteShared");
        assert_preserved(
            "conversations.inviteShared",
            r#"{"ok": true, "invite_id": "I02UKAJ6RJA", "is_legacy_shared_channel": false}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup(
            "conversations.inviteShared",
            r#"{"ok": true, "invite_id": "I02UKAJ6RJA", "is_legacy_shared_channel": false}"#,
        )
        .await;
        let res = client
            .conversations_invite_shared(&req)
            .await
            .expect("conversations.inviteShared");
        assert_preserved(
            "conversations.inviteShared",
            r#"{"ok": true, "invite_id": "I02UKAJ6RJA", "is_legacy_shared_channel": false}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_join() {
    let req = ConversationsJoinRequest::new("x");
    {
        let (_server, client) = setup("conversations.join", r#"{"ok": true, "channel": {"id": "C061EG9SL", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U061F7AUR", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "topic": {"value": "Which widget do you worry about?", "creator": "", "last_set": 0}, "purpose": {"value": "For widget discussion", "creator": "", "last_set": 0}, "previous_names": []}, "warning": "already_in_channel", "response_metadata": {"warnings": ["already_in_channel"]}}"#).await;
        let res = client
            .conversations_join(&req)
            .await
            .expect("conversations.join");
        assert_preserved(
            "conversations.join",
            r#"{"ok": true, "channel": {"id": "C061EG9SL", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U061F7AUR", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "topic": {"value": "Which widget do you worry about?", "creator": "", "last_set": 0}, "purpose": {"value": "For widget discussion", "creator": "", "last_set": 0}, "previous_names": []}, "warning": "already_in_channel", "response_metadata": {"warnings": ["already_in_channel"]}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_kick() {
    let req = ConversationsKickRequest::new("x").user("x");
    {
        let (_server, client) = setup("conversations.kick", r#"{"ok": true, "errors": {}}"#).await;
        let res = client
            .conversations_kick(&req)
            .await
            .expect("conversations.kick");
        assert_preserved("conversations.kick", r#"{"ok": true, "errors": {}}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_leave() {
    let req = ConversationsLeaveRequest::new("x");
    {
        let (_server, client) = setup("conversations.leave", r#"{"ok": true}"#).await;
        let res = client
            .conversations_leave(&req)
            .await
            .expect("conversations.leave");
        assert_preserved("conversations.leave", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_list() {
    let req = ConversationsListRequest::new()
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .exclude_archived(true)
        .limit(1)
        .team_id("x")
        .types("public_channel,private_channel");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(ConversationsListResponse::default().next_cursor(), None);
    let page = ConversationsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("conversations.list", r#"{"ok": true, "channels": [{"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U012A3CDE", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "updated": 1678229664302, "topic": {"value": "Company-wide announcements and work-based matters", "creator": "", "last_set": 0}, "purpose": {"value": "This channel is for team-wide communication and announcements. All team members are in this channel.", "creator": "", "last_set": 0}, "previous_names": [], "num_members": 4}, {"id": "C061EG9T2", "name": "random", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "random", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "updated": 1678229664302, "topic": {"value": "Non-work banter and water cooler conversation", "creator": "", "last_set": 0}, "purpose": {"value": "A place for non-work-related flimflam, faffing, hodge-podge or jibber-jabber you'd prefer to keep out of more focused work-related channels.", "creator": "", "last_set": 0}, "previous_names": [], "num_members": 4}], "response_metadata": {"next_cursor": "dGVhbTpDMDYxRkE1UEI="}}"#).await;
        let res = client
            .conversations_list(&req)
            .await
            .expect("conversations.list");
        assert_preserved(
            "conversations.list",
            r#"{"ok": true, "channels": [{"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U012A3CDE", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "updated": 1678229664302, "topic": {"value": "Company-wide announcements and work-based matters", "creator": "", "last_set": 0}, "purpose": {"value": "This channel is for team-wide communication and announcements. All team members are in this channel.", "creator": "", "last_set": 0}, "previous_names": [], "num_members": 4}, {"id": "C061EG9T2", "name": "random", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "random", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "updated": 1678229664302, "topic": {"value": "Non-work banter and water cooler conversation", "creator": "", "last_set": 0}, "purpose": {"value": "A place for non-work-related flimflam, faffing, hodge-podge or jibber-jabber you'd prefer to keep out of more focused work-related channels.", "creator": "", "last_set": 0}, "previous_names": [], "num_members": 4}], "response_metadata": {"next_cursor": "dGVhbTpDMDYxRkE1UEI="}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("conversations.list", r#"{"ok": true, "channels": [{"id": "G0AKFJBEU", "name": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_channel": false, "is_group": true, "is_im": false, "created": 1493657761, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": true, "is_mpim": true, "is_open": true, "updated": 1678229664302, "topic": {"value": "Group messaging", "creator": "U061F7AUR", "last_set": 1493657761}, "purpose": {"value": "Group messaging with: @mr.banks @slactions-jackson @beforebot", "creator": "U061F7AUR", "last_set": 1493657761}, "priority": 0}, {"id": "D0C0F7S8Y", "created": 1498500348, "is_im": true, "is_org_shared": false, "user": "U0BS9U4SV", "is_user_deleted": false, "priority": 0}, {"id": "D0BSHH4AD", "created": 1498511030, "is_im": true, "is_org_shared": false, "user": "U0C0NS9HN", "is_user_deleted": false, "priority": 0}], "response_metadata": {"next_cursor": "aW1faWQ6RDBCSDk1RExI"}}"#).await;
        let res = client
            .conversations_list(&req)
            .await
            .expect("conversations.list");
        assert_preserved(
            "conversations.list",
            r#"{"ok": true, "channels": [{"id": "G0AKFJBEU", "name": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_channel": false, "is_group": true, "is_im": false, "created": 1493657761, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": true, "is_mpim": true, "is_open": true, "updated": 1678229664302, "topic": {"value": "Group messaging", "creator": "U061F7AUR", "last_set": 1493657761}, "purpose": {"value": "Group messaging with: @mr.banks @slactions-jackson @beforebot", "creator": "U061F7AUR", "last_set": 1493657761}, "priority": 0}, {"id": "D0C0F7S8Y", "created": 1498500348, "is_im": true, "is_org_shared": false, "user": "U0BS9U4SV", "is_user_deleted": false, "priority": 0}, {"id": "D0BSHH4AD", "created": 1498511030, "is_im": true, "is_org_shared": false, "user": "U0C0NS9HN", "is_user_deleted": false, "priority": 0}], "response_metadata": {"next_cursor": "aW1faWQ6RDBCSDk1RExI"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_list_connect_invites() {
    let req = ConversationsListConnectInvitesRequest::new()
        .team_id("x")
        .count(1)
        .cursor("5c3e53d5");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        ConversationsListConnectInvitesResponse::default().next_cursor(),
        None
    );
    let page = ConversationsListConnectInvitesResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("conversations.listConnectInvites", r#"{"ok": true, "invites": [{"direction": "outgoing", "status": "sent", "date_last_updated": 1622591318, "invite_type": "channel", "invite": {"id": "I0139HVDSDQ", "date_created": 1622591287, "date_invalid": 1623800887, "inviting_team": {"id": "E0ANZNL03", "name": "oonamole", "icon": {}, "is_verified": false, "domain": "oonamole", "date_created": 1559335482}, "inviting_user": {"id": "W0ANZNNAX", "team_id": "E0ANZNL03", "name": "ben_boss", "updated": 1616521338, "profile": {"real_name": "Puppy InCharge", "display_name": "puppy_incharge", "real_name_normalized": "Puppy InCharge", "display_name_normalized": "puppy_incharge", "team": "E0ANZNL03", "avatar_hash": "g767309df8c3", "email": "puppyincharge@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "link": "..."}, "channel": {"id": "C013AGH1GBD", "is_private": false, "is_im": false, "name": "pb-shared-7"}, "acceptances": [{"approval_status": "pending_approval", "date_accepted": 1622591318, "date_invalid": 1623800918, "date_last_updated": 1622591318, "accepting_team": {"id": "T0PP93X0Q", "name": "Doughboy", "icon": {}, "is_verified": false, "domain": "doughboy", "date_created": 1521573656}, "accepting_user": {"id": "U0PP93X1N", "team_id": "T0PP93X0Q", "name": "bredman", "updated": 1619479454, "profile": {"real_name": "Brent Doodle", "display_name": "Brent Doodle", "real_name_normalized": "Brent", "display_name_normalized": "Brent Doodle", "team": "T0PP93X0Q", "avatar_hash": "ge2de9bb3fde", "email": "bdoodle@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "reviews": [{"type": "approval", "date_review": 1622591318, "reviewing_team": {"id": "T0PP93X0Q", "name": "Doughboy", "icon": {}, "is_verified": false, "domain": "doughboy", "date_created": 1521573656}}]}]}, {"direction": "incoming", "status": "sent", "date_last_updated": 1622591697, "invite_type": "channel", "invite": {"id": "I0139HWQ134", "date_created": 1622591671, "date_invalid": 1623801271, "inviting_team": {"id": "T0PP93X0Q", "name": "Doughboy", "icon": {}, "is_verified": false, "domain": "doughboy", "date_created": 1521573656}, "inviting_user": {"id": "U0PP93X1N", "team_id": "T0PP93X0Q", "name": "bredman", "updated": 1619479454, "profile": {"real_name": "Pet Dog", "display_name": "Pet Dog", "real_name_normalized": "Pet Dog", "display_name_normalized": "Pet Dog", "team": "T0PP93X0Q", "avatar_hash": "ge2de9bb3fde", "email": "bront@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "link": "..."}, "channel": {"id": "C013AGJCG9H", "is_private": false, "is_im": false, "name": "shared-channel-72"}, "acceptances": [{"approval_status": "pending_approval", "date_accepted": 1622591697, "date_invalid": 1623801297, "date_last_updated": 1622591697, "accepting_team": {"id": "E0ANZNL03", "name": "oonamole", "icon": {}, "is_verified": false, "domain": "oonamole", "date_created": 1559335482}, "accepting_user": {"id": "W0ANZNNAX", "team_id": "E0ANZNL03", "name": "ben_boss", "updated": 1616521338, "profile": {"real_name": "Brent Puppies", "display_name": "Bront", "real_name_normalized": "Brent Puppies", "display_name_normalized": "brent_puppies", "team": "E0ANZNL03", "avatar_hash": "g767309df8c3", "email": "brent@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "reviews": [{"type": "approval", "date_review": 1622591697, "reviewing_team": {"id": "E0ANZNL03", "name": "oonamole", "icon": {}, "is_verified": false, "domain": "oonamole", "date_created": 1559335482}}]}]}]}"#).await;
        let res = client
            .conversations_list_connect_invites(&req)
            .await
            .expect("conversations.listConnectInvites");
        assert_preserved(
            "conversations.listConnectInvites",
            r#"{"ok": true, "invites": [{"direction": "outgoing", "status": "sent", "date_last_updated": 1622591318, "invite_type": "channel", "invite": {"id": "I0139HVDSDQ", "date_created": 1622591287, "date_invalid": 1623800887, "inviting_team": {"id": "E0ANZNL03", "name": "oonamole", "icon": {}, "is_verified": false, "domain": "oonamole", "date_created": 1559335482}, "inviting_user": {"id": "W0ANZNNAX", "team_id": "E0ANZNL03", "name": "ben_boss", "updated": 1616521338, "profile": {"real_name": "Puppy InCharge", "display_name": "puppy_incharge", "real_name_normalized": "Puppy InCharge", "display_name_normalized": "puppy_incharge", "team": "E0ANZNL03", "avatar_hash": "g767309df8c3", "email": "puppyincharge@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "link": "..."}, "channel": {"id": "C013AGH1GBD", "is_private": false, "is_im": false, "name": "pb-shared-7"}, "acceptances": [{"approval_status": "pending_approval", "date_accepted": 1622591318, "date_invalid": 1623800918, "date_last_updated": 1622591318, "accepting_team": {"id": "T0PP93X0Q", "name": "Doughboy", "icon": {}, "is_verified": false, "domain": "doughboy", "date_created": 1521573656}, "accepting_user": {"id": "U0PP93X1N", "team_id": "T0PP93X0Q", "name": "bredman", "updated": 1619479454, "profile": {"real_name": "Brent Doodle", "display_name": "Brent Doodle", "real_name_normalized": "Brent", "display_name_normalized": "Brent Doodle", "team": "T0PP93X0Q", "avatar_hash": "ge2de9bb3fde", "email": "bdoodle@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "reviews": [{"type": "approval", "date_review": 1622591318, "reviewing_team": {"id": "T0PP93X0Q", "name": "Doughboy", "icon": {}, "is_verified": false, "domain": "doughboy", "date_created": 1521573656}}]}]}, {"direction": "incoming", "status": "sent", "date_last_updated": 1622591697, "invite_type": "channel", "invite": {"id": "I0139HWQ134", "date_created": 1622591671, "date_invalid": 1623801271, "inviting_team": {"id": "T0PP93X0Q", "name": "Doughboy", "icon": {}, "is_verified": false, "domain": "doughboy", "date_created": 1521573656}, "inviting_user": {"id": "U0PP93X1N", "team_id": "T0PP93X0Q", "name": "bredman", "updated": 1619479454, "profile": {"real_name": "Pet Dog", "display_name": "Pet Dog", "real_name_normalized": "Pet Dog", "display_name_normalized": "Pet Dog", "team": "T0PP93X0Q", "avatar_hash": "ge2de9bb3fde", "email": "bront@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "link": "..."}, "channel": {"id": "C013AGJCG9H", "is_private": false, "is_im": false, "name": "shared-channel-72"}, "acceptances": [{"approval_status": "pending_approval", "date_accepted": 1622591697, "date_invalid": 1623801297, "date_last_updated": 1622591697, "accepting_team": {"id": "E0ANZNL03", "name": "oonamole", "icon": {}, "is_verified": false, "domain": "oonamole", "date_created": 1559335482}, "accepting_user": {"id": "W0ANZNNAX", "team_id": "E0ANZNL03", "name": "ben_boss", "updated": 1616521338, "profile": {"real_name": "Brent Puppies", "display_name": "Bront", "real_name_normalized": "Brent Puppies", "display_name_normalized": "brent_puppies", "team": "E0ANZNL03", "avatar_hash": "g767309df8c3", "email": "brent@slack.com", "image_24": "...", "image_32": "...", "image_48": "...", "image_72": "...", "image_192": "...", "image_512": "..."}}, "reviews": [{"type": "approval", "date_review": 1622591697, "reviewing_team": {"id": "E0ANZNL03", "name": "oonamole", "icon": {}, "is_verified": false, "domain": "oonamole", "date_created": 1559335482}}]}]}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_mark() {
    let req = ConversationsMarkRequest::new("C012345678", "1593473566.000200");
    {
        let (_server, client) = setup("conversations.mark", r#"{"ok": true}"#).await;
        let res = client
            .conversations_mark(&req)
            .await
            .expect("conversations.mark");
        assert_preserved("conversations.mark", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_members() {
    let req = ConversationsMembersRequest::new("x")
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(ConversationsMembersResponse::default().next_cursor(), None);
    let page = ConversationsMembersResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("conversations.members", r#"{"ok": true, "members": ["U023BECGF", "U061F7AUR", "W012A3CDE"], "response_metadata": {"next_cursor": "e3VzZXJfaWQ6IFcxMjM0NTY3fQ=="}}"#).await;
        let res = client
            .conversations_members(&req)
            .await
            .expect("conversations.members");
        assert_preserved(
            "conversations.members",
            r#"{"ok": true, "members": ["U023BECGF", "U061F7AUR", "W012A3CDE"], "response_metadata": {"next_cursor": "e3VzZXJfaWQ6IFcxMjM0NTY3fQ=="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_open() {
    let req = ConversationsOpenRequest::new()
        .channel("x")
        .return_im(true)
        .users("x")
        .prevent_creation(true);
    {
        let (_server, client) = setup(
            "conversations.open",
            r#"{"ok": true, "channel": {"id": "D069C7QFK"}}"#,
        )
        .await;
        let res = client
            .conversations_open(&req)
            .await
            .expect("conversations.open");
        assert_preserved(
            "conversations.open",
            r#"{"ok": true, "channel": {"id": "D069C7QFK"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("conversations.open", r#"{"ok": true, "no_op": true, "already_open": true, "channel": {"id": "D069C7QFK", "created": 1460147748, "is_im": true, "is_org_shared": false, "user": "U069C7QF3", "last_read": "0000000000.000000", "latest": null, "unread_count": 0, "unread_count_display": 0, "is_open": true, "priority": 0}}"#).await;
        let res = client
            .conversations_open(&req)
            .await
            .expect("conversations.open");
        assert_preserved(
            "conversations.open",
            r#"{"ok": true, "no_op": true, "already_open": true, "channel": {"id": "D069C7QFK", "created": 1460147748, "is_im": true, "is_org_shared": false, "user": "U069C7QF3", "last_read": "0000000000.000000", "latest": null, "unread_count": 0, "unread_count_display": 0, "is_open": true, "priority": 0}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_rename() {
    let req = ConversationsRenameRequest::new("x", "x");
    {
        let (_server, client) = setup("conversations.rename", r#"{"ok": true, "channel": {"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "W012A3BCD", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_read_only": false, "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "last_read": "1502126650.228446", "topic": {"value": "For public discussion of generalities", "creator": "W012A3BCD", "last_set": 1449709364}, "purpose": {"value": "This part of the workspace is for fun. Make fun here.", "creator": "W012A3BCD", "last_set": 1449709364}, "previous_names": ["specifics", "abstractions", "etc"], "num_members": 23, "locale": "en-US"}}"#).await;
        let res = client
            .conversations_rename(&req)
            .await
            .expect("conversations.rename");
        assert_preserved(
            "conversations.rename",
            r#"{"ok": true, "channel": {"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "W012A3BCD", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_read_only": false, "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_member": true, "is_private": false, "is_mpim": false, "last_read": "1502126650.228446", "topic": {"value": "For public discussion of generalities", "creator": "W012A3BCD", "last_set": 1449709364}, "purpose": {"value": "This part of the workspace is for fun. Make fun here.", "creator": "W012A3BCD", "last_set": 1449709364}, "previous_names": ["specifics", "abstractions", "etc"], "num_members": 23, "locale": "en-US"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_replies() {
    let req = ConversationsRepliesRequest::new("x", "x")
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .include_all_metadata(true)
        .inclusive(true)
        .latest("x")
        .limit(1)
        .oldest("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(ConversationsRepliesResponse::default().next_cursor(), None);
    let page = ConversationsRepliesResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("conversations.replies", r#"{"messages": [{"type": "message", "user": "U061F7AUR", "text": "island", "thread_ts": "1482960137.003543", "reply_count": 3, "subscribed": true, "last_read": "1484678597.521003", "unread_count": 0, "ts": "1482960137.003543"}, {"type": "message", "user": "U061F7AUR", "text": "one island", "thread_ts": "1482960137.003543", "parent_user_id": "U061F7AUR", "ts": "1483037603.017503"}, {"type": "message", "user": "U061F7AUR", "text": "two island", "thread_ts": "1482960137.003543", "parent_user_id": "U061F7AUR", "ts": "1483051909.018632"}, {"type": "message", "user": "U061F7AUR", "text": "three for the land", "thread_ts": "1482960137.003543", "parent_user_id": "U061F7AUR", "ts": "1483125339.020269"}], "has_more": true, "ok": true, "response_metadata": {"next_cursor": "bmV4dF90czoxNDg0Njc4MjkwNTE3MDkx"}}"#).await;
        let res = client
            .conversations_replies(&req)
            .await
            .expect("conversations.replies");
        assert_preserved(
            "conversations.replies",
            r#"{"messages": [{"type": "message", "user": "U061F7AUR", "text": "island", "thread_ts": "1482960137.003543", "reply_count": 3, "subscribed": true, "last_read": "1484678597.521003", "unread_count": 0, "ts": "1482960137.003543"}, {"type": "message", "user": "U061F7AUR", "text": "one island", "thread_ts": "1482960137.003543", "parent_user_id": "U061F7AUR", "ts": "1483037603.017503"}, {"type": "message", "user": "U061F7AUR", "text": "two island", "thread_ts": "1482960137.003543", "parent_user_id": "U061F7AUR", "ts": "1483051909.018632"}, {"type": "message", "user": "U061F7AUR", "text": "three for the land", "thread_ts": "1482960137.003543", "parent_user_id": "U061F7AUR", "ts": "1483125339.020269"}], "has_more": true, "ok": true, "response_metadata": {"next_cursor": "bmV4dF90czoxNDg0Njc4MjkwNTE3MDkx"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_request_shared_invite_approve() {
    let req = ConversationsRequestSharedInviteApproveRequest::new("x")
        .is_external_limited(true)
        .channel_id("x")
        .message(serde_json::json!({"k": "v"}));
    {
        let (_server, client) = setup(
            "conversations.requestSharedInvite.approve",
            r#"{"ok": true, "invite_id": "I012345ABCD"}"#,
        )
        .await;
        let res = client
            .conversations_request_shared_invite_approve(&req)
            .await
            .expect("conversations.requestSharedInvite.approve");
        assert_preserved(
            "conversations.requestSharedInvite.approve",
            r#"{"ok": true, "invite_id": "I012345ABCD"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_request_shared_invite_deny() {
    let req = ConversationsRequestSharedInviteDenyRequest::new("x").message("x");
    {
        let (_server, client) = setup(
            "conversations.requestSharedInvite.deny",
            r#"{"ok": true, "invite_id": "I012345ABCD"}"#,
        )
        .await;
        let res = client
            .conversations_request_shared_invite_deny(&req)
            .await
            .expect("conversations.requestSharedInvite.deny");
        assert_preserved(
            "conversations.requestSharedInvite.deny",
            r#"{"ok": true, "invite_id": "I012345ABCD"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_request_shared_invite_list() {
    let req = ConversationsRequestSharedInviteListRequest::new()
        .user_id("x")
        .include_expired(true)
        .include_approved(true)
        .include_denied(true)
        .invite_ids(vec!["A1".to_string(), "A2".to_string()])
        .limit(1)
        .cursor("bG9nX2lkOjc5NjQ1NA==");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(
        ConversationsRequestSharedInviteListResponse::default().next_cursor(),
        None
    );
    let page = ConversationsRequestSharedInviteListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("conversations.requestSharedInvite.list", r#"{"ok": true, "invite_requests": [{"date_last_updated": 1722372331, "id": "I12345", "date_created": 1722372331, "expires_at": 1723581931, "inviting_team": {"id": "E12345", "name": "Acme corp", "icon": {"image_34": "https://.../avatar/avatars-teams/ava_0011-34.png", "image_44": "https://.../avatar/avatars-teams/ava_0011-44.png", "image_68": "https://.../avatar/avatars-teams/ava_0011-68.png", "image_88": "https://.../avatar/avatars-teams/ava_0011-88.png", "image_102": "https://.../avatar/avatars-teams/ava_0011-102.png", "image_132": "https://.../avatar/avatars-teams/ava_0011-132.png", "image_230": "https://.../avatar/avatars-teams/ava_0011-230.png", "image_default": true}, "avatar_base_url": "https://dev.slack.com/avatarsource/", "is_verified": false, "domain": "acme-corp", "date_created": 1637947110, "requires_sponsorship": false}, "inviting_user": {"id": "U12345", "team_id": "E12345", "name": "acme-corp-user", "updated": 1721741979, "who_can_share_contact_card": "EVERYONE", "profile": {"real_name": "acme-corp-user", "display_name": "acme-corp-user", "real_name_normalized": "acme-corp-user", "display_name_normalized": "acme-corp-user", "team": "E12345", "avatar_hash": "hash", "email": "acme-corp-user@acme-corp.com", "image_24": "https://secure.gravatar.com/avatar/...0001-24.png", "image_32": "https://secure.gravatar.com/avatar/...0001-32.png", "image_48": "https://secure.gravatar.com/avatar/...0001-48.png", "image_72": "https://secure.gravatar.com/avatar/...0001-72.png", "image_192": "https://secure.gravatar.com/avatar/...0001-192.png", "image_512": "https://secure.gravatar.com/avatar/...0001-512.png"}}, "is_external_limited": false, "is_sponsored": true, "recipient_email": "external-user@other-corp.com", "target_user": {"recipient_email": "external-user1@other-corp.com", "recipient_user_id": "U123456"}}, {"id": "I12345", "date_created": 1722372331, "expires_at": 1723581931, "date_denied": 1723581901, "inviting_team": {"id": "E12345", "name": "Acme Corp.", "icon": {"image_34": "https://.../avatars-teams/ava_0011-34.png", "image_44": "https://.../avatars-teams/ava_0011-44.png", "image_68": "https://.../avatars-teams/ava_0011-68.png", "image_88": "https://.../avatars-teams/ava_0011-88.png", "image_102": "https://.../avatars-teams/ava_0011-102.png", "image_132": "https://.../avatars-teams/ava_0011-132.png", "image_230": "https://.../avatars-teams/ava_0011-230.png", "image_default": true}, "avatar_base_url": "https://dev.slack.com/avatarsource/", "is_verified": false, "domain": "acme-corp", "date_created": 1637947110, "requires_sponsorship": false}, "inviting_user": {"id": "U12345", "team_id": "E12345", "name": "acme-corp-user", "updated": 1721741979, "who_can_share_contact_card": "EVERYONE", "profile": {"real_name": "acme-corp-user", "display_name": "acme-corp-user", "real_name_normalized": "acme-corp-user", "display_name_normalized": "acme-corp-user", "team": "E12345", "avatar_hash": "hash", "email": "acme-corp-user@acme-corp.com", "image_24": "https://secure.gravatar.com/avatar/...0001-24.png", "image_32": "https://secure.gravatar.com/avatar/...0001-32.png", "image_48": "https://secure.gravatar.com/avatar/...0001-48.png", "image_72": "https://secure.gravatar.com/avatar/...0001-72.png", "image_192": "https://secure.gravatar.com/avatar/...0001-192.png", "image_512": "https://secure.gravatar.com/avatar/...0001-512.png"}}, "is_external_limited": false, "channel": {"id": "C12345", "is_im": false, "is_private": true, "date_created": 1721764754, "name": "channel-name", "connections": [{"team": {"id": "E12345", "name": "Acme Corp.", "icon": {"image_34": "https://.../ava_0011-34.png", "image_44": "https://.../ava_0011-44.png", "image_68": "https://.../ava_0011-68.png", "image_88": "https://.../ava_0011-88.png", "image_102": "https://.../ava_0011-102.png", "image_132": "https://.../ava_0011-132.png", "image_230": "https://.../ava_0011-230.png", "image_default": true}, "avatar_base_url": "https://dev.slack.com/avatarsource/", "is_verified": false, "domain": "acme-corp", "date_created": 1637947110, "requires_sponsorship": false}, "is_private": true}], "pending_connections": [], "previous_connections": []}, "target_user": {"recipient_email": "external-user@other-corp.com"}, "reviewing_user": {"id": "U12345", "team_id": "E12345", "name": "acme-corp-user", "updated": 1721741979, "who_can_share_contact_card": "EVERYONE", "profile": {"real_name": "acme-corp-user", "display_name": "acme-corp-user", "real_name_normalized": "acme-corp-user", "display_name_normalized": "acme-corp-user", "team": "E12345", "avatar_hash": "hash", "email": "acme-corp-user@acme-corp.com", "image_24": "https://secure.gravatar.com/avatar/...0001-24.png", "image_32": "https://secure.gravatar.com/avatar/...0001-32.png", "image_48": "https://secure.gravatar.com/avatar/...0001-48.png", "image_72": "https://secure.gravatar.com/avatar/...0001-72.png", "image_192": "https://secure.gravatar.com/avatar/...0001-192.png", "image_512": "https://secure.gravatar.com/avatar/...0001-512.png"}}}], "response_metadata": {"next_cursor": "aWQ6STAxNkszN0FBQUU="}}"#).await;
        let res = client
            .conversations_request_shared_invite_list(&req)
            .await
            .expect("conversations.requestSharedInvite.list");
        assert_preserved(
            "conversations.requestSharedInvite.list",
            r#"{"ok": true, "invite_requests": [{"date_last_updated": 1722372331, "id": "I12345", "date_created": 1722372331, "expires_at": 1723581931, "inviting_team": {"id": "E12345", "name": "Acme corp", "icon": {"image_34": "https://.../avatar/avatars-teams/ava_0011-34.png", "image_44": "https://.../avatar/avatars-teams/ava_0011-44.png", "image_68": "https://.../avatar/avatars-teams/ava_0011-68.png", "image_88": "https://.../avatar/avatars-teams/ava_0011-88.png", "image_102": "https://.../avatar/avatars-teams/ava_0011-102.png", "image_132": "https://.../avatar/avatars-teams/ava_0011-132.png", "image_230": "https://.../avatar/avatars-teams/ava_0011-230.png", "image_default": true}, "avatar_base_url": "https://dev.slack.com/avatarsource/", "is_verified": false, "domain": "acme-corp", "date_created": 1637947110, "requires_sponsorship": false}, "inviting_user": {"id": "U12345", "team_id": "E12345", "name": "acme-corp-user", "updated": 1721741979, "who_can_share_contact_card": "EVERYONE", "profile": {"real_name": "acme-corp-user", "display_name": "acme-corp-user", "real_name_normalized": "acme-corp-user", "display_name_normalized": "acme-corp-user", "team": "E12345", "avatar_hash": "hash", "email": "acme-corp-user@acme-corp.com", "image_24": "https://secure.gravatar.com/avatar/...0001-24.png", "image_32": "https://secure.gravatar.com/avatar/...0001-32.png", "image_48": "https://secure.gravatar.com/avatar/...0001-48.png", "image_72": "https://secure.gravatar.com/avatar/...0001-72.png", "image_192": "https://secure.gravatar.com/avatar/...0001-192.png", "image_512": "https://secure.gravatar.com/avatar/...0001-512.png"}}, "is_external_limited": false, "is_sponsored": true, "recipient_email": "external-user@other-corp.com", "target_user": {"recipient_email": "external-user1@other-corp.com", "recipient_user_id": "U123456"}}, {"id": "I12345", "date_created": 1722372331, "expires_at": 1723581931, "date_denied": 1723581901, "inviting_team": {"id": "E12345", "name": "Acme Corp.", "icon": {"image_34": "https://.../avatars-teams/ava_0011-34.png", "image_44": "https://.../avatars-teams/ava_0011-44.png", "image_68": "https://.../avatars-teams/ava_0011-68.png", "image_88": "https://.../avatars-teams/ava_0011-88.png", "image_102": "https://.../avatars-teams/ava_0011-102.png", "image_132": "https://.../avatars-teams/ava_0011-132.png", "image_230": "https://.../avatars-teams/ava_0011-230.png", "image_default": true}, "avatar_base_url": "https://dev.slack.com/avatarsource/", "is_verified": false, "domain": "acme-corp", "date_created": 1637947110, "requires_sponsorship": false}, "inviting_user": {"id": "U12345", "team_id": "E12345", "name": "acme-corp-user", "updated": 1721741979, "who_can_share_contact_card": "EVERYONE", "profile": {"real_name": "acme-corp-user", "display_name": "acme-corp-user", "real_name_normalized": "acme-corp-user", "display_name_normalized": "acme-corp-user", "team": "E12345", "avatar_hash": "hash", "email": "acme-corp-user@acme-corp.com", "image_24": "https://secure.gravatar.com/avatar/...0001-24.png", "image_32": "https://secure.gravatar.com/avatar/...0001-32.png", "image_48": "https://secure.gravatar.com/avatar/...0001-48.png", "image_72": "https://secure.gravatar.com/avatar/...0001-72.png", "image_192": "https://secure.gravatar.com/avatar/...0001-192.png", "image_512": "https://secure.gravatar.com/avatar/...0001-512.png"}}, "is_external_limited": false, "channel": {"id": "C12345", "is_im": false, "is_private": true, "date_created": 1721764754, "name": "channel-name", "connections": [{"team": {"id": "E12345", "name": "Acme Corp.", "icon": {"image_34": "https://.../ava_0011-34.png", "image_44": "https://.../ava_0011-44.png", "image_68": "https://.../ava_0011-68.png", "image_88": "https://.../ava_0011-88.png", "image_102": "https://.../ava_0011-102.png", "image_132": "https://.../ava_0011-132.png", "image_230": "https://.../ava_0011-230.png", "image_default": true}, "avatar_base_url": "https://dev.slack.com/avatarsource/", "is_verified": false, "domain": "acme-corp", "date_created": 1637947110, "requires_sponsorship": false}, "is_private": true}], "pending_connections": [], "previous_connections": []}, "target_user": {"recipient_email": "external-user@other-corp.com"}, "reviewing_user": {"id": "U12345", "team_id": "E12345", "name": "acme-corp-user", "updated": 1721741979, "who_can_share_contact_card": "EVERYONE", "profile": {"real_name": "acme-corp-user", "display_name": "acme-corp-user", "real_name_normalized": "acme-corp-user", "display_name_normalized": "acme-corp-user", "team": "E12345", "avatar_hash": "hash", "email": "acme-corp-user@acme-corp.com", "image_24": "https://secure.gravatar.com/avatar/...0001-24.png", "image_32": "https://secure.gravatar.com/avatar/...0001-32.png", "image_48": "https://secure.gravatar.com/avatar/...0001-48.png", "image_72": "https://secure.gravatar.com/avatar/...0001-72.png", "image_192": "https://secure.gravatar.com/avatar/...0001-192.png", "image_512": "https://secure.gravatar.com/avatar/...0001-512.png"}}}], "response_metadata": {"next_cursor": "aWQ6STAxNkszN0FBQUU="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_set_purpose() {
    let req =
        ConversationsSetPurposeRequest::new("x", "This is the random channel, anything goes!");
    {
        let (_server, client) = setup("conversations.setPurpose", r#"{"ok": true}"#).await;
        let res = client
            .conversations_set_purpose(&req)
            .await
            .expect("conversations.setPurpose");
        assert_preserved("conversations.setPurpose", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup(
            "conversations.setPurpose",
            r#"{"ok": true, "purpose": "This is the random channel, anything goes!"}"#,
        )
        .await;
        let res = client
            .conversations_set_purpose(&req)
            .await
            .expect("conversations.setPurpose");
        assert_preserved(
            "conversations.setPurpose",
            r#"{"ok": true, "purpose": "This is the random channel, anything goes!"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_set_topic() {
    let req = ConversationsSetTopicRequest::new("x", "Apply topically for best effects");
    {
        let (_server, client) = setup("conversations.setTopic", r#"{"ok": true, "channel": {"id": "C12345678", "name": "tips-and-tricks", "is_channel": true, "is_group": false, "is_im": false, "is_mpim": false, "is_private": false, "created": 1649195947, "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "tips-and-tricks", "is_shared": false, "is_frozen": false, "is_org_shared": false, "is_pending_ext_shared": false, "pending_shared": [], "parent_conversation": null, "creator": "U12345678", "is_ext_shared": false, "shared_team_ids": ["T12345678"], "pending_connected_team_ids": [], "is_member": true, "last_read": "1649869848.627809", "latest": {"type": "message", "subtype": "channel_topic", "ts": "1649952691.429799", "user": "U12345678", "text": "set the channel topic: Apply topically for best effects", "topic": "Apply topically for best effects"}, "unread_count": 1, "unread_count_display": 0, "topic": {"value": "Apply topically for best effects", "creator": "U12345678", "last_set": 1649952691}, "purpose": {"value": "", "creator": "", "last_set": 0}, "previous_names": []}}"#).await;
        let res = client
            .conversations_set_topic(&req)
            .await
            .expect("conversations.setTopic");
        assert_preserved(
            "conversations.setTopic",
            r#"{"ok": true, "channel": {"id": "C12345678", "name": "tips-and-tricks", "is_channel": true, "is_group": false, "is_im": false, "is_mpim": false, "is_private": false, "created": 1649195947, "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "tips-and-tricks", "is_shared": false, "is_frozen": false, "is_org_shared": false, "is_pending_ext_shared": false, "pending_shared": [], "parent_conversation": null, "creator": "U12345678", "is_ext_shared": false, "shared_team_ids": ["T12345678"], "pending_connected_team_ids": [], "is_member": true, "last_read": "1649869848.627809", "latest": {"type": "message", "subtype": "channel_topic", "ts": "1649952691.429799", "user": "U12345678", "text": "set the channel topic: Apply topically for best effects", "topic": "Apply topically for best effects"}, "unread_count": 1, "unread_count_display": 0, "topic": {"value": "Apply topically for best effects", "creator": "U12345678", "last_set": 1649952691}, "purpose": {"value": "", "creator": "", "last_set": 0}, "previous_names": []}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn conversations_unarchive() {
    let req = ConversationsUnarchiveRequest::new("x");
    {
        let (_server, client) = setup("conversations.unarchive", r#"{"ok": true}"#).await;
        let res = client
            .conversations_unarchive(&req)
            .await
            .expect("conversations.unarchive");
        assert_preserved("conversations.unarchive", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn dialog_open() {
    let req = DialogOpenRequest::new("x", "12345.98765.abcd2358fdea");
    {
        let (_server, client) = setup("dialog.open", r#"{"ok": true}"#).await;
        let res = client.dialog_open(&req).await.expect("dialog.open");
        assert_preserved("dialog.open", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn dnd_end_dnd() {
    let req = DndEndDndRequest::new();
    {
        let (_server, client) = setup("dnd.endDnd", r#"{"ok": true}"#).await;
        let res = client.dnd_end_dnd(&req).await.expect("dnd.endDnd");
        assert_preserved("dnd.endDnd", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("dnd.endDnd", r#"{"ok": true}"#).await;
        let res = client.dnd_end_dnd(&req).await.expect("dnd.endDnd");
        assert_preserved("dnd.endDnd", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn dnd_end_snooze() {
    let req = DndEndSnoozeRequest::new();
    {
        let (_server, client) = setup("dnd.endSnooze", r#"{"ok": true}"#).await;
        let res = client.dnd_end_snooze(&req).await.expect("dnd.endSnooze");
        assert_preserved("dnd.endSnooze", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("dnd.endSnooze", r#"{"ok": true, "dnd_enabled": true, "next_dnd_start_ts": 1450418400, "next_dnd_end_ts": 1450454400, "snooze_enabled": false}"#).await;
        let res = client.dnd_end_snooze(&req).await.expect("dnd.endSnooze");
        assert_preserved(
            "dnd.endSnooze",
            r#"{"ok": true, "dnd_enabled": true, "next_dnd_start_ts": 1450418400, "next_dnd_end_ts": 1450454400, "snooze_enabled": false}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn dnd_info() {
    let req = DndInfoRequest::new().user("U1234").team_id("x");
    {
        let (_server, client) = setup("dnd.info", r#"{"ok": true}"#).await;
        let res = client.dnd_info(&req).await.expect("dnd.info");
        assert_preserved("dnd.info", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn dnd_set_snooze() {
    let req = DndSetSnoozeRequest::new("60");
    {
        let (_server, client) = setup("dnd.setSnooze", r#"{"ok": true, "snooze_enabled": true, "snooze_endtime": 1450373897, "snooze_remaining": 60, "snooze_is_indefinite": false}"#).await;
        let res = client.dnd_set_snooze(&req).await.expect("dnd.setSnooze");
        assert_preserved(
            "dnd.setSnooze",
            r#"{"ok": true, "snooze_enabled": true, "snooze_endtime": 1450373897, "snooze_remaining": 60, "snooze_is_indefinite": false}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn dnd_team_info() {
    let req = DndTeamInfoRequest::new("U1234,W4567").team_id("x");
    {
        let (_server, client) = setup("dnd.teamInfo", r#"{"ok": true, "users": {"U023BECGF": {"dnd_enabled": true, "next_dnd_start_ts": 1450387800, "next_dnd_end_ts": 1450423800}, "W058CJVAA": {"dnd_enabled": false, "next_dnd_start_ts": 1, "next_dnd_end_ts": 1}}}"#).await;
        let res = client.dnd_team_info(&req).await.expect("dnd.teamInfo");
        assert_preserved(
            "dnd.teamInfo",
            r#"{"ok": true, "users": {"U023BECGF": {"dnd_enabled": true, "next_dnd_start_ts": 1450387800, "next_dnd_end_ts": 1450423800}, "W058CJVAA": {"dnd_enabled": false, "next_dnd_start_ts": 1, "next_dnd_end_ts": 1}}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn emoji_list() {
    let req = EmojiListRequest::new().include_categories(true);
    {
        let (_server, client) = setup("emoji.list", r#"{"ok": true}"#).await;
        let res = client.emoji_list(&req).await.expect("emoji.list");
        assert_preserved("emoji.list", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("emoji.list", r#"{"ok": true, "emoji": {"bowtie": "https://emoji.slack-edge.com/T9TK3CUKW/bowtie/f3ec6f2bb0.png", "squirrel": "https://emoji.slack-edge.com/T9TK3CUKW/squirrel/465f40c0e0.png", "shipit": "alias:squirrel"}, "cache_ts": "1575283387.000000"}"#).await;
        let res = client.emoji_list(&req).await.expect("emoji.list");
        assert_preserved(
            "emoji.list",
            r#"{"ok": true, "emoji": {"bowtie": "https://emoji.slack-edge.com/T9TK3CUKW/bowtie/f3ec6f2bb0.png", "squirrel": "https://emoji.slack-edge.com/T9TK3CUKW/squirrel/465f40c0e0.png", "shipit": "alias:squirrel"}, "cache_ts": "1575283387.000000"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn entity_acknowledge_comment_action() {
    let req = EntityAcknowledgeCommentActionRequest::new("x")
        .comment(serde_json::json!({"k": "v"}))
        .error("x");
    {
        let (_server, client) = setup("entity.acknowledgeCommentAction", r#"{"ok": true}"#).await;
        let res = client
            .entity_acknowledge_comment_action(&req)
            .await
            .expect("entity.acknowledgeCommentAction");
        assert_preserved("entity.acknowledgeCommentAction", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn entity_present_comments() {
    let req = EntityPresentCommentsRequest::new(vec![serde_json::json!({"k": "v"})], "x")
        .cursor("x")
        .can_post_comment(true)
        .delete_action_id("x")
        .user_auth_required(true)
        .user_auth_url("https://example.com/onboarding?user_id=xxx")
        .error("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(EntityPresentCommentsResponse::default().next_cursor(), None);
    let page = EntityPresentCommentsResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("entity.presentComments", r#"{"ok": true}"#).await;
        let res = client
            .entity_present_comments(&req)
            .await
            .expect("entity.presentComments");
        assert_preserved("entity.presentComments", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn entity_present_details() {
    let req = EntityPresentDetailsRequest::new("x")
        .metadata(serde_json::json!({"k": "v"}))
        .user_auth_required(true)
        .user_auth_url("https://example.com/onboarding?user_id=xxx")
        .error("x");
    {
        let (_server, client) = setup("entity.presentDetails", r#"{"ok": true}"#).await;
        let res = client
            .entity_present_details(&req)
            .await
            .expect("entity.presentDetails");
        assert_preserved("entity.presentDetails", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_comments_delete() {
    let req = FilesCommentsDeleteRequest::new("F1234567890", "Fc1234567890");
    {
        let (_server, client) = setup("files.comments.delete", r#"{"ok": true}"#).await;
        let res = client
            .files_comments_delete(&req)
            .await
            .expect("files.comments.delete");
        assert_preserved("files.comments.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_complete_upload_external() {
    let req = FilesCompleteUploadExternalRequest::new(vec![serde_json::json!({"k": "v"})])
        .channel_id("C0NF841BK")
        .thread_ts("1524523204.000192")
        .channels("C0NF841BK,C2AW648GH")
        .initial_comment("x")
        .blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .username("My Bot")
        .icon_url("http://lorempixel.com/48/48")
        .icon_emoji(":chart_with_upwards_trend:");
    {
        let (_server, client) = setup(
            "files.completeUploadExternal",
            r#"{"ok": true, "files": [{"id": "F123ABC456", "title": "slack-test"}]}"#,
        )
        .await;
        let res = client
            .files_complete_upload_external(&req)
            .await
            .expect("files.completeUploadExternal");
        assert_preserved(
            "files.completeUploadExternal",
            r#"{"ok": true, "files": [{"id": "F123ABC456", "title": "slack-test"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_delete() {
    let req = FilesDeleteRequest::new("x");
    {
        let (_server, client) = setup("files.delete", r#"{"ok": true}"#).await;
        let res = client.files_delete(&req).await.expect("files.delete");
        assert_preserved("files.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_get_upload_url_external() {
    let req = FilesGetUploadUrlExternalRequest::new(1, "laughingoutloudcat.jpg")
        .snippet_type("python")
        .alt_txt("Aerial view of the Bixby Bridge and coastline of the Big Sur area in California");
    {
        let (_server, client) = setup("files.getUploadURLExternal", r#"{"ok": true, "upload_url": "https://files.slack.com/upload/v1/ABC123...", "file_id": "F123ABC456"}"#).await;
        let res = client
            .files_get_upload_url_external(&req)
            .await
            .expect("files.getUploadURLExternal");
        assert_preserved(
            "files.getUploadURLExternal",
            r#"{"ok": true, "upload_url": "https://files.slack.com/upload/v1/ABC123...", "file_id": "F123ABC456"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_info() {
    let req = FilesInfoRequest::new("F2147483862")
        .count(1)
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1)
        .page(1);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(FilesInfoResponse::default().next_cursor(), None);
    let page = FilesInfoResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("files.info", r#"{"ok": true, "file": {"id": "F0S43PZDF", "created": 1531763342, "timestamp": 1531763342, "name": "tedair.gif", "title": "tedair.gif", "mimetype": "image/gif", "filetype": "gif", "pretty_type": "GIF", "user": "U061F7AUR", "editable": false, "size": 137531, "mode": "hosted", "is_external": false, "external_type": "", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://.../tedair.gif", "url_private_download": "https://.../tedair.gif", "thumb_64": "https://.../tedair_64.png", "thumb_80": "https://.../tedair_80.png", "thumb_360": "https://.../tedair_360.png", "thumb_360_w": 176, "thumb_360_h": 226, "thumb_160": "https://.../tedair_=_160.png", "thumb_360_gif": "https://.../tedair_360.gif", "image_exif_rotation": 1, "original_w": 176, "original_h": 226, "deanimate_gif": "https://.../tedair_deanimate_gif.png", "pjpeg": "https://.../tedair_pjpeg.jpg", "permalink": "https://.../tedair.gif", "permalink_public": "https://.../...", "comments_count": 0, "is_starred": false, "shares": {"public": {"C0T8SE4AU": [{"reply_users": ["U061F7AUR"], "reply_users_count": 1, "reply_count": 1, "ts": "1531763348.000001", "thread_ts": "1531763273.000015", "latest_reply": "1531763348.000001", "channel_name": "file-under", "team_id": "T061EG9R6"}]}}, "channels": ["C0T8SE4AU"], "groups": [], "ims": [], "has_rich_preview": false, "alt_txt": "tedair.gif"}, "comments": [], "response_metadata": {"next_cursor": "dGVhbTpDMUg5UkVTR0w="}}"#).await;
        let res = client.files_info(&req).await.expect("files.info");
        assert_preserved(
            "files.info",
            r#"{"ok": true, "file": {"id": "F0S43PZDF", "created": 1531763342, "timestamp": 1531763342, "name": "tedair.gif", "title": "tedair.gif", "mimetype": "image/gif", "filetype": "gif", "pretty_type": "GIF", "user": "U061F7AUR", "editable": false, "size": 137531, "mode": "hosted", "is_external": false, "external_type": "", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://.../tedair.gif", "url_private_download": "https://.../tedair.gif", "thumb_64": "https://.../tedair_64.png", "thumb_80": "https://.../tedair_80.png", "thumb_360": "https://.../tedair_360.png", "thumb_360_w": 176, "thumb_360_h": 226, "thumb_160": "https://.../tedair_=_160.png", "thumb_360_gif": "https://.../tedair_360.gif", "image_exif_rotation": 1, "original_w": 176, "original_h": 226, "deanimate_gif": "https://.../tedair_deanimate_gif.png", "pjpeg": "https://.../tedair_pjpeg.jpg", "permalink": "https://.../tedair.gif", "permalink_public": "https://.../...", "comments_count": 0, "is_starred": false, "shares": {"public": {"C0T8SE4AU": [{"reply_users": ["U061F7AUR"], "reply_users_count": 1, "reply_count": 1, "ts": "1531763348.000001", "thread_ts": "1531763273.000015", "latest_reply": "1531763348.000001", "channel_name": "file-under", "team_id": "T061EG9R6"}]}}, "channels": ["C0T8SE4AU"], "groups": [], "ims": [], "has_rich_preview": false, "alt_txt": "tedair.gif"}, "comments": [], "response_metadata": {"next_cursor": "dGVhbTpDMUg5UkVTR0w="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_list() {
    let req = FilesListRequest::new()
        .channel("x")
        .count(1)
        .page(1)
        .show_files_hidden_by_limit(true)
        .team_id("x")
        .ts_from("123456789")
        .ts_to("123456789")
        .types("images")
        .user("x");
    {
        let (_server, client) = setup("files.list", r#"{"ok": true, "files": [{"id": "F0S43P1CZ", "created": 1531763254, "timestamp": 1531763254, "name": "billair.gif", "title": "billair.gif", "mimetype": "image/gif", "filetype": "gif", "pretty_type": "GIF", "user": "U061F7AUR", "editable": false, "size": 144538, "mode": "hosted", "is_external": false, "external_type": "", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://.../billair.gif", "url_private_download": "https://.../billair.gif", "thumb_64": "https://.../billair_64.png", "thumb_80": "https://.../billair_80.png", "thumb_360": "https://.../billair_360.png", "thumb_360_w": 176, "thumb_360_h": 226, "thumb_160": "https://.../billair_=_160.png", "thumb_360_gif": "https://.../billair_360.gif", "image_exif_rotation": 1, "original_w": 176, "original_h": 226, "deanimate_gif": "https://.../billair_deanimate_gif.png", "pjpeg": "https://.../billair_pjpeg.jpg", "permalink": "https://.../billair.gif", "permalink_public": "https://.../...", "channels": ["C0T8SE4AU"], "groups": [], "ims": [], "comments_count": 0}, {"id": "F0S43PZDF", "created": 1531763342, "timestamp": 1531763342, "name": "tedair.gif", "title": "tedair.gif", "mimetype": "image/gif", "filetype": "gif", "pretty_type": "GIF", "user": "U061F7AUR", "editable": false, "size": 137531, "mode": "hosted", "is_external": false, "external_type": "", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://.../tedair.gif", "url_private_download": "https://.../tedair.gif", "thumb_64": "https://.../tedair_64.png", "thumb_80": "https://.../tedair_80.png", "thumb_360": "https://.../tedair_360.png", "thumb_360_w": 176, "thumb_360_h": 226, "thumb_160": "https://.../tedair_=_160.png", "thumb_360_gif": "https://.../tedair_360.gif", "image_exif_rotation": 1, "original_w": 176, "original_h": 226, "deanimate_gif": "https://.../tedair_deanimate_gif.png", "pjpeg": "https://.../tedair_pjpeg.jpg", "permalink": "https://.../tedair.gif", "permalink_public": "https://.../...", "channels": ["C0T8SE4AU"], "groups": [], "ims": [], "comments_count": 0}], "paging": {"count": 100, "total": 2, "page": 1, "pages": 1}}"#).await;
        let res = client.files_list(&req).await.expect("files.list");
        assert_preserved(
            "files.list",
            r#"{"ok": true, "files": [{"id": "F0S43P1CZ", "created": 1531763254, "timestamp": 1531763254, "name": "billair.gif", "title": "billair.gif", "mimetype": "image/gif", "filetype": "gif", "pretty_type": "GIF", "user": "U061F7AUR", "editable": false, "size": 144538, "mode": "hosted", "is_external": false, "external_type": "", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://.../billair.gif", "url_private_download": "https://.../billair.gif", "thumb_64": "https://.../billair_64.png", "thumb_80": "https://.../billair_80.png", "thumb_360": "https://.../billair_360.png", "thumb_360_w": 176, "thumb_360_h": 226, "thumb_160": "https://.../billair_=_160.png", "thumb_360_gif": "https://.../billair_360.gif", "image_exif_rotation": 1, "original_w": 176, "original_h": 226, "deanimate_gif": "https://.../billair_deanimate_gif.png", "pjpeg": "https://.../billair_pjpeg.jpg", "permalink": "https://.../billair.gif", "permalink_public": "https://.../...", "channels": ["C0T8SE4AU"], "groups": [], "ims": [], "comments_count": 0}, {"id": "F0S43PZDF", "created": 1531763342, "timestamp": 1531763342, "name": "tedair.gif", "title": "tedair.gif", "mimetype": "image/gif", "filetype": "gif", "pretty_type": "GIF", "user": "U061F7AUR", "editable": false, "size": 137531, "mode": "hosted", "is_external": false, "external_type": "", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://.../tedair.gif", "url_private_download": "https://.../tedair.gif", "thumb_64": "https://.../tedair_64.png", "thumb_80": "https://.../tedair_80.png", "thumb_360": "https://.../tedair_360.png", "thumb_360_w": 176, "thumb_360_h": 226, "thumb_160": "https://.../tedair_=_160.png", "thumb_360_gif": "https://.../tedair_360.gif", "image_exif_rotation": 1, "original_w": 176, "original_h": 226, "deanimate_gif": "https://.../tedair_deanimate_gif.png", "pjpeg": "https://.../tedair_pjpeg.jpg", "permalink": "https://.../tedair.gif", "permalink_public": "https://.../...", "channels": ["C0T8SE4AU"], "groups": [], "ims": [], "comments_count": 0}], "paging": {"count": 100, "total": 2, "page": 1, "pages": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_remote_add() {
    let req = FilesRemoteAddRequest::new(
        "123456",
        "http://example.com/my_cloud_service_file/abc123",
        "Danger, High Voltage!",
    )
    .filetype("doc")
    .indexable_file_contents("...")
    .preview_image("...");
    {
        let (_server, client) = setup("files.remote.add", r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740062388, "timestamp": 1740062388, "name": "Test", "title": "Test", "mimetype": "application/vnd.slack-remote", "filetype": "remote", "pretty_type": "Remote", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {}, "channels": [], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#).await;
        let res = client
            .files_remote_add(&req)
            .await
            .expect("files.remote.add");
        assert_preserved(
            "files.remote.add",
            r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740062388, "timestamp": 1740062388, "name": "Test", "title": "Test", "mimetype": "application/vnd.slack-remote", "filetype": "remote", "pretty_type": "Remote", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {}, "channels": [], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_remote_info() {
    let req = FilesRemoteInfoRequest::new()
        .external_id("123456")
        .file("F2147483862");
    {
        let (_server, client) = setup("files.remote.info", r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740062388, "timestamp": 1740062388, "name": "Test", "title": "Test", "mimetype": "application/vnd.slack-remote", "filetype": "remote", "pretty_type": "Remote", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {}, "channels": [], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#).await;
        let res = client
            .files_remote_info(&req)
            .await
            .expect("files.remote.info");
        assert_preserved(
            "files.remote.info",
            r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740062388, "timestamp": 1740062388, "name": "Test", "title": "Test", "mimetype": "application/vnd.slack-remote", "filetype": "remote", "pretty_type": "Remote", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {}, "channels": [], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_remote_list() {
    let req = FilesRemoteListRequest::new()
        .channel("x")
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1)
        .ts_from("123456789")
        .ts_to("123456789");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(FilesRemoteListResponse::default().next_cursor(), None);
    let page = FilesRemoteListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup(
            "files.remote.list",
            r#"{"ok": true, "files": [], "response_metadata": {"next_cursor": ""}}"#,
        )
        .await;
        let res = client
            .files_remote_list(&req)
            .await
            .expect("files.remote.list");
        assert_preserved(
            "files.remote.list",
            r#"{"ok": true, "files": [], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_remote_remove() {
    let req = FilesRemoteRemoveRequest::new()
        .external_id("123456")
        .file("F2147483862");
    {
        let (_server, client) = setup("files.remote.remove", r#"{"ok": true}"#).await;
        let res = client
            .files_remote_remove(&req)
            .await
            .expect("files.remote.remove");
        assert_preserved("files.remote.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_remote_share() {
    let req = FilesRemoteShareRequest::new("x")
        .external_id("123456")
        .file("F2147483862");
    {
        let (_server, client) = setup("files.remote.share", r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740066121, "timestamp": 1740066121, "name": "Test", "title": "Test", "mimetype": "application/vnd.slack-remote", "filetype": "remote", "pretty_type": "Remote", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {"public": {"C03QJUTKS4C": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.886799", "channel_name": "the-test-channel", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}], "C04567YFDK6": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.650279", "channel_name": "dev-test", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}]}}, "channels": ["C12ABCDEF3G", "C12345ABCD6"], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#).await;
        let res = client
            .files_remote_share(&req)
            .await
            .expect("files.remote.share");
        assert_preserved(
            "files.remote.share",
            r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740066121, "timestamp": 1740066121, "name": "Test", "title": "Test", "mimetype": "application/vnd.slack-remote", "filetype": "remote", "pretty_type": "Remote", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {"public": {"C03QJUTKS4C": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.886799", "channel_name": "the-test-channel", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}], "C04567YFDK6": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.650279", "channel_name": "dev-test", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}]}}, "channels": ["C12ABCDEF3G", "C12345ABCD6"], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_remote_update() {
    let req = FilesRemoteUpdateRequest::new()
        .external_id("123456")
        .external_url("http://example.com/my_cloud_service_file/abc123")
        .file("F2147483862")
        .filetype("doc")
        .indexable_file_contents("...")
        .preview_image("...")
        .title("Danger, High Voltage!");
    {
        let (_server, client) = setup("files.remote.update", r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740066121, "timestamp": 1740066121, "name": "Test", "title": "Test", "mimetype": "application/msword", "filetype": "doc", "pretty_type": "Word Document", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {"public": {"C03QJUTKS4C": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.886799", "channel_name": "the-test-channel", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}], "C04567YFDK6": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.650279", "channel_name": "dev-test", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}]}}, "channels": ["C12ABCDEF3G", "C12345ABCD6"], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#).await;
        let res = client
            .files_remote_update(&req)
            .await
            .expect("files.remote.update");
        assert_preserved(
            "files.remote.update",
            r#"{"ok": true, "file": {"id": "F08EAQ813FW", "created": 1740066121, "timestamp": 1740066121, "name": "Test", "title": "Test", "mimetype": "application/msword", "filetype": "doc", "pretty_type": "Word Document", "user": "U123A4BCDE5", "user_team": "T123A4BC5DE", "editable": false, "size": 0, "mode": "external", "is_external": true, "external_type": "app", "is_public": true, "public_url_shared": false, "display_as_bot": false, "username": "", "url_private": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "media_display_type": "unknown", "permalink": "https://thetestenv.slack.com/files/U123A4BCDE5/F08EAQ813FW/test", "comments_count": 0, "is_starred": false, "shares": {"public": {"C03QJUTKS4C": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.886799", "channel_name": "the-test-channel", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}], "C04567YFDK6": [{"reply_users": [], "reply_users_count": 0, "reply_count": 0, "ts": "1740066131.650279", "channel_name": "dev-test", "team_id": "T123A4BC5DE", "share_user_id": "U123A4BCDE5", "source": "UNKNOWN"}]}}, "channels": ["C12ABCDEF3G", "C12345ABCD6"], "groups": [], "ims": [], "has_more_shares": false, "external_id": "1234", "external_url": "https://docs.google.com/document/d/1e8LtkvCSe_NH0UU0RyLgssmUQLT8G_3RMCGzyPWcx58/edit?tab=t.0", "has_rich_preview": false, "file_access": "visible"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_revoke_public_url() {
    let req = FilesRevokePublicUrlRequest::new("x");
    {
        let (_server, client) = setup("files.revokePublicURL", r#"{"ok": true}"#).await;
        let res = client
            .files_revoke_public_url(&req)
            .await
            .expect("files.revokePublicURL");
        assert_preserved("files.revokePublicURL", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn files_shared_public_url() {
    let req = FilesSharedPublicUrlRequest::new("x");
    {
        let (_server, client) = setup("files.sharedPublicURL", r#"{"ok": true}"#).await;
        let res = client
            .files_shared_public_url(&req)
            .await
            .expect("files.sharedPublicURL");
        assert_preserved("files.sharedPublicURL", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_complete_error() {
    let req = FunctionsCompleteErrorRequest::new("Fx12345ABCDE", "x");
    {
        let (_server, client) = setup("functions.completeError", r#"{"ok": true}"#).await;
        let res = client
            .functions_complete_error(&req)
            .await
            .expect("functions.completeError");
        assert_preserved("functions.completeError", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_complete_success() {
    let req = FunctionsCompleteSuccessRequest::new("Fx12345ABCDE", serde_json::json!({"k": "v"}));
    {
        let (_server, client) = setup("functions.completeSuccess", r#"{"ok": true}"#).await;
        let res = client
            .functions_complete_success(&req)
            .await
            .expect("functions.completeSuccess");
        assert_preserved("functions.completeSuccess", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_distributions_permissions_add() {
    let req = FunctionsDistributionsPermissionsAddRequest::new()
        .function_id("Fn12345")
        .function_callback_id("my_function")
        .function_app_id("A12345")
        .user_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("functions.distributions.permissions.add", r#"{"ok": true, "permission_type": "named_entities", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#).await;
        let res = client
            .functions_distributions_permissions_add(&req)
            .await
            .expect("functions.distributions.permissions.add");
        assert_preserved(
            "functions.distributions.permissions.add",
            r#"{"ok": true, "permission_type": "named_entities", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_distributions_permissions_list() {
    let req = FunctionsDistributionsPermissionsListRequest::new()
        .function_id("Fn12345")
        .function_callback_id("my_function")
        .function_app_id("A12345");
    {
        let (_server, client) = setup("functions.distributions.permissions.list", r#"{"ok": true, "permission_type": "app_collaborators", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#).await;
        let res = client
            .functions_distributions_permissions_list(&req)
            .await
            .expect("functions.distributions.permissions.list");
        assert_preserved(
            "functions.distributions.permissions.list",
            r#"{"ok": true, "permission_type": "app_collaborators", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_distributions_permissions_remove() {
    let req = FunctionsDistributionsPermissionsRemoveRequest::new()
        .function_id("Fn12345")
        .function_callback_id("my_function")
        .function_app_id("A12345")
        .user_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("functions.distributions.permissions.remove", r#"{"ok": true, "permission_type": "named_entities", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#).await;
        let res = client
            .functions_distributions_permissions_remove(&req)
            .await
            .expect("functions.distributions.permissions.remove");
        assert_preserved(
            "functions.distributions.permissions.remove",
            r#"{"ok": true, "permission_type": "named_entities", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_distributions_permissions_set() {
    let req = FunctionsDistributionsPermissionsSetRequest::new()
        .function_id("Fn12345")
        .function_callback_id("my_function")
        .function_app_id("A12345")
        .permission_type("x")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .team_ids(vec!["A1".to_string(), "A2".to_string()])
        .org_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("functions.distributions.permissions.set", r#"{"ok": true, "permission_type": "app_collaborators", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#).await;
        let res = client
            .functions_distributions_permissions_set(&req)
            .await
            .expect("functions.distributions.permissions.set");
        assert_preserved(
            "functions.distributions.permissions.set",
            r#"{"ok": true, "permission_type": "app_collaborators", "users": [{"user_id": "U01565LTEBD", "username": "joe_smith", "email": "joesmith@salesforce.com"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_workflows_steps_list() {
    let req = FunctionsWorkflowsStepsListRequest::new("x")
        .workflow_id("x")
        .workflow("#/workflows/my-workflow")
        .workflow_app_id("x");
    {
        let (_server, client) = setup("functions.workflows.steps.list", r#"{"ok": true, "steps_versions": [{"title": "Send a greeting", "workflow_id": "Wf014H7FCWG2", "step_id": "0", "is_deleted": false, "workflow_version_created": "1677282339978193"}]}"#).await;
        let res = client
            .functions_workflows_steps_list(&req)
            .await
            .expect("functions.workflows.steps.list");
        assert_preserved(
            "functions.workflows.steps.list",
            r#"{"ok": true, "steps_versions": [{"title": "Send a greeting", "workflow_id": "Wf014H7FCWG2", "step_id": "0", "is_deleted": false, "workflow_version_created": "1677282339978193"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn functions_workflows_steps_responses_export() {
    let req = FunctionsWorkflowsStepsResponsesExportRequest::new("x")
        .workflow_id("x")
        .workflow("#/workflows/my-workflow")
        .workflow_app_id("x");
    {
        let (_server, client) = setup(
            "functions.workflows.steps.responses.export",
            r#"{"ok": true}"#,
        )
        .await;
        let res = client
            .functions_workflows_steps_responses_export(&req)
            .await
            .expect("functions.workflows.steps.responses.export");
        assert_preserved(
            "functions.workflows.steps.responses.export",
            r#"{"ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn migration_exchange() {
    let req = MigrationExchangeRequest::new(vec![serde_json::json!({"k": "v"})])
        .team_id("x")
        .to_old(true);
    {
        let (_server, client) = setup("migration.exchange", r#"{"ok": true, "team_id": "T1KR7PE1W", "enterprise_id": "E1KQTNXE1", "user_id_map": {"U06UBSUN5": "W06M56XJM", "U06UEB62U": "W06PTT6GH", "U06UBSVB3": "W06PUUDLY", "U06UBSVDX": "W06PUUDMW", "W06UAZ65Q": "W06UAZ65Q"}, "invalid_user_ids": ["U21ABZZXX"]}"#).await;
        let res = client
            .migration_exchange(&req)
            .await
            .expect("migration.exchange");
        assert_preserved(
            "migration.exchange",
            r#"{"ok": true, "team_id": "T1KR7PE1W", "enterprise_id": "E1KQTNXE1", "user_id_map": {"U06UBSUN5": "W06M56XJM", "U06UEB62U": "W06PTT6GH", "U06UBSVB3": "W06PUUDLY", "U06UBSVDX": "W06PUUDMW", "W06UAZ65Q": "W06UAZ65Q"}, "invalid_user_ids": ["U21ABZZXX"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn oauth_access() {
    let req = OauthAccessRequest::new()
        .client_id("2141029472.691202649728")
        .client_secret("e1b9e11dfcd19c1982d5de12921e17e8c")
        .code("4724469134.4644010092847.232b4e6d82c333b475fc30f5f5a341d294feb1a94392c2fd791f7ab7731a443d1a")
        .redirect_uri("http://example.com")
        .single_channel(true);
    {
        let (_server, client) = setup("oauth.access", r#"{"ok": true, "access_token": "xoxa-EXAMPLE", "token_type": "app", "app_id": "A012345678", "app_user_id": "U0NKHRW57", "team_name": "Subarachnoid Workspace", "team_id": "T061EG9R6", "enterprise_id": null, "authorizing_user": {"user_id": "U061F7AUR", "app_home": "D0PNCRP9N"}, "installer_user": {"user_id": "U061F7AUR", "app_home": "D0PNCRP9N"}, "scopes": {"app_home": ["chat:write", "im:history", "im:read"], "team": [], "channel": ["channels:history", "channels:read", "chat:write"], "group": ["chat:write"], "mpim": ["chat:write"], "im": ["chat:write"], "user": []}}"#).await;
        let res = client.oauth_access(&req).await.expect("oauth.access");
        assert_preserved(
            "oauth.access",
            r#"{"ok": true, "access_token": "xoxa-EXAMPLE", "token_type": "app", "app_id": "A012345678", "app_user_id": "U0NKHRW57", "team_name": "Subarachnoid Workspace", "team_id": "T061EG9R6", "enterprise_id": null, "authorizing_user": {"user_id": "U061F7AUR", "app_home": "D0PNCRP9N"}, "installer_user": {"user_id": "U061F7AUR", "app_home": "D0PNCRP9N"}, "scopes": {"app_home": ["chat:write", "im:history", "im:read"], "team": [], "channel": ["channels:history", "channels:read", "chat:write"], "group": ["chat:write"], "mpim": ["chat:write"], "im": ["chat:write"], "user": []}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn oauth_v2_access() {
    let req = OauthV2AccessRequest::new()
        .client_id("2141029472.691202649728")
        .client_secret("e1b9e11dfcd19c1982d5de12921e17e8c")
        .code("4724469134.4644010092847.232b4e6d82c333b475fc30f5f5a341d294feb1a94392c2fd791f7ab7731a443d1a")
        .code_verifier("secret12345")
        .redirect_uri("http://example.com")
        .grant_type("authorization_code")
        .refresh_token("xoxe-EXAMPLE")
        .assertion("eyJhbGciOiJSUzI1NiIsInR…");
    {
        let (_server, client) = setup("oauth.v2.access", r#"{"ok": true, "access_token": "xoxb-EXAMPLE", "token_type": "bot", "scope": "commands,incoming-webhook", "bot_user_id": "U0KRQLJ9H", "app_id": "A0KRD7HC3", "team": {"name": "Slack Softball Team", "id": "T9TK3CUKW"}, "enterprise": {"name": "slack-sports", "id": "E12345678"}, "authed_user": {"id": "U1234", "scope": "chat:write", "access_token": "xoxp-EXAMPLE", "token_type": "user"}}"#).await;
        let res = client.oauth_v2_access(&req).await.expect("oauth.v2.access");
        assert_preserved(
            "oauth.v2.access",
            r#"{"ok": true, "access_token": "xoxb-EXAMPLE", "token_type": "bot", "scope": "commands,incoming-webhook", "bot_user_id": "U0KRQLJ9H", "app_id": "A0KRD7HC3", "team": {"name": "Slack Softball Team", "id": "T9TK3CUKW"}, "enterprise": {"name": "slack-sports", "id": "E12345678"}, "authed_user": {"id": "U1234", "scope": "chat:write", "access_token": "xoxp-EXAMPLE", "token_type": "user"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("oauth.v2.access", r#"{"ok": true, "access_token": "xoxe.xoxb-EXAMPLE..", "token_type": "bot", "scope": "commands,incoming-webhook", "bot_user_id": "U0KRQLJ9H", "app_id": "A0KRD7HC3", "expires_in": 43200, "refresh_token": "xoxe-EXAMPLE...", "team": {"name": "Slack Softball Team", "id": "T9TK3CUKW"}, "enterprise": {"name": "slack-sports", "id": "E12345678"}, "authed_user": {"id": "U1234", "scope": "chat:write", "access_token": "xoxe.xoxp-EXAMPLE", "expires_in": 43200, "refresh_token": "xoxe-EXAMPLE...", "token_type": "user"}}"#).await;
        let res = client.oauth_v2_access(&req).await.expect("oauth.v2.access");
        assert_preserved(
            "oauth.v2.access",
            r#"{"ok": true, "access_token": "xoxe.xoxb-EXAMPLE..", "token_type": "bot", "scope": "commands,incoming-webhook", "bot_user_id": "U0KRQLJ9H", "app_id": "A0KRD7HC3", "expires_in": 43200, "refresh_token": "xoxe-EXAMPLE...", "team": {"name": "Slack Softball Team", "id": "T9TK3CUKW"}, "enterprise": {"name": "slack-sports", "id": "E12345678"}, "authed_user": {"id": "U1234", "scope": "chat:write", "access_token": "xoxe.xoxp-EXAMPLE", "expires_in": 43200, "refresh_token": "xoxe-EXAMPLE...", "token_type": "user"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("oauth.v2.access", r#"{"ok": true, "app_id": "A0118NQPZZC", "authed_user": {"id": "U065VRX1T0", "scope": "identity.basic,identity.email,identity.avatar,identity.team", "access_token": "xoxp-EXAMPLE", "token_type": "user"}, "team": {"id": "T024BE7LD"}, "enterprise": null, "is_enterprise_install": false}"#).await;
        let res = client.oauth_v2_access(&req).await.expect("oauth.v2.access");
        assert_preserved(
            "oauth.v2.access",
            r#"{"ok": true, "app_id": "A0118NQPZZC", "authed_user": {"id": "U065VRX1T0", "scope": "identity.basic,identity.email,identity.avatar,identity.team", "access_token": "xoxp-EXAMPLE", "token_type": "user"}, "team": {"id": "T024BE7LD"}, "enterprise": null, "is_enterprise_install": false}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn oauth_v2_begin_short_token_rotation() {
    let req = OauthV2BeginShortTokenRotationRequest::new(
        "4123121235.9872358710",
        "e1b9e11dfcd19c1982d5de12921e17e8c",
    );
    {
        let (_server, client) = setup(
            "oauth.v2.beginShortTokenRotation",
            r#"{"ok": true, "new_token": "xoxp-EXAMPLE"}"#,
        )
        .await;
        let res = client
            .oauth_v2_begin_short_token_rotation(&req)
            .await
            .expect("oauth.v2.beginShortTokenRotation");
        assert_preserved(
            "oauth.v2.beginShortTokenRotation",
            r#"{"ok": true, "new_token": "xoxp-EXAMPLE"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn oauth_v2_complete_short_token_rotation() {
    let req = OauthV2CompleteShortTokenRotationRequest::new(
        "4123121235.9872358710",
        "e1b9e11dfcd19c1982d5de12921e17e8c",
        "xoxp-EXAMPLE",
    );
    {
        let (_server, client) = setup(
            "oauth.v2.completeShortTokenRotation",
            r#"{"ok": true, "token": "xoxp-EXAMPLE"}"#,
        )
        .await;
        let res = client
            .oauth_v2_complete_short_token_rotation(&req)
            .await
            .expect("oauth.v2.completeShortTokenRotation");
        assert_preserved(
            "oauth.v2.completeShortTokenRotation",
            r#"{"ok": true, "token": "xoxp-EXAMPLE"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn oauth_v2_exchange() {
    let req =
        OauthV2ExchangeRequest::new("4123121235.9872358710", "e1b9e11dfcd19c1982d5de12921e17e8c");
    {
        let (_server, client) = setup("oauth.v2.exchange", r#"{"ok": true}"#).await;
        let res = client
            .oauth_v2_exchange(&req)
            .await
            .expect("oauth.v2.exchange");
        assert_preserved("oauth.v2.exchange", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn oauth_v2_user_access() {
    let req = OauthV2UserAccessRequest::new()
        .client_id("2141029472.691202649728")
        .client_secret("e1b9e11dfcd19c1982d5de12921e17e8c")
        .code("4724469134.4644010092847.232b4e6d82c333b475fc30f5f5a341d294feb1a94392c2fd791f7ab7731a443d1a")
        .code_verifier("secret12345")
        .redirect_uri("http://example.com")
        .grant_type("authorization_code")
        .refresh_token("xoxe-EXAMPLE")
        .assertion("eyJhbGciOiJSUzI1NiIsInR…");
    {
        let (_server, client) = setup("oauth.v2.user.access", r#"{"ok": true, "access_token": "xoxp-EXAMPLE...", "token_type": "user", "id_token": "eyJhbGciOiJSUzI1Ni...", "authed_user": {"id": "U12345", "scope": "openid,email,profile"}, "team": {"id": "T012345"}}"#).await;
        let res = client
            .oauth_v2_user_access(&req)
            .await
            .expect("oauth.v2.user.access");
        assert_preserved(
            "oauth.v2.user.access",
            r#"{"ok": true, "access_token": "xoxp-EXAMPLE...", "token_type": "user", "id_token": "eyJhbGciOiJSUzI1Ni...", "authed_user": {"id": "U12345", "scope": "openid,email,profile"}, "team": {"id": "T012345"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn openid_connect_token() {
    let req = OpenidConnectTokenRequest::new()
        .client_id("2141029472.691202649728")
        .client_secret("e1b9e11dfcd19c1982d5de12921e17e8c")
        .code("4724469134.4644010092847.232b4e6d82c333b475fc30f5f5a341d294feb1a94392c2fd791f7ab7731a443d1a")
        .redirect_uri("http://example.com")
        .grant_type("authorization_code")
        .refresh_token("xoxe-EXAMPLE")
        .code_verifier("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk");
    {
        let (_server, client) = setup("openid.connect.token", r#"{"ok": true, "access_token": "xoxp-EXAMPLE", "token_type": "Bearer", "id_token": "eyJhbGcMjY5OTA2MzcWNrLmNvbVwvdGVhbV9p..."}"#).await;
        let res = client
            .openid_connect_token(&req)
            .await
            .expect("openid.connect.token");
        assert_preserved(
            "openid.connect.token",
            r#"{"ok": true, "access_token": "xoxp-EXAMPLE", "token_type": "Bearer", "id_token": "eyJhbGcMjY5OTA2MzcWNrLmNvbVwvdGVhbV9p..."}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn openid_connect_user_info() {
    let req = OpenidConnectUserInfoRequest::new();
    {
        let (_server, client) = setup("openid.connect.userInfo", r#"{"ok": true, "sub": "U0R7JM", "https://slack.com/user_id": "U0R7JM", "https://slack.com/team_id": "T0R7GR", "email": "krane@slack-corp.com", "email_verified": true, "date_email_verified": 1622128723, "name": "krane", "picture": "https://secure.gravatar.com/....png", "given_name": "Bront", "family_name": "Labradoodle", "locale": "en-US", "https://slack.com/team_name": "kraneflannel", "https://slack.com/team_domain": "kraneflannel", "https://slack.com/user_image_24": "...", "https://slack.com/user_image_32": "...", "https://slack.com/user_image_48": "...", "https://slack.com/user_image_72": "...", "https://slack.com/user_image_192": "...", "https://slack.com/user_image_512": "...", "https://slack.com/team_image_34": "...", "https://slack.com/team_image_44": "...", "https://slack.com/team_image_68": "...", "https://slack.com/team_image_88": "...", "https://slack.com/team_image_102": "...", "https://slack.com/team_image_132": "...", "https://slack.com/team_image_230": "...", "https://slack.com/team_image_default": true}"#).await;
        let res = client
            .openid_connect_user_info(&req)
            .await
            .expect("openid.connect.userInfo");
        assert_preserved(
            "openid.connect.userInfo",
            r#"{"ok": true, "sub": "U0R7JM", "https://slack.com/user_id": "U0R7JM", "https://slack.com/team_id": "T0R7GR", "email": "krane@slack-corp.com", "email_verified": true, "date_email_verified": 1622128723, "name": "krane", "picture": "https://secure.gravatar.com/....png", "given_name": "Bront", "family_name": "Labradoodle", "locale": "en-US", "https://slack.com/team_name": "kraneflannel", "https://slack.com/team_domain": "kraneflannel", "https://slack.com/user_image_24": "...", "https://slack.com/user_image_32": "...", "https://slack.com/user_image_48": "...", "https://slack.com/user_image_72": "...", "https://slack.com/user_image_192": "...", "https://slack.com/user_image_512": "...", "https://slack.com/team_image_34": "...", "https://slack.com/team_image_44": "...", "https://slack.com/team_image_68": "...", "https://slack.com/team_image_88": "...", "https://slack.com/team_image_102": "...", "https://slack.com/team_image_132": "...", "https://slack.com/team_image_230": "...", "https://slack.com/team_image_default": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn pins_add() {
    let req = PinsAddRequest::new("x").timestamp("x");
    {
        let (_server, client) = setup("pins.add", r#"{"ok": true}"#).await;
        let res = client.pins_add(&req).await.expect("pins.add");
        assert_preserved("pins.add", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn pins_list() {
    let req = PinsListRequest::new("x");
    {
        let (_server, client) = setup("pins.list", r#"{"items": [{"channel": "C123ABC456", "created": 1508881078, "created_by": "U123ABC456", "message": {"permalink": "https://hitchhikers.slack.com/archives/C2U86NC6H/p1508197641000151", "pinned_to": ["C2U86NC6H"], "text": "What is the meaning of life?", "ts": "1508197641.000151", "type": "message", "user": "U123ABC456"}, "type": "message"}, {"channel": "C123ABC456", "created": 1508880991, "created_by": "U123ABC456", "message": {"permalink": "https://hitchhikers.slack.com/archives/C2U86NC6H/p1508284197000015", "pinned_to": ["C123ABC456"], "text": "The meaning of life, the universe, and everything is 42.", "ts": "1503289197.000015", "type": "message", "user": "U123ABC456"}, "type": "message"}], "ok": true}"#).await;
        let res = client.pins_list(&req).await.expect("pins.list");
        assert_preserved(
            "pins.list",
            r#"{"items": [{"channel": "C123ABC456", "created": 1508881078, "created_by": "U123ABC456", "message": {"permalink": "https://hitchhikers.slack.com/archives/C2U86NC6H/p1508197641000151", "pinned_to": ["C2U86NC6H"], "text": "What is the meaning of life?", "ts": "1508197641.000151", "type": "message", "user": "U123ABC456"}, "type": "message"}, {"channel": "C123ABC456", "created": 1508880991, "created_by": "U123ABC456", "message": {"permalink": "https://hitchhikers.slack.com/archives/C2U86NC6H/p1508284197000015", "pinned_to": ["C123ABC456"], "text": "The meaning of life, the universe, and everything is 42.", "ts": "1503289197.000015", "type": "message", "user": "U123ABC456"}, "type": "message"}], "ok": true}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn pins_remove() {
    let req = PinsRemoveRequest::new("x").timestamp("x");
    {
        let (_server, client) = setup("pins.remove", r#"{"ok": true}"#).await;
        let res = client.pins_remove(&req).await.expect("pins.remove");
        assert_preserved("pins.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reactions_add() {
    let req = ReactionsAddRequest::new("x", "thumbsup", "x");
    {
        let (_server, client) = setup("reactions.add", r#"{"ok": true}"#).await;
        let res = client.reactions_add(&req).await.expect("reactions.add");
        assert_preserved("reactions.add", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reactions_get() {
    let req = ReactionsGetRequest::new()
        .channel("C0NF841BK")
        .file("F1234567890")
        .file_comment("x")
        .full(true)
        .timestamp("1524523204.000192");
    {
        let (_server, client) = setup("reactions.get", r#"{"ok": true, "type": "message", "message": {"type": "message", "text": "Hi there!", "user": "W123456", "ts": "1648602352.215969", "team": "T123456", "reactions": [{"name": "grinning", "users": ["W222222"], "count": 1}, {"name": "question", "users": ["W333333"], "count": 1}], "permalink": "https://xxx.slack.com/archives/C123456/p1648602352215969"}, "channel": "C123ABC456"}"#).await;
        let res = client.reactions_get(&req).await.expect("reactions.get");
        assert_preserved(
            "reactions.get",
            r#"{"ok": true, "type": "message", "message": {"type": "message", "text": "Hi there!", "user": "W123456", "ts": "1648602352.215969", "team": "T123456", "reactions": [{"name": "grinning", "users": ["W222222"], "count": 1}, {"name": "question", "users": ["W333333"], "count": 1}], "permalink": "https://xxx.slack.com/archives/C123456/p1648602352215969"}, "channel": "C123ABC456"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reactions_list() {
    let req = ReactionsListRequest::new()
        .user("x")
        .full(true)
        .count(1)
        .page(1)
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1)
        .team_id("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(ReactionsListResponse::default().next_cursor(), None);
    let page = ReactionsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("reactions.list", r#"{"items": [{"type": "message", "channel": "C123ABC456", "message": {"bot_id": "B123ABC456", "reactions": [{"count": 1, "name": "robot_face", "users": ["U123ABC456"]}], "subtype": "bot_message", "text": "Hello from Python! :tada:", "ts": "1507849573.000090", "username": "Shipit Notifications"}}, {"comment": {"type": "file_comment", "comment": "This is a file comment", "created": 1508286096, "id": "Fc123ABC456", "reactions": [{"count": 1, "name": "white_check_mark", "users": ["U123ABC456"]}], "timestamp": 1508286096, "user": "U123ABC456"}, "file": {"channels": ["C123ABC456"], "comments_count": 1, "created": 1507850315, "reactions": [{"count": 1, "name": "stuck_out_tongue_winking_eye", "users": ["U123ABC456"]}], "title": "computer.gif", "user": "U123ABC456", "username": ""}}, {"file": {"channels": ["C123ABC456"], "comments_count": 1, "created": 1507850315, "id": "F123ABC456", "name": "computer.gif", "reactions": [{"count": 1, "name": "stuck_out_tongue_winking_eye", "users": ["U123ABC456"]}], "size": 1639034, "title": "computer.gif", "user": "U123ABC456", "username": ""}, "type": "file"}], "ok": true, "response_metadata": {"next_cursor": "dGVhbTpDMUg5UkVTR0w="}}"#).await;
        let res = client.reactions_list(&req).await.expect("reactions.list");
        assert_preserved(
            "reactions.list",
            r#"{"items": [{"type": "message", "channel": "C123ABC456", "message": {"bot_id": "B123ABC456", "reactions": [{"count": 1, "name": "robot_face", "users": ["U123ABC456"]}], "subtype": "bot_message", "text": "Hello from Python! :tada:", "ts": "1507849573.000090", "username": "Shipit Notifications"}}, {"comment": {"type": "file_comment", "comment": "This is a file comment", "created": 1508286096, "id": "Fc123ABC456", "reactions": [{"count": 1, "name": "white_check_mark", "users": ["U123ABC456"]}], "timestamp": 1508286096, "user": "U123ABC456"}, "file": {"channels": ["C123ABC456"], "comments_count": 1, "created": 1507850315, "reactions": [{"count": 1, "name": "stuck_out_tongue_winking_eye", "users": ["U123ABC456"]}], "title": "computer.gif", "user": "U123ABC456", "username": ""}}, {"file": {"channels": ["C123ABC456"], "comments_count": 1, "created": 1507850315, "id": "F123ABC456", "name": "computer.gif", "reactions": [{"count": 1, "name": "stuck_out_tongue_winking_eye", "users": ["U123ABC456"]}], "size": 1639034, "title": "computer.gif", "user": "U123ABC456", "username": ""}, "type": "file"}], "ok": true, "response_metadata": {"next_cursor": "dGVhbTpDMUg5UkVTR0w="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reactions_remove() {
    let req = ReactionsRemoveRequest::new("thumbsup")
        .file("x")
        .file_comment("x")
        .channel("x")
        .timestamp("x");
    {
        let (_server, client) = setup("reactions.remove", r#"{"ok": true}"#).await;
        let res = client
            .reactions_remove(&req)
            .await
            .expect("reactions.remove");
        assert_preserved("reactions.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reminders_add() {
    let req = RemindersAddRequest::new("eat a banana", "1602288000")
        .user("U18888888")
        .team_id("x")
        .recurrence(serde_json::json!({"k": "v"}));
    {
        let (_server, client) = setup("reminders.add", r#"{"ok": true, "reminder": {"id": "Rm12345678", "creator": "U123ABC456", "user": "U123ABC456", "text": "eat a banana", "recurring": false, "time": 1602288000, "complete_ts": 0}}"#).await;
        let res = client.reminders_add(&req).await.expect("reminders.add");
        assert_preserved(
            "reminders.add",
            r#"{"ok": true, "reminder": {"id": "Rm12345678", "creator": "U123ABC456", "user": "U123ABC456", "text": "eat a banana", "recurring": false, "time": 1602288000, "complete_ts": 0}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reminders_complete() {
    let req = RemindersCompleteRequest::new("Rm12345678").team_id("x");
    {
        let (_server, client) = setup("reminders.complete", r#"{"ok": true}"#).await;
        let res = client
            .reminders_complete(&req)
            .await
            .expect("reminders.complete");
        assert_preserved("reminders.complete", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("reminders.complete", r#"{"ok": true}"#).await;
        let res = client
            .reminders_complete(&req)
            .await
            .expect("reminders.complete");
        assert_preserved("reminders.complete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reminders_delete() {
    let req = RemindersDeleteRequest::new("Rm12345678").team_id("x");
    {
        let (_server, client) = setup("reminders.delete", r#"{"ok": true}"#).await;
        let res = client
            .reminders_delete(&req)
            .await
            .expect("reminders.delete");
        assert_preserved("reminders.delete", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("reminders.delete", r#"{"ok": true}"#).await;
        let res = client
            .reminders_delete(&req)
            .await
            .expect("reminders.delete");
        assert_preserved("reminders.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reminders_info() {
    let req = RemindersInfoRequest::new("Rm23456789").team_id("x");
    {
        let (_server, client) = setup("reminders.info", r#"{"ok": true}"#).await;
        let res = client.reminders_info(&req).await.expect("reminders.info");
        assert_preserved("reminders.info", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("reminders.info", r#"{"ok": true, "reminder": {"id": "Rm12345678", "creator": "U18888888", "user": "U18888888", "text": "eat a banana", "recurring": false, "time": 1458678068, "complete_ts": 1458678200}}"#).await;
        let res = client.reminders_info(&req).await.expect("reminders.info");
        assert_preserved(
            "reminders.info",
            r#"{"ok": true, "reminder": {"id": "Rm12345678", "creator": "U18888888", "user": "U18888888", "text": "eat a banana", "recurring": false, "time": 1458678068, "complete_ts": 1458678200}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn reminders_list() {
    let req = RemindersListRequest::new().team_id("x");
    {
        let (_server, client) = setup("reminders.list", r#"{"ok": true}"#).await;
        let res = client.reminders_list(&req).await.expect("reminders.list");
        assert_preserved("reminders.list", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("reminders.list", r#"{"ok": true, "reminders": [{"id": "Rm12345678", "creator": "U18888888", "user": "U18888888", "text": "eat a banana", "recurring": false, "time": 1458678068, "complete_ts": 0}, {"id": "Rm23456789", "creator": "U18888888", "user": "U18888888", "text": "drink water", "recurring": true}]}"#).await;
        let res = client.reminders_list(&req).await.expect("reminders.list");
        assert_preserved(
            "reminders.list",
            r#"{"ok": true, "reminders": [{"id": "Rm12345678", "creator": "U18888888", "user": "U18888888", "text": "eat a banana", "recurring": false, "time": 1458678068, "complete_ts": 0}, {"id": "Rm23456789", "creator": "U18888888", "user": "U18888888", "text": "drink water", "recurring": true}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn rtm_connect() {
    let req = RtmConnectRequest::new()
        .batch_presence_aware(true)
        .presence_sub(true);
    {
        let (_server, client) = setup("rtm.connect", r#"{"ok": true, "self": {"id": "U4X318ZMZ", "name": "robotoverlord"}, "team": {"domain": "slackdemo", "id": "T2U81E2FP", "name": "SlackDemo"}, "url": "wss://..."}"#).await;
        let res = client.rtm_connect(&req).await.expect("rtm.connect");
        assert_preserved(
            "rtm.connect",
            r#"{"ok": true, "self": {"id": "U4X318ZMZ", "name": "robotoverlord"}, "team": {"domain": "slackdemo", "id": "T2U81E2FP", "name": "SlackDemo"}, "url": "wss://..."}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn rtm_start() {
    let req = RtmStartRequest::new()
        .simple_latest(true)
        .no_unreads(true)
        .mpim_aware(true)
        .presence_sub(true)
        .batch_presence_aware(true)
        .no_latest(true)
        .include_locale(true);
    {
        let (_server, client) = setup("rtm.start", r#"{"ok": true}"#).await;
        let res = client.rtm_start(&req).await.expect("rtm.start");
        assert_preserved("rtm.start", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn search_all() {
    let req = SearchAllRequest::new("pickleface")
        .count(1)
        .highlight(true)
        .page(1)
        .sort("timestamp")
        .sort_dir("asc")
        .team_id("x");
    {
        let (_server, client) = setup("search.all", r#"{"files": {"matches": [{"channels": [], "comments_count": 1, "created": 1508804330, "display_as_bot": false, "editable": false, "external_type": "", "filetype": "png", "groups": [], "id": "F7PKF1NR7", "image_exif_rotation": 1, "ims": [], "initial_comment": {"comment": "Sure! Here's the workflow diagram!", "created": 1508804330, "id": "Fc7NLL52E7", "is_intro": true, "timestamp": 1508804330, "user": "U2U85N1RZ"}, "is_external": false, "is_public": true, "mimetype": "image/png", "mode": "hosted", "name": "slack workflow diagram.png", "original_h": 117, "original_w": 128, "permalink": "https://example.slack.com/files/U2U85N1RZ/F7PKF1NR7/slack_workflow_diagram.png", "permalink_public": "https://slack-files.com/T2U81E2FZ-F7PKF1NR7-bea9143f18", "pretty_type": "PNG", "preview": null, "public_url_shared": false, "score": "0.99982661240974", "size": 35705, "thumb_160": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_160.png", "thumb_360": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_360.png", "thumb_360_h": 117, "thumb_360_w": 128, "thumb_64": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_64.png", "thumb_80": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_80.png", "timestamp": 1508804330, "title": "slack workflow diagram", "top_file": false, "url_private": "https://files.slack.com/files-pri/T2U81E2FZ-F7PKF1NR7/slack_workflow_diagram.png", "url_private_download": "https://files.slack.com/files-pri/T2U81E2FZ-F7PKF1NR7/download/slack_workflow_diagram.png", "user": "U2U85N1RZ", "username": "amy"}], "pagination": {"first": 1, "last": 1, "page": 1, "page_count": 1, "per_page": 20, "total_count": 1}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 1}, "total": 1}, "messages": {"matches": [{"channel": {"id": "C2U86NC6M", "is_ext_shared": false, "is_mpim": false, "is_org_shared": false, "is_pending_ext_shared": false, "is_private": false, "is_shared": false, "name": "general", "pending_shared": []}, "iid": "35692677-e60e-43d9-ac45-1987cea88975", "next": {"iid": "6f510ea1-e1d3-4f3f-bdb9-f9c6f6e9d609", "text": "Thanks!", "ts": "1508804378.000219", "type": "message", "user": "U2U85HJ7R", "username": "john"}, "permalink": "https://example.slack.com/archives/C2U86NC6M/p1508804330000296", "previous": {"iid": "aba8603c-0543-4fb2-9118-a5ac85f3d138", "text": "Can you send me the Slack workflow diagram?", "ts": "1508804301.000026", "type": "message", "user": "U2U85HJ7R", "username": "john"}, "team": "T2U81E2FZ", "text": "uploaded a file: <https://example.slack.com/files/U2U85N1RZ/F7PKF1NR7/slack_workflow_diagram.png|slack workflow diagram> and commented: Sure! Here's the workflow diagram!", "ts": "1508804330.000296", "type": "message", "user": "U2U85N1RZ", "username": "amy"}], "pagination": {"first": 1, "last": 1, "page": 1, "page_count": 1, "per_page": 20, "total_count": 1}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 1}, "total": 1}, "ok": true, "posts": {"matches": [], "total": 0}, "query": "diagram"}"#).await;
        let res = client.search_all(&req).await.expect("search.all");
        assert_preserved(
            "search.all",
            r#"{"files": {"matches": [{"channels": [], "comments_count": 1, "created": 1508804330, "display_as_bot": false, "editable": false, "external_type": "", "filetype": "png", "groups": [], "id": "F7PKF1NR7", "image_exif_rotation": 1, "ims": [], "initial_comment": {"comment": "Sure! Here's the workflow diagram!", "created": 1508804330, "id": "Fc7NLL52E7", "is_intro": true, "timestamp": 1508804330, "user": "U2U85N1RZ"}, "is_external": false, "is_public": true, "mimetype": "image/png", "mode": "hosted", "name": "slack workflow diagram.png", "original_h": 117, "original_w": 128, "permalink": "https://example.slack.com/files/U2U85N1RZ/F7PKF1NR7/slack_workflow_diagram.png", "permalink_public": "https://slack-files.com/T2U81E2FZ-F7PKF1NR7-bea9143f18", "pretty_type": "PNG", "preview": null, "public_url_shared": false, "score": "0.99982661240974", "size": 35705, "thumb_160": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_160.png", "thumb_360": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_360.png", "thumb_360_h": 117, "thumb_360_w": 128, "thumb_64": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_64.png", "thumb_80": "https://files.slack.com/files-tmb/T2U81E2FZ-F7PKF1NR7-19f33fc256/slack_workflow_diagram_80.png", "timestamp": 1508804330, "title": "slack workflow diagram", "top_file": false, "url_private": "https://files.slack.com/files-pri/T2U81E2FZ-F7PKF1NR7/slack_workflow_diagram.png", "url_private_download": "https://files.slack.com/files-pri/T2U81E2FZ-F7PKF1NR7/download/slack_workflow_diagram.png", "user": "U2U85N1RZ", "username": "amy"}], "pagination": {"first": 1, "last": 1, "page": 1, "page_count": 1, "per_page": 20, "total_count": 1}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 1}, "total": 1}, "messages": {"matches": [{"channel": {"id": "C2U86NC6M", "is_ext_shared": false, "is_mpim": false, "is_org_shared": false, "is_pending_ext_shared": false, "is_private": false, "is_shared": false, "name": "general", "pending_shared": []}, "iid": "35692677-e60e-43d9-ac45-1987cea88975", "next": {"iid": "6f510ea1-e1d3-4f3f-bdb9-f9c6f6e9d609", "text": "Thanks!", "ts": "1508804378.000219", "type": "message", "user": "U2U85HJ7R", "username": "john"}, "permalink": "https://example.slack.com/archives/C2U86NC6M/p1508804330000296", "previous": {"iid": "aba8603c-0543-4fb2-9118-a5ac85f3d138", "text": "Can you send me the Slack workflow diagram?", "ts": "1508804301.000026", "type": "message", "user": "U2U85HJ7R", "username": "john"}, "team": "T2U81E2FZ", "text": "uploaded a file: <https://example.slack.com/files/U2U85N1RZ/F7PKF1NR7/slack_workflow_diagram.png|slack workflow diagram> and commented: Sure! Here's the workflow diagram!", "ts": "1508804330.000296", "type": "message", "user": "U2U85N1RZ", "username": "amy"}], "pagination": {"first": 1, "last": 1, "page": 1, "page_count": 1, "per_page": 20, "total_count": 1}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 1}, "total": 1}, "ok": true, "posts": {"matches": [], "total": 0}, "query": "diagram"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn search_files() {
    let req = SearchFilesRequest::new("pickleface")
        .count(1)
        .highlight(true)
        .page(1)
        .sort("timestamp")
        .sort_dir("asc")
        .team_id("x");
    {
        let (_server, client) = setup("search.files", r#"{"files": {"matches": [{"channels": [], "comments_count": 1, "created": 1507850315, "deanimate_gif": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_deanimate_gif.png", "display_as_bot": false, "editable": false, "external_type": "", "filetype": "gif", "groups": [], "id": "F7H0D7ZBB", "image_exif_rotation": 1, "ims": [], "is_external": false, "is_public": true, "mimetype": "image/gif", "mode": "hosted", "name": "computer.gif", "original_h": 313, "original_w": 500, "permalink": "https://eventsdemo.slack.com/files/U2U85N1RZ/F7H0D7ZBB/computer.gif", "permalink_public": "https://slack-files.com/T2U81E2BB-F7H0D7ZBB-85b7f5557e", "pretty_type": "GIF", "preview": null, "public_url_shared": false, "reactions": [{"count": 1, "name": "stuck_out_tongue_winking_eye", "users": ["U2U85N1RZ"]}], "score": "0.38899223746309", "size": 1639034, "thumb_160": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_160.png", "thumb_360": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_360.png", "thumb_360_gif": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_360.gif", "thumb_360_h": 225, "thumb_360_w": 360, "thumb_480": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_480.png", "thumb_480_gif": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_480.gif", "thumb_480_h": 300, "thumb_480_w": 480, "thumb_64": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_64.png", "thumb_80": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_80.png", "timestamp": 1507850315, "title": "computer.gif", "top_file": false, "url_private": "https://files.slack.com/files-pri/T2U81E2BB-F7H0D7ZBB/computer.gif", "url_private_download": "https://files.slack.com/files-pri/T2U81E2BB-F7H0D7ZBB/download/computer.gif", "user": "U2U85N1RZ", "username": ""}], "pagination": {"first": 1, "last": 3, "page": 1, "page_count": 1, "per_page": 20, "total_count": 3}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 3}, "total": 3}, "ok": true, "query": "computer.gif"}"#).await;
        let res = client.search_files(&req).await.expect("search.files");
        assert_preserved(
            "search.files",
            r#"{"files": {"matches": [{"channels": [], "comments_count": 1, "created": 1507850315, "deanimate_gif": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_deanimate_gif.png", "display_as_bot": false, "editable": false, "external_type": "", "filetype": "gif", "groups": [], "id": "F7H0D7ZBB", "image_exif_rotation": 1, "ims": [], "is_external": false, "is_public": true, "mimetype": "image/gif", "mode": "hosted", "name": "computer.gif", "original_h": 313, "original_w": 500, "permalink": "https://eventsdemo.slack.com/files/U2U85N1RZ/F7H0D7ZBB/computer.gif", "permalink_public": "https://slack-files.com/T2U81E2BB-F7H0D7ZBB-85b7f5557e", "pretty_type": "GIF", "preview": null, "public_url_shared": false, "reactions": [{"count": 1, "name": "stuck_out_tongue_winking_eye", "users": ["U2U85N1RZ"]}], "score": "0.38899223746309", "size": 1639034, "thumb_160": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_160.png", "thumb_360": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_360.png", "thumb_360_gif": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_360.gif", "thumb_360_h": 225, "thumb_360_w": 360, "thumb_480": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_480.png", "thumb_480_gif": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_480.gif", "thumb_480_h": 300, "thumb_480_w": 480, "thumb_64": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_64.png", "thumb_80": "https://files.slack.com/files-tmb/T2U81E2BB-F7H0D7ZBB-21624821e6/computer_80.png", "timestamp": 1507850315, "title": "computer.gif", "top_file": false, "url_private": "https://files.slack.com/files-pri/T2U81E2BB-F7H0D7ZBB/computer.gif", "url_private_download": "https://files.slack.com/files-pri/T2U81E2BB-F7H0D7ZBB/download/computer.gif", "user": "U2U85N1RZ", "username": ""}], "pagination": {"first": 1, "last": 3, "page": 1, "page_count": 1, "per_page": 20, "total_count": 3}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 3}, "total": 3}, "ok": true, "query": "computer.gif"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn search_messages() {
    let req = SearchMessagesRequest::new("pickleface")
        .count(1)
        .highlight(true)
        .page(1)
        .cursor("x")
        .sort("timestamp")
        .sort_dir("asc")
        .team_id("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(SearchMessagesResponse::default().next_cursor(), None);
    let page = SearchMessagesResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("search.messages", r#"{"messages": {"matches": [{"channel": {"id": "C12345678", "is_ext_shared": false, "is_mpim": false, "is_org_shared": false, "is_pending_ext_shared": false, "is_private": false, "is_shared": false, "name": "general", "pending_shared": []}, "iid": "cb64bdaa-c1e8-4631-8a91-0f78080113e9", "permalink": "https://hitchhikers.slack.com/archives/C12345678/p1508284197000015", "team": "T12345678", "text": "The meaning of life the universe and everything is 42.", "ts": "1508284197.000015", "type": "message", "user": "U2U85N1RV", "username": "roach"}, {"channel": {"id": "C12345678", "is_ext_shared": false, "is_mpim": false, "is_org_shared": false, "is_pending_ext_shared": false, "is_private": false, "is_shared": false, "name": "random", "pending_shared": []}, "iid": "9a00d3c9-bd2d-45b0-988b-6cff99ae2a90", "permalink": "https://hitchhikers.slack.com/archives/C12345678/p1508795665000236", "team": "T12345678", "text": "The meaning of life the universe and everything is 101010", "ts": "1508795665.000236", "type": "message", "user": "", "username": "robot overlord"}], "pagination": {"first": 1, "last": 2, "page": 1, "page_count": 1, "per_page": 20, "total_count": 2}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 2}, "total": 2}, "ok": true, "query": "The meaning of life the universe and everything"}"#).await;
        let res = client.search_messages(&req).await.expect("search.messages");
        assert_preserved(
            "search.messages",
            r#"{"messages": {"matches": [{"channel": {"id": "C12345678", "is_ext_shared": false, "is_mpim": false, "is_org_shared": false, "is_pending_ext_shared": false, "is_private": false, "is_shared": false, "name": "general", "pending_shared": []}, "iid": "cb64bdaa-c1e8-4631-8a91-0f78080113e9", "permalink": "https://hitchhikers.slack.com/archives/C12345678/p1508284197000015", "team": "T12345678", "text": "The meaning of life the universe and everything is 42.", "ts": "1508284197.000015", "type": "message", "user": "U2U85N1RV", "username": "roach"}, {"channel": {"id": "C12345678", "is_ext_shared": false, "is_mpim": false, "is_org_shared": false, "is_pending_ext_shared": false, "is_private": false, "is_shared": false, "name": "random", "pending_shared": []}, "iid": "9a00d3c9-bd2d-45b0-988b-6cff99ae2a90", "permalink": "https://hitchhikers.slack.com/archives/C12345678/p1508795665000236", "team": "T12345678", "text": "The meaning of life the universe and everything is 101010", "ts": "1508795665.000236", "type": "message", "user": "", "username": "robot overlord"}], "pagination": {"first": 1, "last": 2, "page": 1, "page_count": 1, "per_page": 20, "total_count": 2}, "paging": {"count": 20, "page": 1, "pages": 1, "total": 2}, "total": 2}, "ok": true, "query": "The meaning of life the universe and everything"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_access_delete() {
    let req = SlackListsAccessDeleteRequest::new("F1234ABCD")
        .channel_ids(vec!["A1".to_string(), "A2".to_string()])
        .user_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("slackLists.access.delete", r#"{"ok": true}"#).await;
        let res = client
            .slack_lists_access_delete(&req)
            .await
            .expect("slackLists.access.delete");
        assert_preserved("slackLists.access.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_access_set() {
    let req = SlackListsAccessSetRequest::new("F1234ABCD", "read")
        .channel_ids(vec!["A1".to_string(), "A2".to_string()])
        .user_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("slackLists.access.set", r#"{"ok": true}"#).await;
        let res = client
            .slack_lists_access_set(&req)
            .await
            .expect("slackLists.access.set");
        assert_preserved("slackLists.access.set", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_create() {
    let req = SlackListsCreateRequest::new("My List")
        .description_blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .schema(vec![serde_json::json!({"k": "v"})])
        .copy_from_list_id("F1234567")
        .include_copied_list_records(true)
        .todo_mode(true);
    {
        let (_server, client) = setup(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD"}"#,
        )
        .await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD"}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.create", r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}], "subtask_schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}]}}"#).await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}], "subtask_schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}]}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.create", r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "rich_text_notes", "name": "Notes", "is_primary_column": false, "type": "rich_text", "id": "Col056A7BCDE8"}, {"key": "message_link", "name": "Message", "is_primary_column": false, "type": "message", "id": "Col090A1BCDE2"}, {"key": "ranking", "name": "Ranking", "is_primary_column": false, "type": "number", "options": {"precision": 2, "show_member_name": true}, "id": "Col034A5BCDE6"}, {"key": "status", "name": "Status", "is_primary_column": false, "type": "select", "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}, "id": "Col078A9BCDE0"}, {"key": "labels", "name": "Labels", "is_primary_column": false, "type": "multi_select", "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}, "id": "Col112A2BCDE3"}, {"key": "date", "name": "Date", "is_primary_column": false, "type": "date", "id": "Col223A3BCDE4"}, {"key": "owner", "name": "Owner", "is_primary_column": false, "type": "user", "options": {"format": "single_entity", "show_member_name": true}, "id": "Col334A4BCDE5"}, {"key": "attachments", "name": "Attachments", "is_primary_column": false, "type": "attachment", "id": "Col556A6BCDE7"}, {"key": "ready", "name": "Ready?", "is_primary_column": false, "type": "checkbox", "id": "Col778A8BCDE9"}, {"key": "email", "name": "Email", "is_primary_column": false, "type": "email", "id": "Col990A0BCDE1"}, {"key": "phone", "name": "Phone", "is_primary_column": false, "type": "phone", "id": "Col111A1BCDE1"}, {"key": "channel", "name": "Channel", "is_primary_column": false, "type": "channel", "id": "Col222A2BCDE2"}, {"key": "rating", "name": "Rating", "is_primary_column": false, "type": "rating", "options": {"emoji": ":star:", "max": 5, "show_member_name": true}, "id": "Col333A3BCDE3"}, {"key": "vote", "name": "Vote", "is_primary_column": false, "type": "vote", "id": "Co444A4BCDE4"}, {"key": "assignee", "name": "Assignee", "is_primary_column": false, "type": "assignee", "id": "Col555A5BCDE5"}, {"key": "due_date", "name": "Due Date", "is_primary_column": false, "type": "due_date", "id": "Col666A6BCDE6"}, {"key": "completed", "name": "Completed", "is_primary_column": false, "type": "completed", "id": "Col777A7BCDE7"}, {"key": "canvas", "name": "Canvas", "is_primary_column": false, "type": "canvas", "id": "Col888A8BCDE8"}, {"key": "link", "name": "Link", "is_primary_column": false, "type": "link", "id": "Col999A9BCDE9"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col000A0BCDE0"}]}}"#).await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "rich_text_notes", "name": "Notes", "is_primary_column": false, "type": "rich_text", "id": "Col056A7BCDE8"}, {"key": "message_link", "name": "Message", "is_primary_column": false, "type": "message", "id": "Col090A1BCDE2"}, {"key": "ranking", "name": "Ranking", "is_primary_column": false, "type": "number", "options": {"precision": 2, "show_member_name": true}, "id": "Col034A5BCDE6"}, {"key": "status", "name": "Status", "is_primary_column": false, "type": "select", "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}, "id": "Col078A9BCDE0"}, {"key": "labels", "name": "Labels", "is_primary_column": false, "type": "multi_select", "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}, "id": "Col112A2BCDE3"}, {"key": "date", "name": "Date", "is_primary_column": false, "type": "date", "id": "Col223A3BCDE4"}, {"key": "owner", "name": "Owner", "is_primary_column": false, "type": "user", "options": {"format": "single_entity", "show_member_name": true}, "id": "Col334A4BCDE5"}, {"key": "attachments", "name": "Attachments", "is_primary_column": false, "type": "attachment", "id": "Col556A6BCDE7"}, {"key": "ready", "name": "Ready?", "is_primary_column": false, "type": "checkbox", "id": "Col778A8BCDE9"}, {"key": "email", "name": "Email", "is_primary_column": false, "type": "email", "id": "Col990A0BCDE1"}, {"key": "phone", "name": "Phone", "is_primary_column": false, "type": "phone", "id": "Col111A1BCDE1"}, {"key": "channel", "name": "Channel", "is_primary_column": false, "type": "channel", "id": "Col222A2BCDE2"}, {"key": "rating", "name": "Rating", "is_primary_column": false, "type": "rating", "options": {"emoji": ":star:", "max": 5, "show_member_name": true}, "id": "Col333A3BCDE3"}, {"key": "vote", "name": "Vote", "is_primary_column": false, "type": "vote", "id": "Co444A4BCDE4"}, {"key": "assignee", "name": "Assignee", "is_primary_column": false, "type": "assignee", "id": "Col555A5BCDE5"}, {"key": "due_date", "name": "Due Date", "is_primary_column": false, "type": "due_date", "id": "Col666A6BCDE6"}, {"key": "completed", "name": "Completed", "is_primary_column": false, "type": "completed", "id": "Col777A7BCDE7"}, {"key": "canvas", "name": "Canvas", "is_primary_column": false, "type": "canvas", "id": "Col888A8BCDE8"}, {"key": "link", "name": "Link", "is_primary_column": false, "type": "link", "id": "Col999A9BCDE9"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col000A0BCDE0"}]}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.create", r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "rich_text_notes", "name": "Notes", "is_primary_column": false, "type": "rich_text", "id": "Col056A7BCDE8"}, {"key": "message_link", "name": "Message", "is_primary_column": false, "type": "message", "id": "Col090A1BCDE2"}, {"key": "estimate", "name": "Estimate", "is_primary_column": false, "type": "number", "options": {"precision": 2, "show_member_name": true}, "id": "Col034A5BCDE6"}, {"key": "status", "name": "Status", "is_primary_column": false, "type": "select", "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}, "id": "Col078A9BCDE0"}, {"key": "labels", "name": "Labels", "is_primary_column": false, "type": "multi_select", "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}, "id": "Col112A2BCDE3"}, {"key": "date", "name": "Date", "is_primary_column": false, "type": "date", "id": "Col223A3BCDE4"}, {"key": "owner", "name": "Owner", "is_primary_column": false, "type": "user", "options": {"format": "single_entity", "show_member_name": true}, "id": "Col334A4BCDE5"}, {"key": "attachments", "name": "Attachments", "is_primary_column": false, "type": "attachment", "id": "Col556A6BCDE7"}, {"key": "ready", "name": "Ready?", "is_primary_column": false, "type": "checkbox", "id": "Col778A8BCDE9"}, {"key": "email", "name": "Email", "is_primary_column": false, "type": "email", "id": "Col990A0BCDE1"}, {"key": "phone", "name": "Phone", "is_primary_column": false, "type": "phone", "id": "Col987A6BCDE5"}, {"key": "channel", "name": "Channel", "is_primary_column": false, "type": "channel", "id": "Col111A1BCDE1"}, {"key": "rating", "name": "Rating", "is_primary_column": false, "type": "rating", "options": {"emoji": ":star:", "max": 5, "show_member_name": true}, "id": "Col333A3BCDE3"}, {"key": "vote", "name": "Vote", "is_primary_column": false, "type": "vote", "id": "Co444A4BCDE4"}, {"key": "assignee", "name": "Assignee", "is_primary_column": false, "type": "assignee", "id": "Col555A5BCDE5"}, {"key": "due_date", "name": "Due Date", "is_primary_column": false, "type": "due_date", "id": "Col777A7BCDE7"}, {"key": "completed", "name": "Completed", "is_primary_column": false, "type": "completed", "id": "Col888A8BCDE8"}, {"key": "canvas", "name": "Canvas", "is_primary_column": false, "type": "canvas", "id": "Col999A9BCDE9"}, {"key": "link", "name": "Link", "is_primary_column": false, "type": "link", "id": "Col000A0BCDE0"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col543A2BCDE1"}]}}"#).await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "rich_text_notes", "name": "Notes", "is_primary_column": false, "type": "rich_text", "id": "Col056A7BCDE8"}, {"key": "message_link", "name": "Message", "is_primary_column": false, "type": "message", "id": "Col090A1BCDE2"}, {"key": "estimate", "name": "Estimate", "is_primary_column": false, "type": "number", "options": {"precision": 2, "show_member_name": true}, "id": "Col034A5BCDE6"}, {"key": "status", "name": "Status", "is_primary_column": false, "type": "select", "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}, "id": "Col078A9BCDE0"}, {"key": "labels", "name": "Labels", "is_primary_column": false, "type": "multi_select", "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}, "id": "Col112A2BCDE3"}, {"key": "date", "name": "Date", "is_primary_column": false, "type": "date", "id": "Col223A3BCDE4"}, {"key": "owner", "name": "Owner", "is_primary_column": false, "type": "user", "options": {"format": "single_entity", "show_member_name": true}, "id": "Col334A4BCDE5"}, {"key": "attachments", "name": "Attachments", "is_primary_column": false, "type": "attachment", "id": "Col556A6BCDE7"}, {"key": "ready", "name": "Ready?", "is_primary_column": false, "type": "checkbox", "id": "Col778A8BCDE9"}, {"key": "email", "name": "Email", "is_primary_column": false, "type": "email", "id": "Col990A0BCDE1"}, {"key": "phone", "name": "Phone", "is_primary_column": false, "type": "phone", "id": "Col987A6BCDE5"}, {"key": "channel", "name": "Channel", "is_primary_column": false, "type": "channel", "id": "Col111A1BCDE1"}, {"key": "rating", "name": "Rating", "is_primary_column": false, "type": "rating", "options": {"emoji": ":star:", "max": 5, "show_member_name": true}, "id": "Col333A3BCDE3"}, {"key": "vote", "name": "Vote", "is_primary_column": false, "type": "vote", "id": "Co444A4BCDE4"}, {"key": "assignee", "name": "Assignee", "is_primary_column": false, "type": "assignee", "id": "Col555A5BCDE5"}, {"key": "due_date", "name": "Due Date", "is_primary_column": false, "type": "due_date", "id": "Col777A7BCDE7"}, {"key": "completed", "name": "Completed", "is_primary_column": false, "type": "completed", "id": "Col888A8BCDE8"}, {"key": "canvas", "name": "Canvas", "is_primary_column": false, "type": "canvas", "id": "Col999A9BCDE9"}, {"key": "link", "name": "Link", "is_primary_column": false, "type": "link", "id": "Col000A0BCDE0"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col543A2BCDE1"}]}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.create", r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "rich_text_notes", "name": "Notes", "is_primary_column": false, "type": "rich_text", "id": "Col056A7BCDE8"}, {"key": "message_link", "name": "Message", "is_primary_column": false, "type": "message", "id": "Col090A1BCDE2"}, {"key": "estimate", "name": "Estimate", "is_primary_column": false, "type": "number", "options": {"precision": 2, "show_member_name": true}, "id": "Col034A5BCDE6"}, {"key": "status", "name": "Status", "is_primary_column": false, "type": "select", "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}, "id": "Col078A9BCDE0"}, {"key": "labels", "name": "Labels", "is_primary_column": false, "type": "multi_select", "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}, "id": "Col112A2BCDE3"}, {"key": "date", "name": "Date", "is_primary_column": false, "type": "date", "id": "Col223A3BCDE4"}, {"key": "owner", "name": "Owner", "is_primary_column": false, "type": "user", "options": {"format": "single_entity", "show_member_name": true}, "id": "Col334A4BCDE5"}, {"key": "attachments", "name": "Attachments", "is_primary_column": false, "type": "attachment", "id": "Col556A6BCDE7"}, {"key": "ready", "name": "Ready?", "is_primary_column": false, "type": "checkbox", "id": "Col778A8BCDE9"}, {"key": "email", "name": "Email", "is_primary_column": false, "type": "email", "id": "Col990A0BCDE1"}, {"key": "phone", "name": "Phone", "is_primary_column": false, "type": "phone", "id": "Col111A1BCDE1"}, {"key": "channel", "name": "Channel", "is_primary_column": false, "type": "channel", "id": "Col333A3BCDE3"}, {"key": "rating", "name": "Rating", "is_primary_column": false, "type": "rating", "options": {"emoji": ":star:", "max": 5, "show_member_name": true}, "id": "Co444A4BCDE4"}, {"key": "vote", "name": "Vote", "is_primary_column": false, "type": "vote", "id": "Col555A5BCDE5"}, {"key": "assignee", "name": "Assignee", "is_primary_column": false, "type": "assignee", "id": "Col777A7BCDE7"}, {"key": "due_date", "name": "Due Date", "is_primary_column": false, "type": "due_date", "id": "Col888A8BCDE8"}, {"key": "completed", "name": "Completed", "is_primary_column": false, "type": "completed", "id": "Col999A9BCDE9"}, {"key": "canvas", "name": "Canvas", "is_primary_column": false, "type": "canvas", "id": "Col000A0BCDE0"}, {"key": "link", "name": "Link", "is_primary_column": false, "type": "link", "id": "Col543A2BCDE1"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col654A3BCDE2"}]}}"#).await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "rich_text_notes", "name": "Notes", "is_primary_column": false, "type": "rich_text", "id": "Col056A7BCDE8"}, {"key": "message_link", "name": "Message", "is_primary_column": false, "type": "message", "id": "Col090A1BCDE2"}, {"key": "estimate", "name": "Estimate", "is_primary_column": false, "type": "number", "options": {"precision": 2, "show_member_name": true}, "id": "Col034A5BCDE6"}, {"key": "status", "name": "Status", "is_primary_column": false, "type": "select", "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}, "id": "Col078A9BCDE0"}, {"key": "labels", "name": "Labels", "is_primary_column": false, "type": "multi_select", "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}, "id": "Col112A2BCDE3"}, {"key": "date", "name": "Date", "is_primary_column": false, "type": "date", "id": "Col223A3BCDE4"}, {"key": "owner", "name": "Owner", "is_primary_column": false, "type": "user", "options": {"format": "single_entity", "show_member_name": true}, "id": "Col334A4BCDE5"}, {"key": "attachments", "name": "Attachments", "is_primary_column": false, "type": "attachment", "id": "Col556A6BCDE7"}, {"key": "ready", "name": "Ready?", "is_primary_column": false, "type": "checkbox", "id": "Col778A8BCDE9"}, {"key": "email", "name": "Email", "is_primary_column": false, "type": "email", "id": "Col990A0BCDE1"}, {"key": "phone", "name": "Phone", "is_primary_column": false, "type": "phone", "id": "Col111A1BCDE1"}, {"key": "channel", "name": "Channel", "is_primary_column": false, "type": "channel", "id": "Col333A3BCDE3"}, {"key": "rating", "name": "Rating", "is_primary_column": false, "type": "rating", "options": {"emoji": ":star:", "max": 5, "show_member_name": true}, "id": "Co444A4BCDE4"}, {"key": "vote", "name": "Vote", "is_primary_column": false, "type": "vote", "id": "Col555A5BCDE5"}, {"key": "assignee", "name": "Assignee", "is_primary_column": false, "type": "assignee", "id": "Col777A7BCDE7"}, {"key": "due_date", "name": "Due Date", "is_primary_column": false, "type": "due_date", "id": "Col888A8BCDE8"}, {"key": "completed", "name": "Completed", "is_primary_column": false, "type": "completed", "id": "Col999A9BCDE9"}, {"key": "canvas", "name": "Canvas", "is_primary_column": false, "type": "canvas", "id": "Col000A0BCDE0"}, {"key": "link", "name": "Link", "is_primary_column": false, "type": "link", "id": "Col543A2BCDE1"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col654A3BCDE2"}]}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.create", r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}], "subtask_schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}]}}"#).await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}], "subtask_schema": [{"key": "name", "name": "Name", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}]}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.create", r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}]}}"#).await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}]}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.create", r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}]}}"#).await;
        let res = client
            .slack_lists_create(&req)
            .await
            .expect("slackLists.create");
        assert_preserved(
            "slackLists.create",
            r#"{"ok": true, "list_id": "F1234ABCD", "list_metadata": {"schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}], "subtask_schema": [{"key": "title", "name": "Title", "is_primary_column": true, "type": "text", "id": "Col012A3BCDE4"}, {"key": "todo_completed", "name": "Completed", "is_primary_column": false, "type": "todo_completed", "id": "Col00"}, {"key": "todo_assignee", "name": "Assignee", "is_primary_column": false, "type": "todo_assignee", "options": {"format": "multi_entity", "default_value": null, "show_member_name": true}, "id": "Col01"}, {"key": "todo_due_date", "name": "Due Date", "is_primary_column": false, "type": "todo_due_date", "id": "Col02"}]}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_download_get() {
    let req = SlackListsDownloadGetRequest::new("F1234567", "Le48SKT566")
        .format("x")
        .include_threads(true)
        .include_attachments(true);
    {
        let (_server, client) = setup("slackLists.download.get", r#"{"ok": true, "status": "COMPLETED", "download_url": "https://files.com/files-pri/1234567890-F12345678/csv/list?origin_team=T1234567890"}"#).await;
        let res = client
            .slack_lists_download_get(&req)
            .await
            .expect("slackLists.download.get");
        assert_preserved(
            "slackLists.download.get",
            r#"{"ok": true, "status": "COMPLETED", "download_url": "https://files.com/files-pri/1234567890-F12345678/csv/list?origin_team=T1234567890"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_download_start() {
    let req = SlackListsDownloadStartRequest::new("F1234567")
        .include_archived(true)
        .format("x")
        .include_threads(true)
        .include_attachments(true);
    {
        let (_server, client) = setup(
            "slackLists.download.start",
            r#"{"ok": true, "job_id": "LeF1234567"}"#,
        )
        .await;
        let res = client
            .slack_lists_download_start(&req)
            .await
            .expect("slackLists.download.start");
        assert_preserved(
            "slackLists.download.start",
            r#"{"ok": true, "job_id": "LeF1234567"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_items_create() {
    let req = SlackListsItemsCreateRequest::new("x")
        .duplicated_item_id("x")
        .parent_item_id("x")
        .initial_fields(vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) = setup("slackLists.items.create", r#"{"ok": true, "item": {"id": "Rec018ALE9718", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744345"}}"#).await;
        let res = client
            .slack_lists_items_create(&req)
            .await
            .expect("slackLists.items.create");
        assert_preserved(
            "slackLists.items.create",
            r#"{"ok": true, "item": {"id": "Rec018ALE9718", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744345"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.items.create", r#"{"ok": true, "item": {"id": "Rec018ALFLNTU", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"k0zIi\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Fix bug\"}]}]}]", "text": "Fix bug", "rich_text": [{"type": "rich_text", "block_id": "k0zIi", "elements": [{"type": "rich_text_section", "elements": [{"text": "Fix bug", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}}"#).await;
        let res = client
            .slack_lists_items_create(&req)
            .await
            .expect("slackLists.items.create");
        assert_preserved(
            "slackLists.items.create",
            r#"{"ok": true, "item": {"id": "Rec018ALFLNTU", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"k0zIi\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Fix bug\"}]}]}]", "text": "Fix bug", "rich_text": [{"type": "rich_text", "block_id": "k0zIi", "elements": [{"type": "rich_text_section", "elements": [{"text": "Fix bug", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.items.create", r#"{"ok": true, "item": {"id": "Rec018ALA7RPU", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "status", "value": "completed", "select": ["completed"], "column_id": "Col018AL7649G"}, {"key": "date", "value": "2025-09-19", "date": ["2025-09-19"], "timestamp": [-1], "column_id": "Col018AL764AE"}, {"key": "owner", "value": "U012A34BCDE", "user": ["U012A34BCDE"], "column_id": "Col018B8C91V1"}], "updated_timestamp": "1758744346"}}"#).await;
        let res = client
            .slack_lists_items_create(&req)
            .await
            .expect("slackLists.items.create");
        assert_preserved(
            "slackLists.items.create",
            r#"{"ok": true, "item": {"id": "Rec018ALA7RPU", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "status", "value": "completed", "select": ["completed"], "column_id": "Col018AL7649G"}, {"key": "date", "value": "2025-09-19", "date": ["2025-09-19"], "timestamp": [-1], "column_id": "Col018AL764AE"}, {"key": "owner", "value": "U012A34BCDE", "user": ["U012A34BCDE"], "column_id": "Col018B8C91V1"}], "updated_timestamp": "1758744346"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.items.create", r#"{"ok": true, "item": {"id": "Rec018ALFLP2N", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}}"#).await;
        let res = client
            .slack_lists_items_create(&req)
            .await
            .expect("slackLists.items.create");
        assert_preserved(
            "slackLists.items.create",
            r#"{"ok": true, "item": {"id": "Rec018ALFLP2N", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.items.create", r#"{"ok": true, "item": {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}}"#).await;
        let res = client
            .slack_lists_items_create(&req)
            .await
            .expect("slackLists.items.create");
        assert_preserved(
            "slackLists.items.create",
            r#"{"ok": true, "item": {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_items_delete() {
    let req = SlackListsItemsDeleteRequest::new("F12345678", "Rec1234567");
    {
        let (_server, client) = setup("slackLists.items.delete", r#"{"ok": true}"#).await;
        let res = client
            .slack_lists_items_delete(&req)
            .await
            .expect("slackLists.items.delete");
        assert_preserved("slackLists.items.delete", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_items_delete_multiple() {
    let req = SlackListsItemsDeleteMultipleRequest::new(
        "F12345678",
        vec!["A1".to_string(), "A2".to_string()],
    );
    {
        let (_server, client) = setup("slackLists.items.deleteMultiple", r#"{"ok": true}"#).await;
        let res = client
            .slack_lists_items_delete_multiple(&req)
            .await
            .expect("slackLists.items.deleteMultiple");
        assert_preserved("slackLists.items.deleteMultiple", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_items_info() {
    let req =
        SlackListsItemsInfoRequest::new("F1234567", "Rec014K005UQJ").include_is_subscribed(true);
    {
        let (_server, client) = setup("slackLists.items.info", r#"{"ok": true, "list": {"id": "F1234567", "created": 1758744341, "timestamp": 1758744341, "name": "list", "title": "Sprint Board", "mimetype": "application/vnd.slack-list", "filetype": "list", "pretty_type": "List", "user": "W0AB1CDE2", "user_team": "E0AB1CD2E", "editable": true, "size": 0, "mode": "list", "is_external": false, "external_type": "", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "list_metadata": {"schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}, {"id": "Col018B8C91TM", "name": "Notes", "key": "rich_text_notes", "type": "rich_text", "is_primary_column": false}, {"id": "Col018AL76490", "name": "Message", "key": "message_link", "type": "message", "is_primary_column": false}, {"id": "Col018B8C91U3", "name": "Estimate", "key": "estimate", "type": "number", "is_primary_column": false, "options": {"precision": 2, "show_member_name": true}}, {"id": "Col018AL7649G", "name": "Status", "key": "status", "type": "select", "is_primary_column": false, "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}}, {"id": "Col018B8C91UK", "name": "Labels", "key": "labels", "type": "multi_select", "is_primary_column": false, "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}}, {"id": "Col018AL764AE", "name": "Date", "key": "date", "type": "date", "is_primary_column": false}, {"id": "Col018B8C91V1", "name": "Owner", "key": "owner", "type": "user", "is_primary_column": false, "options": {"format": "single_entity", "show_member_name": true}}, {"id": "Col018AL764AW", "name": "Attachments", "key": "attachments", "type": "attachment", "is_primary_column": false}, {"id": "Col018B8C9203", "name": "Ready?", "key": "ready", "type": "checkbox", "is_primary_column": false}, {"id": "Col018AL764BC", "name": "Email", "key": "email", "type": "email", "is_primary_column": false}, {"id": "Col018B8C921H", "name": "Phone", "key": "phone", "type": "phone", "is_primary_column": false}, {"id": "Col018AL764G2", "name": "Channel", "key": "channel", "type": "channel", "is_primary_column": false}, {"id": "Col018B8C9259", "name": "Rating", "key": "rating", "type": "rating", "is_primary_column": false, "options": {"emoji": ":star:", "max": 5, "show_member_name": true}}, {"id": "Col018AL77TH8", "name": "Vote", "key": "vote", "type": "vote", "is_primary_column": false}, {"id": "Col018B8D6Q91", "name": "Assignee", "key": "assignee", "type": "assignee", "is_primary_column": false}, {"id": "Col018AL83THQ", "name": "Due Date", "key": "due_date", "type": "due_date", "is_primary_column": false}, {"id": "Col018AL857QS", "name": "Completed", "key": "completed", "type": "completed", "is_primary_column": false}, {"id": "Col018B8D84R1", "name": "Canvas", "key": "canvas", "type": "canvas", "is_primary_column": false}, {"id": "Col018AL9577U", "name": "Link", "key": "link", "type": "link", "is_primary_column": false}], "views": [{"id": "View018B8E85S7", "name": "Record", "type": "record", "is_locked": false, "position": "1758744341", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744341, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": false}, {"id": "View018AL9598S", "name": "All items", "type": "table", "is_locked": false, "position": "1758744342", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744342, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": true, "default_view_key": "all_items", "show_completed_items": true}], "integrations": [], "icon": "", "description": "", "description_blocks": [{"type": "rich_text", "block_id": "XYcTZ", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Sprint work"}]}]}], "is_trial": false, "subtask_schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}], "creation_source": {"type": "copy_from_list", "reference_id": "F018AL98L3U"}, "todo_mode": false, "default_view": ""}, "list_limits": {"over_row_maximum": false, "row_count_limit": 1000, "row_count": 7, "archived_row_count": 0, "over_column_maximum": false, "column_count": 20, "column_count_limit": 30, "over_view_maximum": false, "view_count": 2, "view_count_limit": 50, "max_attachments_per_cell": 10}, "url_private": "https://files.com/files-pri/T0AB1CD2E-F1234567/list", "url_private_download": "https://files.com/files-pri/T0AB1CD2E-F1234567/download/list", "permalink": "https://someenterpriseorg.slack.com/lists/T0ABCDEFG/F1234567", "permalink_public": "https://files.com/T0AB1CD2E-F1234567-123ab4cd56", "last_editor": "W0QR1STU2", "list_csv_download_url": "https://files.com/files-pri/T0AB1CD2E-F1234567/csv/list", "updated": 1758744347, "is_starred": false, "skipped_shares": true, "teams_shared_with": ["E0AB1CD2E"], "is_restricted_sharing_enabled": false, "has_rich_preview": false, "file_access": "visible", "access": "owner", "org_or_workspace_access": "none", "is_ai_suggested": false}, "record": {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0QR1STU2", "updated_by": "W0QR1STU2", "fields": [], "updated_timestamp": "1758744346"}, "subtasks": []}"#).await;
        let res = client
            .slack_lists_items_info(&req)
            .await
            .expect("slackLists.items.info");
        assert_preserved(
            "slackLists.items.info",
            r#"{"ok": true, "list": {"id": "F1234567", "created": 1758744341, "timestamp": 1758744341, "name": "list", "title": "Sprint Board", "mimetype": "application/vnd.slack-list", "filetype": "list", "pretty_type": "List", "user": "W0AB1CDE2", "user_team": "E0AB1CD2E", "editable": true, "size": 0, "mode": "list", "is_external": false, "external_type": "", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "list_metadata": {"schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}, {"id": "Col018B8C91TM", "name": "Notes", "key": "rich_text_notes", "type": "rich_text", "is_primary_column": false}, {"id": "Col018AL76490", "name": "Message", "key": "message_link", "type": "message", "is_primary_column": false}, {"id": "Col018B8C91U3", "name": "Estimate", "key": "estimate", "type": "number", "is_primary_column": false, "options": {"precision": 2, "show_member_name": true}}, {"id": "Col018AL7649G", "name": "Status", "key": "status", "type": "select", "is_primary_column": false, "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}}, {"id": "Col018B8C91UK", "name": "Labels", "key": "labels", "type": "multi_select", "is_primary_column": false, "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}}, {"id": "Col018AL764AE", "name": "Date", "key": "date", "type": "date", "is_primary_column": false}, {"id": "Col018B8C91V1", "name": "Owner", "key": "owner", "type": "user", "is_primary_column": false, "options": {"format": "single_entity", "show_member_name": true}}, {"id": "Col018AL764AW", "name": "Attachments", "key": "attachments", "type": "attachment", "is_primary_column": false}, {"id": "Col018B8C9203", "name": "Ready?", "key": "ready", "type": "checkbox", "is_primary_column": false}, {"id": "Col018AL764BC", "name": "Email", "key": "email", "type": "email", "is_primary_column": false}, {"id": "Col018B8C921H", "name": "Phone", "key": "phone", "type": "phone", "is_primary_column": false}, {"id": "Col018AL764G2", "name": "Channel", "key": "channel", "type": "channel", "is_primary_column": false}, {"id": "Col018B8C9259", "name": "Rating", "key": "rating", "type": "rating", "is_primary_column": false, "options": {"emoji": ":star:", "max": 5, "show_member_name": true}}, {"id": "Col018AL77TH8", "name": "Vote", "key": "vote", "type": "vote", "is_primary_column": false}, {"id": "Col018B8D6Q91", "name": "Assignee", "key": "assignee", "type": "assignee", "is_primary_column": false}, {"id": "Col018AL83THQ", "name": "Due Date", "key": "due_date", "type": "due_date", "is_primary_column": false}, {"id": "Col018AL857QS", "name": "Completed", "key": "completed", "type": "completed", "is_primary_column": false}, {"id": "Col018B8D84R1", "name": "Canvas", "key": "canvas", "type": "canvas", "is_primary_column": false}, {"id": "Col018AL9577U", "name": "Link", "key": "link", "type": "link", "is_primary_column": false}], "views": [{"id": "View018B8E85S7", "name": "Record", "type": "record", "is_locked": false, "position": "1758744341", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744341, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": false}, {"id": "View018AL9598S", "name": "All items", "type": "table", "is_locked": false, "position": "1758744342", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744342, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": true, "default_view_key": "all_items", "show_completed_items": true}], "integrations": [], "icon": "", "description": "", "description_blocks": [{"type": "rich_text", "block_id": "XYcTZ", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Sprint work"}]}]}], "is_trial": false, "subtask_schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}], "creation_source": {"type": "copy_from_list", "reference_id": "F018AL98L3U"}, "todo_mode": false, "default_view": ""}, "list_limits": {"over_row_maximum": false, "row_count_limit": 1000, "row_count": 7, "archived_row_count": 0, "over_column_maximum": false, "column_count": 20, "column_count_limit": 30, "over_view_maximum": false, "view_count": 2, "view_count_limit": 50, "max_attachments_per_cell": 10}, "url_private": "https://files.com/files-pri/T0AB1CD2E-F1234567/list", "url_private_download": "https://files.com/files-pri/T0AB1CD2E-F1234567/download/list", "permalink": "https://someenterpriseorg.slack.com/lists/T0ABCDEFG/F1234567", "permalink_public": "https://files.com/T0AB1CD2E-F1234567-123ab4cd56", "last_editor": "W0QR1STU2", "list_csv_download_url": "https://files.com/files-pri/T0AB1CD2E-F1234567/csv/list", "updated": 1758744347, "is_starred": false, "skipped_shares": true, "teams_shared_with": ["E0AB1CD2E"], "is_restricted_sharing_enabled": false, "has_rich_preview": false, "file_access": "visible", "access": "owner", "org_or_workspace_access": "none", "is_ai_suggested": false}, "record": {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0QR1STU2", "updated_by": "W0QR1STU2", "fields": [], "updated_timestamp": "1758744346"}, "subtasks": []}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.items.info", r#"{"ok": true, "list": {"id": "F1234567", "created": 1758744341, "timestamp": 1758744341, "name": "list", "title": "Sprint Board", "mimetype": "application/vnd.slack-list", "filetype": "list", "pretty_type": "List", "user": "W0AB1CDE2", "user_team": "E0AB1CD2E", "editable": true, "size": 0, "mode": "list", "is_external": false, "external_type": "", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "list_metadata": {"schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}, {"id": "Col018B8C91TM", "name": "Notes", "key": "rich_text_notes", "type": "rich_text", "is_primary_column": false}, {"id": "Col018AL76490", "name": "Message", "key": "message_link", "type": "message", "is_primary_column": false}, {"id": "Col018B8C91U3", "name": "Estimate", "key": "estimate", "type": "number", "is_primary_column": false, "options": {"precision": 2, "show_member_name": true}}, {"id": "Col018AL7649G", "name": "Status", "key": "status", "type": "select", "is_primary_column": false, "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}}, {"id": "Col018B8C91UK", "name": "Labels", "key": "labels", "type": "multi_select", "is_primary_column": false, "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}}, {"id": "Col018AL764AE", "name": "Date", "key": "date", "type": "date", "is_primary_column": false}, {"id": "Col018B8C91V1", "name": "Owner", "key": "owner", "type": "user", "is_primary_column": false, "options": {"format": "single_entity", "show_member_name": true}}, {"id": "Col018AL764AW", "name": "Attachments", "key": "attachments", "type": "attachment", "is_primary_column": false}, {"id": "Col018B8C9203", "name": "Ready?", "key": "ready", "type": "checkbox", "is_primary_column": false}, {"id": "Col018AL764BC", "name": "Email", "key": "email", "type": "email", "is_primary_column": false}, {"id": "Col018B8C921H", "name": "Phone", "key": "phone", "type": "phone", "is_primary_column": false}, {"id": "Col018AL764G2", "name": "Channel", "key": "channel", "type": "channel", "is_primary_column": false}, {"id": "Col018B8C9259", "name": "Rating", "key": "rating", "type": "rating", "is_primary_column": false, "options": {"emoji": ":star:", "max": 5, "show_member_name": true}}, {"id": "Col018AL77TH8", "name": "Vote", "key": "vote", "type": "vote", "is_primary_column": false}, {"id": "Col018B8D6Q91", "name": "Assignee", "key": "assignee", "type": "assignee", "is_primary_column": false}, {"id": "Col018AL83THQ", "name": "Due Date", "key": "due_date", "type": "due_date", "is_primary_column": false}, {"id": "Col018AL857QS", "name": "Completed", "key": "completed", "type": "completed", "is_primary_column": false}, {"id": "Col018B8D84R1", "name": "Canvas", "key": "canvas", "type": "canvas", "is_primary_column": false}, {"id": "Col018AL9577U", "name": "Link", "key": "link", "type": "link", "is_primary_column": false}], "views": [{"id": "View018B8E85S7", "name": "Record", "type": "record", "is_locked": false, "position": "1758744341", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744341, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": false}, {"id": "View018AL9598S", "name": "All items", "type": "table", "is_locked": false, "position": "1758744342", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744342, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": true, "default_view_key": "all_items", "show_completed_items": true}], "integrations": [], "icon": "", "description": "", "description_blocks": [{"type": "rich_text", "block_id": "XYcTZ", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Sprint work"}]}]}], "is_trial": false, "subtask_schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}], "creation_source": {"type": "copy_from_list", "reference_id": "F018AL98L3U"}, "todo_mode": false, "default_view": ""}, "list_limits": {"over_row_maximum": false, "row_count_limit": 1000, "row_count": 7, "archived_row_count": 0, "over_column_maximum": false, "column_count": 20, "column_count_limit": 30, "over_view_maximum": false, "view_count": 2, "view_count_limit": 50, "max_attachments_per_cell": 10}, "url_private": "https://files.com/files-pri/T0AB1CD2E-F1234567/list", "url_private_download": "https://files.com/files-pri/T0AB1CD2E-F1234567/download/list", "permalink": "https://someenterpriseorg.slack.com/lists/T0RTANENP/F1234567", "permalink_public": "https://files.com/T0AB1CD2E-F1234567-490eb8da78", "last_editor": "W0QR1STU2", "list_csv_download_url": "https://files.com/files-pri/T0AB1CD2E-F1234567/csv/list", "updated": 1758744347, "is_starred": false, "skipped_shares": true, "teams_shared_with": ["E0AB1CD2E"], "is_restricted_sharing_enabled": false, "has_rich_preview": false, "file_access": "visible", "access": "owner", "org_or_workspace_access": "none", "is_ai_suggested": false}, "record": {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0QR1STU2", "updated_by": "W0QR1STU2", "fields": [], "updated_timestamp": "1758744346", "is_subscribed": false}, "subtasks": []}"#).await;
        let res = client
            .slack_lists_items_info(&req)
            .await
            .expect("slackLists.items.info");
        assert_preserved(
            "slackLists.items.info",
            r#"{"ok": true, "list": {"id": "F1234567", "created": 1758744341, "timestamp": 1758744341, "name": "list", "title": "Sprint Board", "mimetype": "application/vnd.slack-list", "filetype": "list", "pretty_type": "List", "user": "W0AB1CDE2", "user_team": "E0AB1CD2E", "editable": true, "size": 0, "mode": "list", "is_external": false, "external_type": "", "is_public": false, "public_url_shared": false, "display_as_bot": false, "username": "", "list_metadata": {"schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}, {"id": "Col018B8C91TM", "name": "Notes", "key": "rich_text_notes", "type": "rich_text", "is_primary_column": false}, {"id": "Col018AL76490", "name": "Message", "key": "message_link", "type": "message", "is_primary_column": false}, {"id": "Col018B8C91U3", "name": "Estimate", "key": "estimate", "type": "number", "is_primary_column": false, "options": {"precision": 2, "show_member_name": true}}, {"id": "Col018AL7649G", "name": "Status", "key": "status", "type": "select", "is_primary_column": false, "options": {"choices": [{"value": "not_started", "label": "Not Started", "color": "red"}, {"value": "in_progress", "label": "In Progress", "color": "yellow"}, {"value": "completed", "label": "Completed", "color": "green"}], "format": "single_select", "show_member_name": true}}, {"id": "Col018B8C91UK", "name": "Labels", "key": "labels", "type": "multi_select", "is_primary_column": false, "options": {"choices": [{"value": "p0", "label": "P0", "color": "red"}, {"value": "p1", "label": "P1", "color": "yellow"}, {"value": "p2", "label": "P2", "color": "green"}], "format": "multi_select", "show_member_name": true}}, {"id": "Col018AL764AE", "name": "Date", "key": "date", "type": "date", "is_primary_column": false}, {"id": "Col018B8C91V1", "name": "Owner", "key": "owner", "type": "user", "is_primary_column": false, "options": {"format": "single_entity", "show_member_name": true}}, {"id": "Col018AL764AW", "name": "Attachments", "key": "attachments", "type": "attachment", "is_primary_column": false}, {"id": "Col018B8C9203", "name": "Ready?", "key": "ready", "type": "checkbox", "is_primary_column": false}, {"id": "Col018AL764BC", "name": "Email", "key": "email", "type": "email", "is_primary_column": false}, {"id": "Col018B8C921H", "name": "Phone", "key": "phone", "type": "phone", "is_primary_column": false}, {"id": "Col018AL764G2", "name": "Channel", "key": "channel", "type": "channel", "is_primary_column": false}, {"id": "Col018B8C9259", "name": "Rating", "key": "rating", "type": "rating", "is_primary_column": false, "options": {"emoji": ":star:", "max": 5, "show_member_name": true}}, {"id": "Col018AL77TH8", "name": "Vote", "key": "vote", "type": "vote", "is_primary_column": false}, {"id": "Col018B8D6Q91", "name": "Assignee", "key": "assignee", "type": "assignee", "is_primary_column": false}, {"id": "Col018AL83THQ", "name": "Due Date", "key": "due_date", "type": "due_date", "is_primary_column": false}, {"id": "Col018AL857QS", "name": "Completed", "key": "completed", "type": "completed", "is_primary_column": false}, {"id": "Col018B8D84R1", "name": "Canvas", "key": "canvas", "type": "canvas", "is_primary_column": false}, {"id": "Col018AL9577U", "name": "Link", "key": "link", "type": "link", "is_primary_column": false}], "views": [{"id": "View018B8E85S7", "name": "Record", "type": "record", "is_locked": false, "position": "1758744341", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744341, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": false}, {"id": "View018AL9598S", "name": "All items", "type": "table", "is_locked": false, "position": "1758744342", "columns": [{"visible": true, "key": "title", "id": "Col018AL7648J", "position": "5000000000"}, {"visible": true, "key": "rich_text_notes", "id": "Col018B8C91TM", "position": "5000000001"}, {"visible": true, "key": "message_link", "id": "Col018AL76490", "position": "5000000002"}, {"visible": true, "key": "estimate", "id": "Col018B8C91U3", "position": "5000000003"}, {"visible": true, "key": "status", "id": "Col018AL7649G", "position": "5000000004"}, {"visible": true, "key": "labels", "id": "Col018B8C91UK", "position": "5000000005"}, {"visible": true, "key": "date", "id": "Col018AL764AE", "position": "5000000006"}, {"visible": true, "key": "owner", "id": "Col018B8C91V1", "position": "5000000007"}, {"visible": true, "key": "attachments", "id": "Col018AL764AW", "position": "5000000008"}, {"visible": true, "key": "ready", "id": "Col018B8C9203", "position": "5000000009"}, {"visible": true, "key": "email", "id": "Col018AL764BC", "position": "5000000010"}, {"visible": true, "key": "phone", "id": "Col018B8C921H", "position": "5000000011"}, {"visible": true, "key": "channel", "id": "Col018AL764G2", "position": "5000000012"}, {"visible": true, "key": "rating", "id": "Col018B8C9259", "position": "5000000013"}, {"visible": true, "key": "vote", "id": "Col018AL77TH8", "position": "5000000014"}, {"visible": true, "key": "assignee", "id": "Col018B8D6Q91", "position": "5000000015"}, {"visible": true, "key": "due_date", "id": "Col018AL83THQ", "position": "5000000016"}, {"visible": true, "key": "completed", "id": "Col018AL857QS", "position": "5000000017"}, {"visible": true, "key": "canvas", "id": "Col018B8D84R1", "position": "5000000018"}, {"visible": true, "key": "link", "id": "Col018AL9577U", "position": "5000000019"}], "date_created": 1758744342, "created_by": "W0QR1STU2", "stick_column_left": false, "is_all_items_view": true, "default_view_key": "all_items", "show_completed_items": true}], "integrations": [], "icon": "", "description": "", "description_blocks": [{"type": "rich_text", "block_id": "XYcTZ", "elements": [{"type": "rich_text_section", "elements": [{"type": "text", "text": "Sprint work"}]}]}], "is_trial": false, "subtask_schema": [{"id": "Col018AL7648J", "name": "Title", "key": "title", "type": "text", "is_primary_column": true}], "creation_source": {"type": "copy_from_list", "reference_id": "F018AL98L3U"}, "todo_mode": false, "default_view": ""}, "list_limits": {"over_row_maximum": false, "row_count_limit": 1000, "row_count": 7, "archived_row_count": 0, "over_column_maximum": false, "column_count": 20, "column_count_limit": 30, "over_view_maximum": false, "view_count": 2, "view_count_limit": 50, "max_attachments_per_cell": 10}, "url_private": "https://files.com/files-pri/T0AB1CD2E-F1234567/list", "url_private_download": "https://files.com/files-pri/T0AB1CD2E-F1234567/download/list", "permalink": "https://someenterpriseorg.slack.com/lists/T0RTANENP/F1234567", "permalink_public": "https://files.com/T0AB1CD2E-F1234567-490eb8da78", "last_editor": "W0QR1STU2", "list_csv_download_url": "https://files.com/files-pri/T0AB1CD2E-F1234567/csv/list", "updated": 1758744347, "is_starred": false, "skipped_shares": true, "teams_shared_with": ["E0AB1CD2E"], "is_restricted_sharing_enabled": false, "has_rich_preview": false, "file_access": "visible", "access": "owner", "org_or_workspace_access": "none", "is_ai_suggested": false}, "record": {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0QR1STU2", "updated_by": "W0QR1STU2", "fields": [], "updated_timestamp": "1758744346", "is_subscribed": false}, "subtasks": []}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_items_list() {
    let req = SlackListsItemsListRequest::new("F12345678")
        .limit(1)
        .cursor(
            "bGlzdF9pZDoxMjIxNzk3NzMyNDgzO2lkOjEyNzAxMjMxNTEzOTQ7ZGF0ZV9jcmVhdGVkOjE3NTE1NTkyMTU=",
        )
        .archived(true)
        .include_list(true);
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(SlackListsItemsListResponse::default().next_cursor(), None);
    let page = SlackListsItemsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("slackLists.items.list", r#"{"ok": true, "items": [{"id": "Rec018B8X2B3M", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"08jc0\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire\"}]}]}]", "text": "Onboard new hire", "rich_text": [{"type": "rich_text", "block_id": "08jc0", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}, {"key": "estimate", "value": 3, "number": [3], "column_id": "Col018B8C91U3"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALFLP2N", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALA7RPU", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "status", "value": "completed", "select": ["completed"], "column_id": "Col018AL7649G"}, {"key": "date", "value": "2025-09-19", "date": ["2025-09-19"], "timestamp": [-1], "column_id": "Col018AL764AE"}, {"key": "owner", "value": "U014W31KQMR", "user": ["U014W31KQMR"], "column_id": "Col018B8C91V1"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8LPLG3", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"UXkfa\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire \"},{\"type\":\"text\",\"text\":\"(week 1)\",\"style\":{\"bold\":true}}]}]}]", "text": "Onboard new hire *(week 1)*", "rich_text": [{"type": "rich_text", "block_id": "UXkfa", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire ", "type": "text"}, {"text": "(week 1)", "type": "text", "style": {"bold": true}}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALFLNTU", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"k0zIi\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Fix bug\"}]}]}]", "text": "Fix bug", "rich_text": [{"type": "rich_text", "block_id": "k0zIi", "elements": [{"type": "rich_text_section", "elements": [{"text": "Fix bug", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALE9718", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744345"}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .slack_lists_items_list(&req)
            .await
            .expect("slackLists.items.list");
        assert_preserved(
            "slackLists.items.list",
            r#"{"ok": true, "items": [{"id": "Rec018B8X2B3M", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"08jc0\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire\"}]}]}]", "text": "Onboard new hire", "rich_text": [{"type": "rich_text", "block_id": "08jc0", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}, {"key": "estimate", "value": 3, "number": [3], "column_id": "Col018B8C91U3"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALFLP2N", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALA7RPU", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "status", "value": "completed", "select": ["completed"], "column_id": "Col018AL7649G"}, {"key": "date", "value": "2025-09-19", "date": ["2025-09-19"], "timestamp": [-1], "column_id": "Col018AL764AE"}, {"key": "owner", "value": "U014W31KQMR", "user": ["U014W31KQMR"], "column_id": "Col018B8C91V1"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8LPLG3", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"UXkfa\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire \"},{\"type\":\"text\",\"text\":\"(week 1)\",\"style\":{\"bold\":true}}]}]}]", "text": "Onboard new hire *(week 1)*", "rich_text": [{"type": "rich_text", "block_id": "UXkfa", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire ", "type": "text"}, {"text": "(week 1)", "type": "text", "style": {"bold": true}}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALFLNTU", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"k0zIi\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Fix bug\"}]}]}]", "text": "Fix bug", "rich_text": [{"type": "rich_text", "block_id": "k0zIi", "elements": [{"type": "rich_text_section", "elements": [{"text": "Fix bug", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALE9718", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744345"}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("slackLists.items.list", r#"{"ok": true, "items": [{"id": "Rec018B8X2B3M", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"08jc0\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire\"}]}]}]", "text": "Onboard new hire", "rich_text": [{"type": "rich_text", "block_id": "08jc0", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}, {"key": "estimate", "value": 3, "number": [3], "column_id": "Col018B8C91U3"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALFLP2N", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALA7RPU", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "status", "value": "completed", "select": ["completed"], "column_id": "Col018AL7649G"}, {"key": "date", "value": "2025-09-19", "date": ["2025-09-19"], "timestamp": [-1], "column_id": "Col018AL764AE"}, {"key": "owner", "value": "U014W31KQMR", "user": ["U014W31KQMR"], "column_id": "Col018B8C91V1"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8LPLG3", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"UXkfa\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire \"},{\"type\":\"text\",\"text\":\"(week 1)\",\"style\":{\"bold\":true}}]}]}]", "text": "Onboard new hire *(week 1)*", "rich_text": [{"type": "rich_text", "block_id": "UXkfa", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire ", "type": "text"}, {"text": "(week 1)", "type": "text", "style": {"bold": true}}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALFLNTU", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"k0zIi\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Fix bug\"}]}]}]", "text": "Fix bug", "rich_text": [{"type": "rich_text", "block_id": "k0zIi", "elements": [{"type": "rich_text_section", "elements": [{"text": "Fix bug", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALE9718", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744345"}], "response_metadata": {"next_cursor": ""}}"#).await;
        let res = client
            .slack_lists_items_list(&req)
            .await
            .expect("slackLists.items.list");
        assert_preserved(
            "slackLists.items.list",
            r#"{"ok": true, "items": [{"id": "Rec018B8X2B3M", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"08jc0\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire\"}]}]}]", "text": "Onboard new hire", "rich_text": [{"type": "rich_text", "block_id": "08jc0", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}, {"key": "estimate", "value": 3, "number": [3], "column_id": "Col018B8C91U3"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8RR603", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALFLP2N", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744346"}, {"id": "Rec018ALA7RPU", "list_id": "F1234567", "date_created": 1758744346, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "status", "value": "completed", "select": ["completed"], "column_id": "Col018AL7649G"}, {"key": "date", "value": "2025-09-19", "date": ["2025-09-19"], "timestamp": [-1], "column_id": "Col018AL764AE"}, {"key": "owner", "value": "U014W31KQMR", "user": ["U014W31KQMR"], "column_id": "Col018B8C91V1"}], "updated_timestamp": "1758744346"}, {"id": "Rec018B8LPLG3", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"UXkfa\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Onboard new hire \"},{\"type\":\"text\",\"text\":\"(week 1)\",\"style\":{\"bold\":true}}]}]}]", "text": "Onboard new hire *(week 1)*", "rich_text": [{"type": "rich_text", "block_id": "UXkfa", "elements": [{"type": "rich_text_section", "elements": [{"text": "Onboard new hire ", "type": "text"}, {"text": "(week 1)", "type": "text", "style": {"bold": true}}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALFLNTU", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [{"key": "rich_text_notes", "value": "[{\"type\":\"rich_text\",\"block_id\":\"k0zIi\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Fix bug\"}]}]}]", "text": "Fix bug", "rich_text": [{"type": "rich_text", "block_id": "k0zIi", "elements": [{"type": "rich_text_section", "elements": [{"text": "Fix bug", "type": "text"}]}]}], "column_id": "Col018B8C91TM"}], "updated_timestamp": "1758744345"}, {"id": "Rec018ALE9718", "list_id": "F1234567", "date_created": 1758744345, "created_by": "W0AB1CDE2", "updated_by": "W0AB1CDE2", "fields": [], "updated_timestamp": "1758744345"}], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup(
            "slackLists.items.list",
            r#"{"ok": true, "items": [], "response_metadata": {"next_cursor": ""}}"#,
        )
        .await;
        let res = client
            .slack_lists_items_list(&req)
            .await
            .expect("slackLists.items.list");
        assert_preserved(
            "slackLists.items.list",
            r#"{"ok": true, "items": [], "response_metadata": {"next_cursor": ""}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_items_update() {
    let req = SlackListsItemsUpdateRequest::new("F0123ABC456", vec![serde_json::json!({"k": "v"})]);
    {
        let (_server, client) = setup("slackLists.items.update", r#"{"ok": true}"#).await;
        let res = client
            .slack_lists_items_update(&req)
            .await
            .expect("slackLists.items.update");
        assert_preserved("slackLists.items.update", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn slack_lists_update() {
    let req = SlackListsUpdateRequest::new("x")
        .name("My List")
        .description_blocks(vec![slack_web_api::blocks::Block::from(
            slack_web_api::blocks::DividerBlock::new(),
        )])
        .todo_mode(true);
    {
        let (_server, client) = setup("slackLists.update", r#"{"ok": true}"#).await;
        let res = client
            .slack_lists_update(&req)
            .await
            .expect("slackLists.update");
        assert_preserved("slackLists.update", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn stars_add() {
    let req = StarsAddRequest::new()
        .channel("x")
        .file("x")
        .file_comment("x")
        .timestamp("x");
    {
        let (_server, client) = setup("stars.add", r#"{"ok": true}"#).await;
        let res = client.stars_add(&req).await.expect("stars.add");
        assert_preserved("stars.add", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn stars_list() {
    let req = StarsListRequest::new()
        .count(1)
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1)
        .page(1)
        .team_id("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(StarsListResponse::default().next_cursor(), None);
    let page = StarsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("stars.list", r#"{"ok": true, "items": [{"type": "message", "channel": "C123ABC456", "message": {"type": "message", "subtype": "bot_message", "text": "", "ts": "1655762568.324229", "username": "username", "icons": {"emoji": ":test:"}, "bot_id": "BSLACKBOT", "attachments": [{"color": "ecb438", "ts": 1655762568, "id": 1, "fallback": "some text", "text": "some text", "pretext": "*chat.postMessage*", "mrkdwn_in": ["pretext", "text"]}], "permalink": "https://your-workspace.slack.com/archives/C123ABC456/p123456789"}, "date_create": 1656014995}], "paging": {"per_page": 100, "spill": 0, "page": 1, "total": 1, "pages": 1}}"#).await;
        let res = client.stars_list(&req).await.expect("stars.list");
        assert_preserved(
            "stars.list",
            r#"{"ok": true, "items": [{"type": "message", "channel": "C123ABC456", "message": {"type": "message", "subtype": "bot_message", "text": "", "ts": "1655762568.324229", "username": "username", "icons": {"emoji": ":test:"}, "bot_id": "BSLACKBOT", "attachments": [{"color": "ecb438", "ts": 1655762568, "id": 1, "fallback": "some text", "text": "some text", "pretext": "*chat.postMessage*", "mrkdwn_in": ["pretext", "text"]}], "permalink": "https://your-workspace.slack.com/archives/C123ABC456/p123456789"}, "date_create": 1656014995}], "paging": {"per_page": 100, "spill": 0, "page": 1, "total": 1, "pages": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn stars_remove() {
    let req = StarsRemoveRequest::new()
        .channel("x")
        .file("x")
        .file_comment("x")
        .timestamp("x");
    {
        let (_server, client) = setup("stars.remove", r#"{"ok": true}"#).await;
        let res = client.stars_remove(&req).await.expect("stars.remove");
        assert_preserved("stars.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_access_logs() {
    let req = TeamAccessLogsRequest::new()
        .before("1457989166")
        .count("x")
        .page("x")
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1)
        .team_id("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(TeamAccessLogsResponse::default().next_cursor(), None);
    let page = TeamAccessLogsResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("team.accessLogs", r#"{"ok": true, "logins": [{"user_id": "U45678", "username": "alice", "date_first": 1422922864, "date_last": 1422922864, "count": 1, "ip": "127.0.0.1", "user_agent": "SlackWeb Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.35 Safari/537.36", "isp": "BigCo ISP", "country": "US", "region": "CA"}, {"user_id": "U12345", "username": "white_rabbit", "date_first": 1422922493, "date_last": 1422922493, "count": 1, "ip": "127.0.0.1", "user_agent": "SlackWeb Mozilla/5.0 (iPhone; CPU iPhone OS 8_1_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12B466 Safari/600.1.4", "isp": "BigCo ISP", "country": "US", "region": "CA"}], "paging": {"count": 100, "total": 2, "page": 1, "pages": 1}}"#).await;
        let res = client
            .team_access_logs(&req)
            .await
            .expect("team.accessLogs");
        assert_preserved(
            "team.accessLogs",
            r#"{"ok": true, "logins": [{"user_id": "U45678", "username": "alice", "date_first": 1422922864, "date_last": 1422922864, "count": 1, "ip": "127.0.0.1", "user_agent": "SlackWeb Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.35 Safari/537.36", "isp": "BigCo ISP", "country": "US", "region": "CA"}, {"user_id": "U12345", "username": "white_rabbit", "date_first": 1422922493, "date_last": 1422922493, "count": 1, "ip": "127.0.0.1", "user_agent": "SlackWeb Mozilla/5.0 (iPhone; CPU iPhone OS 8_1_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12B466 Safari/600.1.4", "isp": "BigCo ISP", "country": "US", "region": "CA"}], "paging": {"count": 100, "total": 2, "page": 1, "pages": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_billable_info() {
    let req = TeamBillableInfoRequest::new()
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .limit(1)
        .user("x")
        .team_id("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(TeamBillableInfoResponse::default().next_cursor(), None);
    let page = TeamBillableInfoResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("team.billableInfo", r#"{"ok": true, "billable_info": {"U0632EWRW": {"billing_active": false}, "U02UCPE1R": {"billing_active": true}, "U02UEBSD2": {"billing_active": true}}}"#).await;
        let res = client
            .team_billable_info(&req)
            .await
            .expect("team.billableInfo");
        assert_preserved(
            "team.billableInfo",
            r#"{"ok": true, "billable_info": {"U0632EWRW": {"billing_active": false}, "U02UCPE1R": {"billing_active": true}, "U02UEBSD2": {"billing_active": true}}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_billing_info() {
    let req = TeamBillingInfoRequest::new();
    {
        let (_server, client) = setup("team.billing.info", r#"{"ok": true, "plan": "free"}"#).await;
        let res = client
            .team_billing_info(&req)
            .await
            .expect("team.billing.info");
        assert_preserved("team.billing.info", r#"{"ok": true, "plan": "free"}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_external_teams_disconnect() {
    let req = TeamExternalTeamsDisconnectRequest::new("T726G27TT");
    {
        let (_server, client) = setup("team.externalTeams.disconnect", r#"{"ok": true}"#).await;
        let res = client
            .team_external_teams_disconnect(&req)
            .await
            .expect("team.externalTeams.disconnect");
        assert_preserved("team.externalTeams.disconnect", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_external_teams_list() {
    let req = TeamExternalTeamsListRequest::new()
        .limit(1)
        .cursor("T123ABC456")
        .sort_field("x")
        .sort_direction("x")
        .slack_connect_pref_filter(vec![serde_json::json!({"k": "v"})])
        .workspace_filter(vec![serde_json::json!({"k": "v"})])
        .connection_status_filter("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(TeamExternalTeamsListResponse::default().next_cursor(), None);
    let page = TeamExternalTeamsListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("team.externalTeams.list", r#"{"ok": true, "organizations": [{"team_id": "T123ABC456", "team_name": "Sandra Inc.", "team_domain": "sandra", "public_channel_count": 1, "private_channel_count": 1, "im_channel_count": 1, "mpim_channel_count": 1, "connected_workspaces": {"workspace_id": "Jesse Inc", "workspace_name": "E123ABC456"}, "slack_connect_prefs": {}, "connection_status": "CONNECTED", "last_active_timestamp": 1718656058, "is_sponsored": false, "canvas": {"total_count": 1, "ownership_details": [{"team_id": "T123ABC456"}, {"count": 1}]}, "lists": {"total_count": 1, "ownership_details": [{"team_id": "T123ABC456"}, {"count": 1}]}}], "total_count": 1, "response_metadata": {"next_cursor": "T123ABC999"}}"#).await;
        let res = client
            .team_external_teams_list(&req)
            .await
            .expect("team.externalTeams.list");
        assert_preserved(
            "team.externalTeams.list",
            r#"{"ok": true, "organizations": [{"team_id": "T123ABC456", "team_name": "Sandra Inc.", "team_domain": "sandra", "public_channel_count": 1, "private_channel_count": 1, "im_channel_count": 1, "mpim_channel_count": 1, "connected_workspaces": {"workspace_id": "Jesse Inc", "workspace_name": "E123ABC456"}, "slack_connect_prefs": {}, "connection_status": "CONNECTED", "last_active_timestamp": 1718656058, "is_sponsored": false, "canvas": {"total_count": 1, "ownership_details": [{"team_id": "T123ABC456"}, {"count": 1}]}, "lists": {"total_count": 1, "ownership_details": [{"team_id": "T123ABC456"}, {"count": 1}]}}], "total_count": 1, "response_metadata": {"next_cursor": "T123ABC999"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_info() {
    let req = TeamInfoRequest::new().domain("x").team("x");
    {
        let (_server, client) = setup("team.info", r#"{"ok": true, "team": {"id": "T12345", "name": "My Team", "domain": "example", "email_domain": "example.com", "icon": {"image_34": "https://...", "image_44": "https://...", "image_68": "https://...", "image_88": "https://...", "image_102": "https://...", "image_132": "https://...", "image_default": true}, "enterprise_id": "E1234A12AB", "enterprise_name": "Umbrella Corporation"}}"#).await;
        let res = client.team_info(&req).await.expect("team.info");
        assert_preserved(
            "team.info",
            r#"{"ok": true, "team": {"id": "T12345", "name": "My Team", "domain": "example", "email_domain": "example.com", "icon": {"image_34": "https://...", "image_44": "https://...", "image_68": "https://...", "image_88": "https://...", "image_102": "https://...", "image_132": "https://...", "image_default": true}, "enterprise_id": "E1234A12AB", "enterprise_name": "Umbrella Corporation"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_integration_logs() {
    let req = TeamIntegrationLogsRequest::new()
        .app_id("x")
        .change_type("added")
        .count("x")
        .page("x")
        .service_id("x")
        .team_id("x")
        .user("x");
    {
        let (_server, client) = setup("team.integrationLogs", r#"{"ok": true}"#).await;
        let res = client
            .team_integration_logs(&req)
            .await
            .expect("team.integrationLogs");
        assert_preserved("team.integrationLogs", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("team.integrationLogs", r#"{"ok": true, "logs": [{"service_id": 1234567890, "service_type": "Google Calendar", "user_id": "U1234ABCD", "user_name": "Johnny", "channel": "C1234567890", "date": "1392163200", "change_type": "enabled", "scope": "incoming-webhook"}, {"app_id": "2345678901", "app_type": "Johnny App", "user_id": "U2345BCDE", "user_name": "Billy", "date": "1392163201", "change_type": "added", "scope": "chat:write:user,channels:read"}, {"service_id": "3456789012", "service_type": "Airbrake", "user_id": "U3456CDEF", "user_name": "Joey", "channel": "C1234567890", "date": "1392163202", "change_type": "disabled", "reason": "user", "scope": "incoming-webhook"}], "paging": {"count": 3, "total": 3, "page": 1, "pages": 1}}"#).await;
        let res = client
            .team_integration_logs(&req)
            .await
            .expect("team.integrationLogs");
        assert_preserved(
            "team.integrationLogs",
            r#"{"ok": true, "logs": [{"service_id": 1234567890, "service_type": "Google Calendar", "user_id": "U1234ABCD", "user_name": "Johnny", "channel": "C1234567890", "date": "1392163200", "change_type": "enabled", "scope": "incoming-webhook"}, {"app_id": "2345678901", "app_type": "Johnny App", "user_id": "U2345BCDE", "user_name": "Billy", "date": "1392163201", "change_type": "added", "scope": "chat:write:user,channels:read"}, {"service_id": "3456789012", "service_type": "Airbrake", "user_id": "U3456CDEF", "user_name": "Joey", "channel": "C1234567890", "date": "1392163202", "change_type": "disabled", "reason": "user", "scope": "incoming-webhook"}], "paging": {"count": 3, "total": 3, "page": 1, "pages": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_preferences_list() {
    let req = TeamPreferencesListRequest::new();
    {
        let (_server, client) = setup("team.preferences.list", r#"{"ok": true, "display_real_names": false, "disable_file_uploads": "disable_all", "msg_edit_window_mins": 25, "who_can_post_general": "everyone"}"#).await;
        let res = client
            .team_preferences_list(&req)
            .await
            .expect("team.preferences.list");
        assert_preserved(
            "team.preferences.list",
            r#"{"ok": true, "display_real_names": false, "disable_file_uploads": "disable_all", "msg_edit_window_mins": 25, "who_can_post_general": "everyone"}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn team_profile_get() {
    let req = TeamProfileGetRequest::new().visibility("all");
    {
        let (_server, client) = setup("team.profile.get", r#"{"ok": true, "profile": {"fields": [{"id": "111111ABC", "ordering": 0, "label": "Phone extension", "hint": "Enter the extension to reach your desk", "type": "text", "possible_values": null, "options": {"is_scim": true, "is_protected": true}, "is_hidden": false, "section_id": "123ABC"}, {"id": "222222ABC", "ordering": 1, "label": "Date of birth", "hint": "When you were born", "type": "date", "possible_values": null, "options": {"is_scim": true, "is_protected": true}, "is_hidden": true, "section_id": "123ABC"}, {"id": "333333ABC", "ordering": 2, "label": "House", "hint": "Put on the sorting hat", "type": "options_list", "possible_values": ["Gryffindor", "Hufflepuff", "Ravenclaw", "Slytherin"], "options": {"is_scim": false, "is_protected": false}, "is_hidden": false, "section_id": "456DEF"}], "sections": [{"id": "123ABC", "team_id": "T123456", "section_type": "contact", "label": "Contact Information", "order": 1, "is_hidden": true}, {"id": "456DEF", "team_id": "T123456", "section_type": "custom", "label": "About Me", "order": 2, "is_hidden": true}]}}"#).await;
        let res = client
            .team_profile_get(&req)
            .await
            .expect("team.profile.get");
        assert_preserved(
            "team.profile.get",
            r#"{"ok": true, "profile": {"fields": [{"id": "111111ABC", "ordering": 0, "label": "Phone extension", "hint": "Enter the extension to reach your desk", "type": "text", "possible_values": null, "options": {"is_scim": true, "is_protected": true}, "is_hidden": false, "section_id": "123ABC"}, {"id": "222222ABC", "ordering": 1, "label": "Date of birth", "hint": "When you were born", "type": "date", "possible_values": null, "options": {"is_scim": true, "is_protected": true}, "is_hidden": true, "section_id": "123ABC"}, {"id": "333333ABC", "ordering": 2, "label": "House", "hint": "Put on the sorting hat", "type": "options_list", "possible_values": ["Gryffindor", "Hufflepuff", "Ravenclaw", "Slytherin"], "options": {"is_scim": false, "is_protected": false}, "is_hidden": false, "section_id": "456DEF"}], "sections": [{"id": "123ABC", "team_id": "T123456", "section_type": "contact", "label": "Contact Information", "order": 1, "is_hidden": true}, {"id": "456DEF", "team_id": "T123456", "section_type": "custom", "label": "About Me", "order": 2, "is_hidden": true}]}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn tooling_tokens_rotate() {
    let req = ToolingTokensRotateRequest::new("xoxe-EXAMPLE");
    {
        let (_server, client) = setup("tooling.tokens.rotate", r#"{"ok": true, "token": "xoxe.xoxp-...", "refresh_token": "xoxe-...", "team_id": "...", "user_id": "...", "iat": 1633095660, "exp": 1633138860}"#).await;
        let res = client
            .tooling_tokens_rotate(&req)
            .await
            .expect("tooling.tokens.rotate");
        assert_preserved(
            "tooling.tokens.rotate",
            r#"{"ok": true, "token": "xoxe.xoxp-...", "refresh_token": "xoxe-...", "team_id": "...", "user_id": "...", "iat": 1633095660, "exp": 1633138860}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn usergroups_create() {
    let req = UsergroupsCreateRequest::new("My Test Team")
        .channels(vec!["A1".to_string(), "A2".to_string()])
        .additional_channels(vec!["A1".to_string(), "A2".to_string()])
        .description("x")
        .handle("x")
        .include_count(true)
        .team_id("x")
        .enable_section(true);
    {
        let (_server, client) = setup("usergroups.create", r#"{"ok": true}"#).await;
        let res = client
            .usergroups_create(&req)
            .await
            .expect("usergroups.create");
        assert_preserved("usergroups.create", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("usergroups.create", r#"{"ok": true, "usergroup": {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446746793, "date_delete": 0, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "0"}}"#).await;
        let res = client
            .usergroups_create(&req)
            .await
            .expect("usergroups.create");
        assert_preserved(
            "usergroups.create",
            r#"{"ok": true, "usergroup": {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446746793, "date_delete": 0, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "0"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn usergroups_disable() {
    let req = UsergroupsDisableRequest::new("S0604QSJC")
        .include_count(true)
        .team_id("x");
    {
        let (_server, client) = setup("usergroups.disable", r#"{"ok": true}"#).await;
        let res = client
            .usergroups_disable(&req)
            .await
            .expect("usergroups.disable");
        assert_preserved("usergroups.disable", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("usergroups.disable", r#"{"ok": true, "usergroup": {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446747568, "date_delete": 1446747568, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": "U060RNRCZ", "prefs": {"channels": [], "groups": []}, "user_count": "0"}}"#).await;
        let res = client
            .usergroups_disable(&req)
            .await
            .expect("usergroups.disable");
        assert_preserved(
            "usergroups.disable",
            r#"{"ok": true, "usergroup": {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446747568, "date_delete": 1446747568, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": "U060RNRCZ", "prefs": {"channels": [], "groups": []}, "user_count": "0"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn usergroups_enable() {
    let req = UsergroupsEnableRequest::new("S0604QSJC")
        .include_count(true)
        .team_id("x");
    {
        let (_server, client) = setup("usergroups.enable", r#"{"ok": true}"#).await;
        let res = client
            .usergroups_enable(&req)
            .await
            .expect("usergroups.enable");
        assert_preserved("usergroups.enable", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("usergroups.enable", r#"{"ok": true, "usergroup": {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446747767, "date_delete": 0, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "0"}}"#).await;
        let res = client
            .usergroups_enable(&req)
            .await
            .expect("usergroups.enable");
        assert_preserved(
            "usergroups.enable",
            r#"{"ok": true, "usergroup": {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446747767, "date_delete": 0, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "0"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn usergroups_list() {
    let req = UsergroupsListRequest::new()
        .include_count(true)
        .include_disabled(true)
        .include_users(true)
        .team_id("x");
    {
        let (_server, client) = setup("usergroups.list", r#"{"ok": true, "usergroups": [{"id": "S0614TZR7", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Team Admins", "description": "A group of all Administrators on your team.", "handle": "admins", "is_external": false, "date_create": 1446598059, "date_update": 1446670362, "date_delete": 0, "auto_type": "admin", "created_by": "USLACKBOT", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "2"}, {"id": "S06158AV7", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Team Owners", "description": "A group of all Owners on your team.", "handle": "owners", "is_external": false, "date_create": 1446678371, "date_update": 1446678371, "date_delete": 0, "auto_type": "owner", "created_by": "USLACKBOT", "updated_by": "USLACKBOT", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "1"}, {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446747767, "date_delete": 1446748865, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "0"}]}"#).await;
        let res = client.usergroups_list(&req).await.expect("usergroups.list");
        assert_preserved(
            "usergroups.list",
            r#"{"ok": true, "usergroups": [{"id": "S0614TZR7", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Team Admins", "description": "A group of all Administrators on your team.", "handle": "admins", "is_external": false, "date_create": 1446598059, "date_update": 1446670362, "date_delete": 0, "auto_type": "admin", "created_by": "USLACKBOT", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "2"}, {"id": "S06158AV7", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Team Owners", "description": "A group of all Owners on your team.", "handle": "owners", "is_external": false, "date_create": 1446678371, "date_update": 1446678371, "date_delete": 0, "auto_type": "owner", "created_by": "USLACKBOT", "updated_by": "USLACKBOT", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "1"}, {"id": "S0615G0KT", "team_id": "T060RNRCH", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1446746793, "date_update": 1446747767, "date_delete": 1446748865, "auto_type": null, "created_by": "U060RNRCZ", "updated_by": "U060RNRCZ", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "user_count": "0"}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn usergroups_update() {
    let req = UsergroupsUpdateRequest::new("S0604QSJC")
        .channels(vec!["A1".to_string(), "A2".to_string()])
        .additional_channels(vec!["A1".to_string(), "A2".to_string()])
        .description("x")
        .handle("x")
        .include_count(true)
        .name("My Test Team")
        .team_id("x")
        .enable_section(true);
    {
        let (_server, client) = setup("usergroups.update", r#"{"ok": true, "usergroup": {"id": "S0616NG6M", "team_id": "T060R4BHN", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1447096577, "date_update": 1447102109, "date_delete": 0, "auto_type": null, "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "users": ["U060R4BJ4", "U060RNRCZ"], "user_count": 1}}"#).await;
        let res = client
            .usergroups_update(&req)
            .await
            .expect("usergroups.update");
        assert_preserved(
            "usergroups.update",
            r#"{"ok": true, "usergroup": {"id": "S0616NG6M", "team_id": "T060R4BHN", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1447096577, "date_update": 1447102109, "date_delete": 0, "auto_type": null, "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "users": ["U060R4BJ4", "U060RNRCZ"], "user_count": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn usergroups_users_list() {
    let req = UsergroupsUsersListRequest::new("S0604QSJC")
        .include_disabled(true)
        .team_id("x");
    {
        let (_server, client) = setup(
            "usergroups.users.list",
            r#"{"ok": true, "users": ["U060R4BJ4", "W123A4BC5"]}"#,
        )
        .await;
        let res = client
            .usergroups_users_list(&req)
            .await
            .expect("usergroups.users.list");
        assert_preserved(
            "usergroups.users.list",
            r#"{"ok": true, "users": ["U060R4BJ4", "W123A4BC5"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn usergroups_users_update() {
    let req =
        UsergroupsUsersUpdateRequest::new("S0604QSJC", vec!["A1".to_string(), "A2".to_string()])
            .include_count(true)
            .team_id("x")
            .additional_channels(vec!["A1".to_string(), "A2".to_string()])
            .is_shared(true);
    {
        let (_server, client) = setup("usergroups.users.update", r#"{"ok": true, "usergroup": {"id": "S0616NG6M", "team_id": "T060R4BHN", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1447096577, "date_update": 1447102109, "date_delete": 0, "auto_type": null, "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "users": ["U060R4BJ4", "U060RNRCZ"], "user_count": 1}}"#).await;
        let res = client
            .usergroups_users_update(&req)
            .await
            .expect("usergroups.users.update");
        assert_preserved(
            "usergroups.users.update",
            r#"{"ok": true, "usergroup": {"id": "S0616NG6M", "team_id": "T060R4BHN", "is_usergroup": true, "name": "Marketing Team", "description": "Marketing gurus, PR experts and product advocates.", "handle": "marketing-team", "is_external": false, "date_create": 1447096577, "date_update": 1447102109, "date_delete": 0, "auto_type": null, "created_by": "U060R4BJ4", "updated_by": "U060R4BJ4", "deleted_by": null, "prefs": {"channels": [], "groups": []}, "users": ["U060R4BJ4", "U060RNRCZ"], "user_count": 1}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_conversations() {
    let req = UsersConversationsRequest::new()
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .exclude_archived(true)
        .exclude_muted(true)
        .limit(1)
        .team_id("x")
        .types("im,mpim")
        .user("W0B2345D");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(UsersConversationsResponse::default().next_cursor(), None);
    let page = UsersConversationsResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("users.conversations", r#"{"ok": true, "channels": [{"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U012A3CDE", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_private": false, "is_mpim": false, "topic": {"value": "Company-wide announcements and work-based matters", "creator": "", "last_set": 0}, "purpose": {"value": "This channel is for team-wide communication and announcements. All team members are in this channel.", "creator": "", "last_set": 0}, "previous_names": []}, {"id": "C061EG9T2", "name": "random", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "random", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_private": false, "is_mpim": false, "topic": {"value": "Non-work banter and water cooler conversation", "creator": "", "last_set": 0}, "purpose": {"value": "A place for non-work-related flimflam, faffing, hodge-podge or jibber-jabber you'd prefer to keep out of more focused work-related channels.", "creator": "", "last_set": 0}, "previous_names": []}], "response_metadata": {"next_cursor": "dGVhbTpDMDYxRkE1UEI="}}"#).await;
        let res = client
            .users_conversations(&req)
            .await
            .expect("users.conversations");
        assert_preserved(
            "users.conversations",
            r#"{"ok": true, "channels": [{"id": "C012AB3CD", "name": "general", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U012A3CDE", "is_archived": false, "is_general": true, "unlinked": 0, "name_normalized": "general", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_private": false, "is_mpim": false, "topic": {"value": "Company-wide announcements and work-based matters", "creator": "", "last_set": 0}, "purpose": {"value": "This channel is for team-wide communication and announcements. All team members are in this channel.", "creator": "", "last_set": 0}, "previous_names": []}, {"id": "C061EG9T2", "name": "random", "is_channel": true, "is_group": false, "is_im": false, "created": 1449252889, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "random", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_private": false, "is_mpim": false, "topic": {"value": "Non-work banter and water cooler conversation", "creator": "", "last_set": 0}, "purpose": {"value": "A place for non-work-related flimflam, faffing, hodge-podge or jibber-jabber you'd prefer to keep out of more focused work-related channels.", "creator": "", "last_set": 0}, "previous_names": []}], "response_metadata": {"next_cursor": "dGVhbTpDMDYxRkE1UEI="}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("users.conversations", r#"{"ok": true, "channels": [{"id": "G0AKFJBEU", "name": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_channel": false, "is_group": true, "is_im": false, "created": 1493657761, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_private": true, "is_mpim": true, "is_open": true, "topic": {"value": "Group messaging", "creator": "U061F7AUR", "last_set": 1493657761}, "purpose": {"value": "Group messaging with: @mr.banks @slactions-jackson @beforebot", "creator": "U061F7AUR", "last_set": 1493657761}, "priority": 0}, {"id": "D0C0F7S8Y", "created": 1498500348, "is_im": true, "is_org_shared": false, "user": "U0BS9U4SV", "is_user_deleted": false, "priority": 0}, {"id": "D0BSHH4AD", "created": 1498511030, "is_im": true, "is_org_shared": false, "user": "U0C0NS9HN", "is_user_deleted": false, "priority": 0}], "response_metadata": {"next_cursor": "aW1faWQ6RDBCSDk1RExI"}}"#).await;
        let res = client
            .users_conversations(&req)
            .await
            .expect("users.conversations");
        assert_preserved(
            "users.conversations",
            r#"{"ok": true, "channels": [{"id": "G0AKFJBEU", "name": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_channel": false, "is_group": true, "is_im": false, "created": 1493657761, "creator": "U061F7AUR", "is_archived": false, "is_general": false, "unlinked": 0, "name_normalized": "mpdm-mr.banks--slactions-jackson--beforebot-1", "is_shared": false, "is_ext_shared": false, "is_org_shared": false, "pending_shared": [], "is_pending_ext_shared": false, "is_private": true, "is_mpim": true, "is_open": true, "topic": {"value": "Group messaging", "creator": "U061F7AUR", "last_set": 1493657761}, "purpose": {"value": "Group messaging with: @mr.banks @slactions-jackson @beforebot", "creator": "U061F7AUR", "last_set": 1493657761}, "priority": 0}, {"id": "D0C0F7S8Y", "created": 1498500348, "is_im": true, "is_org_shared": false, "user": "U0BS9U4SV", "is_user_deleted": false, "priority": 0}, {"id": "D0BSHH4AD", "created": 1498511030, "is_im": true, "is_org_shared": false, "user": "U0C0NS9HN", "is_user_deleted": false, "priority": 0}], "response_metadata": {"next_cursor": "aW1faWQ6RDBCSDk1RExI"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_delete_photo() {
    let req = UsersDeletePhotoRequest::new();
    {
        let (_server, client) = setup("users.deletePhoto", r#"{"ok": true}"#).await;
        let res = client
            .users_delete_photo(&req)
            .await
            .expect("users.deletePhoto");
        assert_preserved("users.deletePhoto", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_discoverable_contacts_lookup() {
    let req = UsersDiscoverableContactsLookupRequest::new("scott.slacksalot@example.com");
    {
        let (_server, client) = setup(
            "users.discoverableContacts.lookup",
            r#"{"ok": true, "is_discoverable": true}"#,
        )
        .await;
        let res = client
            .users_discoverable_contacts_lookup(&req)
            .await
            .expect("users.discoverableContacts.lookup");
        assert_preserved(
            "users.discoverableContacts.lookup",
            r#"{"ok": true, "is_discoverable": true}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup(
            "users.discoverableContacts.lookup",
            r#"{"ok": true, "is_discoverable": false}"#,
        )
        .await;
        let res = client
            .users_discoverable_contacts_lookup(&req)
            .await
            .expect("users.discoverableContacts.lookup");
        assert_preserved(
            "users.discoverableContacts.lookup",
            r#"{"ok": true, "is_discoverable": false}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_get_presence() {
    let req = UsersGetPresenceRequest::new().user("x");
    {
        let (_server, client) =
            setup("users.getPresence", r#"{"ok": true, "presence": "active"}"#).await;
        let res = client
            .users_get_presence(&req)
            .await
            .expect("users.getPresence");
        assert_preserved(
            "users.getPresence",
            r#"{"ok": true, "presence": "active"}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("users.getPresence", r#"{"ok": true, "presence": "active", "online": true, "auto_away": false, "manual_away": false, "connection_count": 1, "last_activity": 1419027078}"#).await;
        let res = client
            .users_get_presence(&req)
            .await
            .expect("users.getPresence");
        assert_preserved(
            "users.getPresence",
            r#"{"ok": true, "presence": "active", "online": true, "auto_away": false, "manual_away": false, "connection_count": 1, "last_activity": 1419027078}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_identity() {
    let req = UsersIdentityRequest::new();
    {
        let (_server, client) = setup("users.identity", r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6"}, "team": {"id": "T0G9PQBBK"}}"#).await;
        let res = client.users_identity(&req).await.expect("users.identity");
        assert_preserved(
            "users.identity",
            r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6"}, "team": {"id": "T0G9PQBBK"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("users.identity", r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6", "email": "bobby@example.com"}, "team": {"id": "T0G9PQBBK"}}"#).await;
        let res = client.users_identity(&req).await.expect("users.identity");
        assert_preserved(
            "users.identity",
            r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6", "email": "bobby@example.com"}, "team": {"id": "T0G9PQBBK"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("users.identity", r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6", "image_24": "https://cdn.example.com/sonny_24.jpg", "image_32": "https://cdn.example.com/sonny_32.jpg", "image_48": "https://cdn.example.com/sonny_48.jpg", "image_72": "https://cdn.example.com/sonny_72.jpg", "image_192": "https://cdn.example.com/sonny_192.jpg"}, "team": {"id": "T0G9PQBBK"}}"#).await;
        let res = client.users_identity(&req).await.expect("users.identity");
        assert_preserved(
            "users.identity",
            r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6", "image_24": "https://cdn.example.com/sonny_24.jpg", "image_32": "https://cdn.example.com/sonny_32.jpg", "image_48": "https://cdn.example.com/sonny_48.jpg", "image_72": "https://cdn.example.com/sonny_72.jpg", "image_192": "https://cdn.example.com/sonny_192.jpg"}, "team": {"id": "T0G9PQBBK"}}"#,
            &res,
        );
    }
    {
        let (_server, client) = setup("users.identity", r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6"}, "team": {"name": "Captain Fabian's Naval Supply", "id": "T0G9PQBBK"}}"#).await;
        let res = client.users_identity(&req).await.expect("users.identity");
        assert_preserved(
            "users.identity",
            r#"{"ok": true, "user": {"name": "Sonny Whether", "id": "U0G9QF9C6"}, "team": {"name": "Captain Fabian's Naval Supply", "id": "T0G9PQBBK"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_info() {
    let req = UsersInfoRequest::new().include_locale(true).user("x");
    {
        let (_server, client) = setup("users.info", r#"{"ok": true, "user": {"id": "W012A3CDE", "team_id": "T012AB3C4", "name": "spengler", "deleted": false, "color": "9f69e7", "real_name": "Egon Spengler", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "ge3b51ca72de", "status_text": "Print is dead", "status_emoji": ":books:", "real_name": "Egon Spengler", "display_name": "spengler", "real_name_normalized": "Egon Spengler", "display_name_normalized": "spengler", "email": "spengler@ghostbusters.example.com", "image_original": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_24": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_32": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_48": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_72": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_192": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_512": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "team": "T012AB3C4"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1502138686, "is_app_user": false, "has_2fa": false}}"#).await;
        let res = client.users_info(&req).await.expect("users.info");
        assert_preserved(
            "users.info",
            r#"{"ok": true, "user": {"id": "W012A3CDE", "team_id": "T012AB3C4", "name": "spengler", "deleted": false, "color": "9f69e7", "real_name": "Egon Spengler", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "ge3b51ca72de", "status_text": "Print is dead", "status_emoji": ":books:", "real_name": "Egon Spengler", "display_name": "spengler", "real_name_normalized": "Egon Spengler", "display_name_normalized": "spengler", "email": "spengler@ghostbusters.example.com", "image_original": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_24": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_32": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_48": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_72": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_192": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_512": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "team": "T012AB3C4"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1502138686, "is_app_user": false, "has_2fa": false}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_list() {
    let req = UsersListRequest::new()
        .cursor("dXNlcjpVMDYxTkZUVDI=")
        .include_locale(true)
        .limit(1)
        .team_id("x");
    let mut next = req.clone();
    next.set_cursor("c2".into());
    assert_eq!(next.cursor.as_deref(), Some("c2"));
    assert_eq!(UsersListResponse::default().next_cursor(), None);
    let page = UsersListResponse {
        response_metadata: Some(ResponseMetadata {
            next_cursor: Some("n".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(page.next_cursor(), Some("n"));
    {
        let (_server, client) = setup("users.list", r#"{"ok": true, "members": [{"id": "W012A3CDE", "team_id": "T012AB3C4", "name": "spengler", "deleted": false, "color": "9f69e7", "real_name": "spengler", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "ge3b51ca72de", "status_text": "Print is dead", "status_emoji": ":books:", "real_name": "Egon Spengler", "display_name": "spengler", "real_name_normalized": "Egon Spengler", "display_name_normalized": "spengler", "email": "spengler@ghostbusters.example.com", "image_24": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_32": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_48": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_72": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_192": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_512": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "team": "T012AB3C4"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1502138686, "is_app_user": false, "has_2fa": false}, {"id": "W07QCRPA4", "team_id": "T0G9PQBBK", "name": "glinda", "deleted": false, "color": "9f69e7", "real_name": "Glinda Southgood", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "8fbdd10b41c6", "image_24": "https://a.slack-edge.com...png", "image_32": "https://a.slack-edge.com...png", "image_48": "https://a.slack-edge.com...png", "image_72": "https://a.slack-edge.com...png", "image_192": "https://a.slack-edge.com...png", "image_512": "https://a.slack-edge.com...png", "image_1024": "https://a.slack-edge.com...png", "image_original": "https://a.slack-edge.com...png", "first_name": "Glinda", "last_name": "Southgood", "title": "Glinda the Good", "phone": "", "skype": "", "real_name": "Glinda Southgood", "real_name_normalized": "Glinda Southgood", "display_name": "Glinda the Fairly Good", "display_name_normalized": "Glinda the Fairly Good", "email": "glenda@south.oz.coven"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1480527098, "has_2fa": false}], "cache_ts": 1498777272, "response_metadata": {"next_cursor": "dXNlcjpVMEc5V0ZYTlo="}}"#).await;
        let res = client.users_list(&req).await.expect("users.list");
        assert_preserved(
            "users.list",
            r#"{"ok": true, "members": [{"id": "W012A3CDE", "team_id": "T012AB3C4", "name": "spengler", "deleted": false, "color": "9f69e7", "real_name": "spengler", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "ge3b51ca72de", "status_text": "Print is dead", "status_emoji": ":books:", "real_name": "Egon Spengler", "display_name": "spengler", "real_name_normalized": "Egon Spengler", "display_name_normalized": "spengler", "email": "spengler@ghostbusters.example.com", "image_24": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_32": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_48": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_72": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_192": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_512": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "team": "T012AB3C4"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1502138686, "is_app_user": false, "has_2fa": false}, {"id": "W07QCRPA4", "team_id": "T0G9PQBBK", "name": "glinda", "deleted": false, "color": "9f69e7", "real_name": "Glinda Southgood", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "8fbdd10b41c6", "image_24": "https://a.slack-edge.com...png", "image_32": "https://a.slack-edge.com...png", "image_48": "https://a.slack-edge.com...png", "image_72": "https://a.slack-edge.com...png", "image_192": "https://a.slack-edge.com...png", "image_512": "https://a.slack-edge.com...png", "image_1024": "https://a.slack-edge.com...png", "image_original": "https://a.slack-edge.com...png", "first_name": "Glinda", "last_name": "Southgood", "title": "Glinda the Good", "phone": "", "skype": "", "real_name": "Glinda Southgood", "real_name_normalized": "Glinda Southgood", "display_name": "Glinda the Fairly Good", "display_name_normalized": "Glinda the Fairly Good", "email": "glenda@south.oz.coven"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1480527098, "has_2fa": false}], "cache_ts": 1498777272, "response_metadata": {"next_cursor": "dXNlcjpVMEc5V0ZYTlo="}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_lookup_by_email() {
    let req = UsersLookupByEmailRequest::new("spengler@ghostbusters.example.com");
    {
        let (_server, client) = setup("users.lookupByEmail", r#"{"ok": true, "user": {"id": "W012A3CDE", "team_id": "T012AB3C4", "name": "spengler", "deleted": false, "color": "9f69e7", "real_name": "Egon Spengler", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "ge3b51ca72de", "status_text": "Print is dead", "status_emoji": ":books:", "real_name": "Egon Spengler", "display_name": "spengler", "real_name_normalized": "Egon Spengler", "display_name_normalized": "spengler", "email": "spengler@ghostbusters.example.com", "image_24": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_32": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_48": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_72": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_192": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_512": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "team": "T012AB3C4"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1502138686, "is_app_user": false, "has_2fa": false}}"#).await;
        let res = client
            .users_lookup_by_email(&req)
            .await
            .expect("users.lookupByEmail");
        assert_preserved(
            "users.lookupByEmail",
            r#"{"ok": true, "user": {"id": "W012A3CDE", "team_id": "T012AB3C4", "name": "spengler", "deleted": false, "color": "9f69e7", "real_name": "Egon Spengler", "tz": "America/Los_Angeles", "tz_label": "Pacific Daylight Time", "tz_offset": -25200, "profile": {"avatar_hash": "ge3b51ca72de", "status_text": "Print is dead", "status_emoji": ":books:", "real_name": "Egon Spengler", "display_name": "spengler", "real_name_normalized": "Egon Spengler", "display_name_normalized": "spengler", "email": "spengler@ghostbusters.example.com", "image_24": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_32": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_48": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_72": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_192": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "image_512": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg", "team": "T012AB3C4"}, "is_admin": true, "is_owner": false, "is_primary_owner": false, "is_restricted": false, "is_ultra_restricted": false, "is_bot": false, "updated": 1502138686, "is_app_user": false, "has_2fa": false}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_profile_get() {
    let req = UsersProfileGetRequest::new().include_labels(true).user("x");
    {
        let (_server, client) = setup("users.profile.get", r#"{"ok": true, "profile": {"title": "Head of Coffee Production", "phone": "", "skype": "", "real_name": "John Smith", "real_name_normalized": "John Smith", "display_name": "john", "display_name_normalized": "john", "fields": {"Xf0111111": {"value": "Barista", "alt": ""}, "Xf0222222": {"value": "2022-04-11", "alt": ""}, "Xf0333333": {"value": "https://example.com", "alt": ""}}, "status_text": "Watching cold brew steep", "status_emoji": ":coffee:", "status_emoji_display_info": [], "status_expiration": 0, "avatar_hash": "123xyz", "start_date": "2022-03-21", "email": "johnsmith@example.com", "pronouns": "they/them/theirs", "huddle_state": "default_unset", "huddle_state_expiration_ts": 0, "first_name": "john", "last_name": "smith", "image_24": "https://.../...-24.png", "image_32": "https://.../...-32.png", "image_48": "https://.../...-48.png", "image_72": "https://.../...-72.png", "image_192": "https://.../....-192png", "image_512": "https://.../...-512.png"}}"#).await;
        let res = client
            .users_profile_get(&req)
            .await
            .expect("users.profile.get");
        assert_preserved(
            "users.profile.get",
            r#"{"ok": true, "profile": {"title": "Head of Coffee Production", "phone": "", "skype": "", "real_name": "John Smith", "real_name_normalized": "John Smith", "display_name": "john", "display_name_normalized": "john", "fields": {"Xf0111111": {"value": "Barista", "alt": ""}, "Xf0222222": {"value": "2022-04-11", "alt": ""}, "Xf0333333": {"value": "https://example.com", "alt": ""}}, "status_text": "Watching cold brew steep", "status_emoji": ":coffee:", "status_emoji_display_info": [], "status_expiration": 0, "avatar_hash": "123xyz", "start_date": "2022-03-21", "email": "johnsmith@example.com", "pronouns": "they/them/theirs", "huddle_state": "default_unset", "huddle_state_expiration_ts": 0, "first_name": "john", "last_name": "smith", "image_24": "https://.../...-24.png", "image_32": "https://.../...-32.png", "image_48": "https://.../...-48.png", "image_72": "https://.../...-72.png", "image_192": "https://.../....-192png", "image_512": "https://.../...-512.png"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_profile_set() {
    let req = UsersProfileSetRequest::new()
        .name("first_name")
        .profile("{ first_name: \"John\", ... }")
        .user("x")
        .value("John");
    {
        let (_server, client) = setup("users.profile.set", r#"{"ok": true, "profile": {"title": "Head of Coffee Production", "phone": "", "skype": "", "real_name": "John Smith", "real_name_normalized": "John Smith", "display_name": "john", "display_name_normalized": "john", "fields": {"Xf0111111": {"value": "Barista", "alt": ""}, "Xf0222222": {"value": "2022-04-11", "alt": ""}, "Xf0333333": {"value": "https://example.com", "alt": ""}}, "status_text": "Watching cold brew steep", "status_emoji": ":coffee:", "status_emoji_display_info": [], "status_expiration": 0, "avatar_hash": "123xyz", "start_date": "2022-03-21", "email": "johnsmith@example.com", "pronouns": "they/them/theirs", "huddle_state": "default_unset", "huddle_state_expiration_ts": 0, "first_name": "john", "last_name": "smith", "image_24": "https://.../...-24.png", "image_32": "https://.../...-32.png", "image_48": "https://.../...-48.png", "image_72": "https://.../...-72.png", "image_192": "https://.../....-192png", "image_512": "https://.../...-512.png"}}"#).await;
        let res = client
            .users_profile_set(&req)
            .await
            .expect("users.profile.set");
        assert_preserved(
            "users.profile.set",
            r#"{"ok": true, "profile": {"title": "Head of Coffee Production", "phone": "", "skype": "", "real_name": "John Smith", "real_name_normalized": "John Smith", "display_name": "john", "display_name_normalized": "john", "fields": {"Xf0111111": {"value": "Barista", "alt": ""}, "Xf0222222": {"value": "2022-04-11", "alt": ""}, "Xf0333333": {"value": "https://example.com", "alt": ""}}, "status_text": "Watching cold brew steep", "status_emoji": ":coffee:", "status_emoji_display_info": [], "status_expiration": 0, "avatar_hash": "123xyz", "start_date": "2022-03-21", "email": "johnsmith@example.com", "pronouns": "they/them/theirs", "huddle_state": "default_unset", "huddle_state_expiration_ts": 0, "first_name": "john", "last_name": "smith", "image_24": "https://.../...-24.png", "image_32": "https://.../...-32.png", "image_48": "https://.../...-48.png", "image_72": "https://.../...-72.png", "image_192": "https://.../....-192png", "image_512": "https://.../...-512.png"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_set_active() {
    let req = UsersSetActiveRequest::new();
    {
        let (_server, client) = setup("users.setActive", r#"{"ok": true}"#).await;
        let res = client
            .users_set_active(&req)
            .await
            .expect("users.setActive");
        assert_preserved("users.setActive", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_set_photo() {
    let req = UsersSetPhotoRequest::new()
        .crop_w("100")
        .crop_x("10")
        .crop_y("15")
        .image("...");
    {
        let (_server, client) = setup("users.setPhoto", r#"{"ok": true}"#).await;
        let res = client.users_set_photo(&req).await.expect("users.setPhoto");
        assert_preserved("users.setPhoto", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn users_set_presence() {
    let req = UsersSetPresenceRequest::new("away");
    {
        let (_server, client) = setup("users.setPresence", r#"{"ok": true}"#).await;
        let res = client
            .users_set_presence(&req)
            .await
            .expect("users.setPresence");
        assert_preserved("users.setPresence", r#"{"ok": true}"#, &res);
    }
    {
        let (_server, client) = setup("users.setPresence", r#"{"ok": true}"#).await;
        let res = client
            .users_set_presence(&req)
            .await
            .expect("users.setPresence");
        assert_preserved("users.setPresence", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn views_open() {
    let req = ViewsOpenRequest::new(slack_web_api::blocks::View::modal(
        slack_web_api::blocks::TextObject::plain("t"),
        vec![],
    ))
    .trigger_id("12345.98765.abcd2358fdea")
    .interactivity_pointer("12345.98765.abcd2358fdea");
    {
        let (_server, client) = setup("views.open", r#"{"ok": true, "view": {"id": "VMHU10V25", "team_id": "T8N4K1JN", "type": "modal", "title": {"type": "plain_text", "text": "Quite a plain modal"}, "submit": {"type": "plain_text", "text": "Create"}, "blocks": [{"type": "input", "block_id": "a_block_id", "label": {"type": "plain_text", "text": "A simple label", "emoji": true}, "optional": false, "element": {"type": "plain_text_input", "action_id": "an_action_id"}}], "private_metadata": "Shh it is a secret", "callback_id": "identify_your_modals", "external_id": "", "state": {"values": {}}, "hash": "156772938.1827394", "clear_on_close": false, "notify_on_close": false, "root_view_id": "VMHU10V25", "app_id": "AA4928AQ", "bot_id": "BA13894H"}}"#).await;
        let res = client.views_open(&req).await.expect("views.open");
        assert_preserved(
            "views.open",
            r#"{"ok": true, "view": {"id": "VMHU10V25", "team_id": "T8N4K1JN", "type": "modal", "title": {"type": "plain_text", "text": "Quite a plain modal"}, "submit": {"type": "plain_text", "text": "Create"}, "blocks": [{"type": "input", "block_id": "a_block_id", "label": {"type": "plain_text", "text": "A simple label", "emoji": true}, "optional": false, "element": {"type": "plain_text_input", "action_id": "an_action_id"}}], "private_metadata": "Shh it is a secret", "callback_id": "identify_your_modals", "external_id": "", "state": {"values": {}}, "hash": "156772938.1827394", "clear_on_close": false, "notify_on_close": false, "root_view_id": "VMHU10V25", "app_id": "AA4928AQ", "bot_id": "BA13894H"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn views_publish() {
    let req = ViewsPublishRequest::new(
        "U0BPQUNTA",
        slack_web_api::blocks::View::modal(slack_web_api::blocks::TextObject::plain("t"), vec![]),
    )
    .hash("156772938.1827394")
    .interactivity_pointer("x");
    {
        let (_server, client) = setup("views.publish", r#"{"ok": true, "view": {"id": "VMHU10V25", "team_id": "T8N4K1JN", "type": "home", "close": null, "submit": null, "blocks": [{"type": "section", "block_id": "2WGp9", "text": {"type": "mrkdwn", "text": "A simple section with some sample sentence.", "verbatim": false}}], "private_metadata": "Shh it is a secret", "callback_id": "identify_your_home_tab", "state": {"values": {}}, "hash": "156772938.1827394", "clear_on_close": false, "notify_on_close": false, "root_view_id": "VMHU10V25", "previous_view_id": null, "app_id": "AA4928AQ", "external_id": "", "bot_id": "BA13894H"}}"#).await;
        let res = client.views_publish(&req).await.expect("views.publish");
        assert_preserved(
            "views.publish",
            r#"{"ok": true, "view": {"id": "VMHU10V25", "team_id": "T8N4K1JN", "type": "home", "close": null, "submit": null, "blocks": [{"type": "section", "block_id": "2WGp9", "text": {"type": "mrkdwn", "text": "A simple section with some sample sentence.", "verbatim": false}}], "private_metadata": "Shh it is a secret", "callback_id": "identify_your_home_tab", "state": {"values": {}}, "hash": "156772938.1827394", "clear_on_close": false, "notify_on_close": false, "root_view_id": "VMHU10V25", "previous_view_id": null, "app_id": "AA4928AQ", "external_id": "", "bot_id": "BA13894H"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn views_push() {
    let req = ViewsPushRequest::new(slack_web_api::blocks::View::modal(
        slack_web_api::blocks::TextObject::plain("t"),
        vec![],
    ))
    .trigger_id("12345.98765.abcd2358fdea")
    .interactivity_pointer("12345.98765.abcd2358fdea");
    {
        let (_server, client) = setup("views.push", r#"{"ok": true, "view": {"id": "VNM522E2U", "team_id": "T9M4RL1JM", "type": "modal", "title": {"type": "plain_text", "text": "Pushed Modal", "emoji": true}, "close": {"type": "plain_text", "text": "Back", "emoji": true}, "submit": {"type": "plain_text", "text": "Save", "emoji": true}, "blocks": [{"type": "input", "block_id": "edit_details", "element": {"type": "plain_text_input", "action_id": "detail_input"}, "label": {"type": "plain_text", "text": "Edit details"}}], "private_metadata": "", "callback_id": "view_4", "external_id": "", "state": {"values": {}}, "hash": "1569362015.55b5e41b", "clear_on_close": true, "notify_on_close": false, "root_view_id": "VNN729E3U", "previous_view_id": null, "app_id": "AAD3351BQ", "bot_id": "BADF7A34H"}}"#).await;
        let res = client.views_push(&req).await.expect("views.push");
        assert_preserved(
            "views.push",
            r#"{"ok": true, "view": {"id": "VNM522E2U", "team_id": "T9M4RL1JM", "type": "modal", "title": {"type": "plain_text", "text": "Pushed Modal", "emoji": true}, "close": {"type": "plain_text", "text": "Back", "emoji": true}, "submit": {"type": "plain_text", "text": "Save", "emoji": true}, "blocks": [{"type": "input", "block_id": "edit_details", "element": {"type": "plain_text_input", "action_id": "detail_input"}, "label": {"type": "plain_text", "text": "Edit details"}}], "private_metadata": "", "callback_id": "view_4", "external_id": "", "state": {"values": {}}, "hash": "1569362015.55b5e41b", "clear_on_close": true, "notify_on_close": false, "root_view_id": "VNN729E3U", "previous_view_id": null, "app_id": "AAD3351BQ", "bot_id": "BADF7A34H"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn views_update() {
    let req = ViewsUpdateRequest::new(slack_web_api::blocks::View::modal(
        slack_web_api::blocks::TextObject::plain("t"),
        vec![],
    ))
    .view_id("VMM512F2U")
    .external_id("bmarley_view2")
    .hash("156772938.1827394");
    {
        let (_server, client) = setup("views.update", r#"{"ok": true, "view": {"id": "VNM522E2U", "team_id": "T9M4RL1JM", "type": "modal", "title": {"type": "plain_text", "text": "Updated Modal", "emoji": true}, "close": {"type": "plain_text", "text": "Close", "emoji": true}, "submit": null, "blocks": [{"type": "section", "block_id": "s_block", "text": {"type": "plain_text", "text": "I am but an updated modal", "emoji": true}, "accessory": {"type": "button", "action_id": "button_4", "text": {"type": "plain_text", "text": "Click me"}}}], "private_metadata": "", "callback_id": "view_2", "external_id": "", "state": {"values": {}}, "hash": "1569262015.55b5e41b", "clear_on_close": true, "notify_on_close": false, "root_view_id": "VNN729E3U", "previous_view_id": null, "app_id": "AAD3351BQ", "bot_id": "BADF7A34H"}}"#).await;
        let res = client.views_update(&req).await.expect("views.update");
        assert_preserved(
            "views.update",
            r#"{"ok": true, "view": {"id": "VNM522E2U", "team_id": "T9M4RL1JM", "type": "modal", "title": {"type": "plain_text", "text": "Updated Modal", "emoji": true}, "close": {"type": "plain_text", "text": "Close", "emoji": true}, "submit": null, "blocks": [{"type": "section", "block_id": "s_block", "text": {"type": "plain_text", "text": "I am but an updated modal", "emoji": true}, "accessory": {"type": "button", "action_id": "button_4", "text": {"type": "plain_text", "text": "Click me"}}}], "private_metadata": "", "callback_id": "view_2", "external_id": "", "state": {"values": {}}, "hash": "1569262015.55b5e41b", "clear_on_close": true, "notify_on_close": false, "root_view_id": "VNN729E3U", "previous_view_id": null, "app_id": "AAD3351BQ", "bot_id": "BADF7A34H"}}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_featured_add() {
    let req = WorkflowsFeaturedAddRequest::new("x", vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("workflows.featured.add", r#"{"ok": true}"#).await;
        let res = client
            .workflows_featured_add(&req)
            .await
            .expect("workflows.featured.add");
        assert_preserved("workflows.featured.add", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_featured_list() {
    let req = WorkflowsFeaturedListRequest::new(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("workflows.featured.list", r#"{"ok": true, "featured_workflows": [{"channel_id": "C012345678", "triggers": [{"id": "Ft1234", "title": "Tabby workflow"}, {"id": "Ft5678", "title": "Tortoise workflow"}]}, {"channel_id": "C987654321", "triggers": [{"id": "Ft1234", "title": "Ragdoll workflow"}, {"id": "Ft5678", "title": "Calico workflow"}]}]}"#).await;
        let res = client
            .workflows_featured_list(&req)
            .await
            .expect("workflows.featured.list");
        assert_preserved(
            "workflows.featured.list",
            r#"{"ok": true, "featured_workflows": [{"channel_id": "C012345678", "triggers": [{"id": "Ft1234", "title": "Tabby workflow"}, {"id": "Ft5678", "title": "Tortoise workflow"}]}, {"channel_id": "C987654321", "triggers": [{"id": "Ft1234", "title": "Ragdoll workflow"}, {"id": "Ft5678", "title": "Calico workflow"}]}]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_featured_remove() {
    let req = WorkflowsFeaturedRemoveRequest::new("x", vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("workflows.featured.remove", r#"{"ok": true}"#).await;
        let res = client
            .workflows_featured_remove(&req)
            .await
            .expect("workflows.featured.remove");
        assert_preserved("workflows.featured.remove", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_featured_set() {
    let req = WorkflowsFeaturedSetRequest::new("x", vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("workflows.featured.set", r#"{"ok": true}"#).await;
        let res = client
            .workflows_featured_set(&req)
            .await
            .expect("workflows.featured.set");
        assert_preserved("workflows.featured.set", r#"{"ok": true}"#, &res);
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_triggers_permissions_add() {
    let req = WorkflowsTriggersPermissionsAddRequest::new("Ft0000000001")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .channel_ids(vec!["A1".to_string(), "A2".to_string()])
        .team_ids(vec!["A1".to_string(), "A2".to_string()])
        .org_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("workflows.triggers.permissions.add", r#"{"ok": true, "permission_type": "named_entities", "user_ids": ["U014KLZE350", "U01565LTEBD"], "channel_ids": ["C014LMDP71R"]}"#).await;
        let res = client
            .workflows_triggers_permissions_add(&req)
            .await
            .expect("workflows.triggers.permissions.add");
        assert_preserved(
            "workflows.triggers.permissions.add",
            r#"{"ok": true, "permission_type": "named_entities", "user_ids": ["U014KLZE350", "U01565LTEBD"], "channel_ids": ["C014LMDP71R"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_triggers_permissions_list() {
    let req = WorkflowsTriggersPermissionsListRequest::new("Ft0000000001");
    {
        let (_server, client) = setup(
            "workflows.triggers.permissions.list",
            r#"{"ok": true, "permission_type": "app_collaborators", "user_ids": ["U01565LTEBD"]}"#,
        )
        .await;
        let res = client
            .workflows_triggers_permissions_list(&req)
            .await
            .expect("workflows.triggers.permissions.list");
        assert_preserved(
            "workflows.triggers.permissions.list",
            r#"{"ok": true, "permission_type": "app_collaborators", "user_ids": ["U01565LTEBD"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_triggers_permissions_remove() {
    let req = WorkflowsTriggersPermissionsRemoveRequest::new("Ft0000000001")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .channel_ids(vec!["A1".to_string(), "A2".to_string()])
        .team_ids(vec!["A1".to_string(), "A2".to_string()])
        .org_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup("workflows.triggers.permissions.remove", r#"{"ok": true, "permission_type": "named_entities", "user_ids": ["U014KLZE350", "U01565LTEBD"], "channel_ids": ["C014LMDP71R"]}"#).await;
        let res = client
            .workflows_triggers_permissions_remove(&req)
            .await
            .expect("workflows.triggers.permissions.remove");
        assert_preserved(
            "workflows.triggers.permissions.remove",
            r#"{"ok": true, "permission_type": "named_entities", "user_ids": ["U014KLZE350", "U01565LTEBD"], "channel_ids": ["C014LMDP71R"]}"#,
            &res,
        );
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn workflows_triggers_permissions_set() {
    let req = WorkflowsTriggersPermissionsSetRequest::new("Ft0000000001", "x")
        .user_ids(vec!["A1".to_string(), "A2".to_string()])
        .channel_ids(vec!["A1".to_string(), "A2".to_string()])
        .team_ids(vec!["A1".to_string(), "A2".to_string()])
        .org_ids(vec!["A1".to_string(), "A2".to_string()]);
    {
        let (_server, client) = setup(
            "workflows.triggers.permissions.set",
            r#"{"ok": true, "permission_type": "named_entities", "user_ids": ["U01565LTEBD"]}"#,
        )
        .await;
        let res = client
            .workflows_triggers_permissions_set(&req)
            .await
            .expect("workflows.triggers.permissions.set");
        assert_preserved(
            "workflows.triggers.permissions.set",
            r#"{"ok": true, "permission_type": "named_entities", "user_ids": ["U01565LTEBD"]}"#,
            &res,
        );
    }
}
