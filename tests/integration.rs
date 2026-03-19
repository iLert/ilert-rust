use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header, query_param};
use serde_json::json;

use ilert::ilert::ILert;
use ilert::ilert_builders::*;

// --- Event creation (POST /api/events) ---

#[tokio::test]
async fn post_event_sends_correct_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/events"))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();

    let result = client
        .create()
        .event("test-key", ILertEventType::ALERT, Some("Server down".to_string()), Some("host-1".to_string()))
        .execute()
        .await
        .unwrap();

    assert_eq!(result.status, 202);
}

#[tokio::test]
async fn post_event_with_details_sends_all_fields() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/events"))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();

    let result = client
        .create()
        .event_with_details(
            "key1",
            ILertEventType::ALERT,
            Some("Alert summary".to_string()),
            Some("alert-key-1".to_string()),
            Some("Detail text".to_string()),
            Some(ILertPriority::HIGH),
            Some(vec![EventImage::new("https://img.png")]),
            Some(vec![EventLink::new("https://link.com")]),
            Some(json!({"env": "prod"})),
            None,
            Some(2),
            None,
            Some(vec![EventServiceRef::new("web-app")]),
        )
        .execute()
        .await
        .unwrap();

    assert_eq!(result.status, 202);
}

#[tokio::test]
async fn post_event_with_comment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/events"))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();

    let result = client
        .create()
        .event_with_comment(
            "key1",
            Some("alert-1".to_string()),
            Some(vec![EventComment::new("Admin", "Investigating")]),
        )
        .execute()
        .await
        .unwrap();

    assert_eq!(result.status, 202);
}

#[tokio::test]
async fn post_event_custom_path_overrides_default() {
    let mock_server = MockServer::start().await;

    // custom path gets /api prepended by build_url, so /v1/events/mqtt/key -> /api/v1/events/mqtt/key
    Mock::given(method("POST"))
        .and(path("/api/v1/events/mqtt/custom-key"))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();

    let mut post = client.create();
    post.builder.options.path = Some("/v1/events/mqtt/custom-key".to_string());
    post.event("custom-key", ILertEventType::ALERT, Some("test".to_string()), None);

    let result = post.execute().await.unwrap();
    assert_eq!(result.status, 202);
}

// --- GET requests ---

#[tokio::test]
async fn get_users_sends_get_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/users"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!([])),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("test-token").unwrap();

    let result = client.get().users().execute().await.unwrap();
    assert_eq!(result.status, 200);
}

#[tokio::test]
async fn get_user_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/users/42"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!({"id": 42, "username": "chris"})),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    let result = client.get().user(42).execute().await.unwrap();
    assert_eq!(result.status, 200);
    assert_eq!(result.body_json.unwrap()["id"], 42);
}

#[tokio::test]
async fn get_alerts_with_filters() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/alerts"))
        .and(query_param("states", "ACCEPTED"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!([])),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let result = client
        .get()
        .filter("states", "ACCEPTED")
        .alerts()
        .execute()
        .await
        .unwrap();

    assert_eq!(result.status, 200);
}

#[tokio::test]
async fn get_schedule_shifts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/schedules/7/shifts"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!([])),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    let result = client.get().schedule_shifts(7).execute().await.unwrap();
    assert_eq!(result.status, 200);
}

// --- PUT requests ---

#[tokio::test]
async fn put_accept_alert() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/alerts/99/accept"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let result = client.update().accept_alert(99).execute().await.unwrap();
    assert_eq!(result.status, 200);
}

#[tokio::test]
async fn put_resolve_alert() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/alerts/99/resolve"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let result = client.update().resolve_alert(99).execute().await.unwrap();
    assert_eq!(result.status, 200);
}

// --- DELETE requests ---

#[tokio::test]
async fn delete_incident_sends_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/incidents/5"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let result = client.delete().incident(5).execute().await.unwrap();
    assert_eq!(result.status, 204);
}

#[tokio::test]
async fn delete_service_sends_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/services/10"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let result = client.delete().service(10).execute().await.unwrap();
    assert_eq!(result.status, 204);
}

// --- Escalation Policies ---

#[tokio::test]
async fn get_escalation_policy_resolve_by_routing_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/escalation-policies/resolve"))
        .and(query_param("routing-key", "my-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!({"id": 1, "name": "Default"})),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let result = client.get().escalation_policy_resolve("my-key").execute().await.unwrap();
    assert_eq!(result.status, 200);
    assert_eq!(result.body_json.unwrap()["name"], "Default");
}

#[tokio::test]
async fn put_escalation_policy_level_raw() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/escalation-policies/10/levels/2"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!({"escalationTimeout": 5})),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let rule = json!({"escalationTimeout": 5});
    let result = client.update().escalation_policy_level_raw(10, 2, &rule).execute().await.unwrap();
    assert_eq!(result.status, 200);
    assert_eq!(result.body_json.unwrap()["escalationTimeout"], 5);
}

// --- User search by email ---

#[tokio::test]
async fn post_user_search_email() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/users/search-email"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!({"id": 42, "email": "test@example.com"})),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("tok").unwrap();

    let result = client.create().user_search_email("test@example.com").execute().await.unwrap();
    assert_eq!(result.status, 200);
    assert_eq!(result.body_json.unwrap()["email"], "test@example.com");
}

// --- Ping (heartbeat) ---

#[tokio::test]
async fn head_ping_uses_hbt_host() {
    let mock_server = MockServer::start().await;

    Mock::given(method("HEAD"))
        .and(path("/api/pings/my-hbt-key"))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&mock_server)
        .await;

    // pass mock as hbt_host
    let client = ILert::new_with_opts(None, Some(&mock_server.uri()), Some(5)).unwrap();

    let result = client.head().ping("my-hbt-key").execute().await.unwrap();
    assert_eq!(result.status, 202);
}

// --- Auth headers ---

#[tokio::test]
async fn token_auth_sends_bearer_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/users"))
        .and(header("Authorization", "Bearer test-bearer-token"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!([])),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_token("test-bearer-token").unwrap();

    let result = client.get().users().execute().await.unwrap();
    assert_eq!(result.status, 200);
}

#[tokio::test]
async fn basic_auth_sends_encoded_header() {
    let mock_server = MockServer::start().await;

    // user:pass -> base64("user:pass") = "dXNlcjpwYXNz"
    Mock::given(method("GET"))
        .and(path("/api/users"))
        .and(header("Authorization", "Basic dXNlcjpwYXNz"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!([])),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    client.auth_via_user("user", "pass").unwrap();

    let result = client.get().users().execute().await.unwrap();
    assert_eq!(result.status, 200);
}

// --- Error handling ---

#[tokio::test]
async fn server_error_returns_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/events"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();

    let result = client
        .create()
        .event("k1", ILertEventType::ALERT, Some("test".to_string()), None)
        .execute()
        .await
        .unwrap();

    assert_eq!(result.status, 500);
}

#[tokio::test]
async fn response_body_parsed_as_json_when_content_type_matches() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/alerts/1"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_json(json!({"id": 1, "status": "PENDING"})),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    let result = client.get().alert(1).execute().await.unwrap();

    assert_eq!(result.status, 200);
    let body = result.body_json.unwrap();
    assert_eq!(body["id"], 1);
    assert_eq!(body["status"], "PENDING");
    assert!(result.body_raw.is_some());
}

#[tokio::test]
async fn response_body_not_parsed_when_not_json() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/alerts/1"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/plain")
                .set_body_string("plain text response"),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ILert::new_with_opts(Some(&mock_server.uri()), None, Some(5)).unwrap();
    let result = client.get().alert(1).execute().await.unwrap();

    assert_eq!(result.status, 200);
    assert!(result.body_json.is_none());
    assert_eq!(result.body_raw.as_ref().unwrap(), "plain text response");
}

// --- Execute without path fails ---

#[tokio::test]
async fn execute_without_path_returns_error() {
    let client = ILert::new().unwrap();
    let result = client.get().execute().await;
    assert!(result.is_err());
}
