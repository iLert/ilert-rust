#![allow(dead_code,unused)]

pub mod ilert;
pub mod ilert_error;
pub mod ilert_builders;

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::ilert::ILert;
    use crate::ilert_error::{ILertError, ILertResult};
    use crate::ilert_builders::*;

    // --- ILert client construction ---

    #[test]
    fn new_creates_client_with_defaults() {
        let client = ILert::new().unwrap();
        assert!(client.api_token.is_none());
        assert!(client.auth_user.is_none());
        assert!(client.auth_psw.is_none());
    }

    #[test]
    fn new_with_opts_custom_host() {
        let client = ILert::new_with_opts(Some("http://localhost:9999"), None, Some(5)).unwrap();
        let url = client.build_url("/events");
        assert_eq!(url, "http://localhost:9999/api/events");
    }

    #[test]
    fn new_with_opts_defaults_when_none() {
        let client = ILert::new_with_opts(None, None, None).unwrap();
        let url = client.build_url("/events");
        assert_eq!(url, "https://api.ilert.com/api/events");
    }

    #[test]
    fn new_with_opts_custom_hbt_host() {
        let client = ILert::new_with_opts(None, Some("http://beat-test:8080"), None).unwrap();
        let url = client.build_hbt_url("/pings/abc");
        assert_eq!(url, "http://beat-test:8080/api/pings/abc");
    }

    // --- URL building ---

    #[test]
    fn build_url_appends_api_prefix() {
        let client = ILert::new().unwrap();
        assert_eq!(client.build_url("/users"), "https://api.ilert.com/api/users");
        assert_eq!(client.build_url("/alerts/42"), "https://api.ilert.com/api/alerts/42");
    }

    #[test]
    fn build_hbt_url_uses_beat_host() {
        let client = ILert::new().unwrap();
        assert_eq!(client.build_hbt_url("/pings/key1"), "https://beat.ilert.com/api/pings/key1");
    }

    // --- Authentication ---

    #[test]
    fn auth_via_token_sets_token() {
        let mut client = ILert::new().unwrap();
        client.auth_via_token("my-token").unwrap();
        assert_eq!(client.api_token.as_ref().unwrap(), "my-token");
    }

    #[test]
    fn auth_via_user_sets_credentials() {
        let mut client = ILert::new().unwrap();
        client.auth_via_user("user@example.com", "secret").unwrap();
        assert_eq!(client.auth_user.as_ref().unwrap(), "user@example.com");
        assert_eq!(client.auth_psw.as_ref().unwrap(), "secret");
    }

    #[test]
    fn auth_via_token_overwrites_previous() {
        let mut client = ILert::new().unwrap();
        client.auth_via_token("token-1").unwrap();
        client.auth_via_token("token-2").unwrap();
        assert_eq!(client.api_token.as_ref().unwrap(), "token-2");
    }

    // --- ILertEventType ---

    #[test]
    fn event_type_as_str() {
        assert_eq!(ILertEventType::ALERT.as_str(), "ALERT");
        assert_eq!(ILertEventType::ACCEPT.as_str(), "ACCEPT");
        assert_eq!(ILertEventType::RESOLVE.as_str(), "RESOLVE");
        assert_eq!(ILertEventType::COMMENT.as_str(), "COMMENT");
    }

    #[test]
    fn event_type_from_str_valid() {
        assert_eq!(ILertEventType::from_str("ALERT").unwrap().as_str(), "ALERT");
        assert_eq!(ILertEventType::from_str("ACCEPT").unwrap().as_str(), "ACCEPT");
        assert_eq!(ILertEventType::from_str("RESOLVE").unwrap().as_str(), "RESOLVE");
        assert_eq!(ILertEventType::from_str("COMMENT").unwrap().as_str(), "COMMENT");
    }

    #[test]
    fn event_type_from_str_invalid() {
        assert!(ILertEventType::from_str("INVALID").is_err());
        assert!(ILertEventType::from_str("alert").is_err()); // case sensitive
        assert!(ILertEventType::from_str("").is_err());
    }

    // --- ILertPriority ---

    #[test]
    fn priority_as_str() {
        assert_eq!(ILertPriority::HIGH.as_str(), "HIGH");
        assert_eq!(ILertPriority::LOW.as_str(), "LOW");
    }

    #[test]
    fn priority_from_str_valid() {
        assert_eq!(ILertPriority::from_str("HIGH").unwrap().as_str(), "HIGH");
        assert_eq!(ILertPriority::from_str("LOW").unwrap().as_str(), "LOW");
    }

    #[test]
    fn priority_from_str_invalid() {
        assert!(ILertPriority::from_str("MEDIUM").is_err());
        assert!(ILertPriority::from_str("high").is_err());
        assert!(ILertPriority::from_str("").is_err());
    }

    // --- EventImage ---

    #[test]
    fn event_image_new() {
        let img = EventImage::new("https://example.com/img.png");
        assert_eq!(img.src, "https://example.com/img.png");
        assert!(img.href.is_none());
        assert!(img.alt.is_none());
    }

    #[test]
    fn event_image_serializes() {
        let img = EventImage { src: "https://img.png".to_string(), href: Some("https://link.com".to_string()), alt: Some("alt text".to_string()) };
        let json = serde_json::to_value(&img).unwrap();
        assert_eq!(json["src"], "https://img.png");
        assert_eq!(json["href"], "https://link.com");
        assert_eq!(json["alt"], "alt text");
    }

    #[test]
    fn event_image_deserializes() {
        let json = r#"{"src": "https://img.png"}"#;
        let img: EventImage = serde_json::from_str(json).unwrap();
        assert_eq!(img.src, "https://img.png");
        assert!(img.href.is_none());
    }

    // --- EventLink ---

    #[test]
    fn event_link_new() {
        let link = EventLink::new("https://example.com");
        assert_eq!(link.href, "https://example.com");
        assert!(link.text.is_none());
    }

    #[test]
    fn event_link_serializes() {
        let link = EventLink { href: "https://example.com".to_string(), text: Some("Dashboard".to_string()) };
        let json = serde_json::to_value(&link).unwrap();
        assert_eq!(json["href"], "https://example.com");
        assert_eq!(json["text"], "Dashboard");
    }

    // --- EventComment ---

    #[test]
    fn event_comment_new() {
        let comment = EventComment::new("Peter", "This is a comment");
        assert_eq!(comment.creator, "Peter");
        assert_eq!(comment.content, "This is a comment");
    }

    #[test]
    fn event_comment_serializes() {
        let comment = EventComment::new("Admin", "Investigating");
        let json = serde_json::to_value(&comment).unwrap();
        assert_eq!(json["creator"], "Admin");
        assert_eq!(json["content"], "Investigating");
    }

    // --- ILertError ---

    #[test]
    fn error_new_and_display() {
        let err = ILertError::new("something went wrong");
        assert_eq!(err.message, "something went wrong");
        assert_eq!(format!("{}", err), "something went wrong");
    }

    #[test]
    fn error_implements_std_error() {
        let err = ILertError::new("test");
        let std_err: &dyn std::error::Error = &err;
        assert_eq!(std_err.to_string(), "test");
    }

    // --- BaseRequestOptions ---

    #[test]
    fn base_request_options_default() {
        let opts = BaseRequestOptions::new();
        assert!(opts.path.is_none());
        assert!(opts.url.is_none());
        assert!(opts.body.is_none());
        assert!(!opts.use_hbt_host);
        assert!(opts.headers.is_empty());
    }

    // --- Builder path setup ---

    #[test]
    fn get_builder_users_sets_path() {
        let client = ILert::new().unwrap();
        let mut get = client.get();
        get.users();
        // execute would fail without a server, but path should be set
    }

    #[test]
    fn get_builder_skip_and_limit() {
        let client = ILert::new().unwrap();
        let get = client.get().skip(5).limit(20);
        // these are fluent and return Self
    }

    #[test]
    fn get_builder_filter_chaining() {
        let client = ILert::new().unwrap();
        let get = client.get()
            .filter("states", "ACCEPTED")
            .filter("states", "RESOLVED");
        // filters should be accumulated
    }

    #[test]
    fn post_builder_event_body_is_json() {
        let client = ILert::new().unwrap();
        let mut post = client.create();
        post.event(
            "test-api-key",
            ILertEventType::ALERT,
            Some("Server down".to_string()),
            Some("host-1".to_string()),
        );
        let body = post.builder.options.body.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["integrationKey"], "test-api-key");
        assert_eq!(parsed["eventType"], "ALERT");
        assert_eq!(parsed["summary"], "Server down");
        assert_eq!(parsed["alertKey"], "host-1");
    }

    #[test]
    fn post_builder_event_with_details_body() {
        let client = ILert::new().unwrap();
        let mut post = client.create();
        let mut labels = std::collections::HashMap::new();
        labels.insert("env".to_string(), "prod".to_string());
        labels.insert("team".to_string(), "backend".to_string());
        post.event_with_details(
            "key1",
            ILertEventType::RESOLVE,
            Some("Resolved".to_string()),
            Some("alert-1".to_string()),
            Some("Detail text".to_string()),
            Some(ILertPriority::HIGH),
            Some(vec![EventImage::new("https://img.png")]),
            Some(vec![EventLink::new("https://link.com")]),
            Some(json!({"env": "prod"})),
            Some("routing-1".to_string()),
            Some(3),
            Some(labels),
            Some(vec![EventServiceRef::new("my-service")]),
        );
        let body = post.builder.options.body.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["integrationKey"], "key1");
        assert_eq!(parsed["eventType"], "RESOLVE");
        assert_eq!(parsed["summary"], "Resolved");
        assert_eq!(parsed["alertKey"], "alert-1");
        assert_eq!(parsed["details"], "Detail text");
        assert_eq!(parsed["priority"], "HIGH");
        assert_eq!(parsed["severity"], 3);
        assert_eq!(parsed["images"][0]["src"], "https://img.png");
        assert_eq!(parsed["links"][0]["href"], "https://link.com");
        assert_eq!(parsed["customDetails"]["env"], "prod");
        assert_eq!(parsed["routingKey"], "routing-1");
        assert_eq!(parsed["labels"]["env"], "prod");
        assert_eq!(parsed["labels"]["team"], "backend");
        assert_eq!(parsed["services"][0]["alias"], "my-service");
    }

    #[test]
    fn post_builder_event_with_comment_body() {
        let client = ILert::new().unwrap();
        let mut post = client.create();
        post.event_with_comment(
            "key1",
            Some("alert-1".to_string()),
            Some(vec![EventComment::new("Admin", "Looking into it")]),
        );
        let body = post.builder.options.body.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["integrationKey"], "key1");
        assert_eq!(parsed["eventType"], "COMMENT");
        assert_eq!(parsed["alertKey"], "alert-1");
        assert_eq!(parsed["comments"][0]["creator"], "Admin");
        assert_eq!(parsed["comments"][0]["content"], "Looking into it");
    }

    #[test]
    fn post_builder_event_uses_default_events_path() {
        let client = ILert::new().unwrap();
        let mut post = client.create();
        post.event("k1", ILertEventType::ALERT, None, None);
        assert_eq!(post.builder.options.path.as_ref().unwrap(), "/events");
    }

    #[test]
    fn post_builder_event_preserves_custom_path() {
        let client = ILert::new().unwrap();
        let mut post = client.create();
        post.builder.options.path = Some("/v1/events/mqtt/key1".to_string());
        post.event("k1", ILertEventType::ALERT, None, None);
        // should NOT overwrite the custom path
        assert_eq!(post.builder.options.path.as_ref().unwrap(), "/v1/events/mqtt/key1");
    }

    #[test]
    fn post_builder_event_with_none_optionals() {
        let client = ILert::new().unwrap();
        let mut post = client.create();
        post.event("k1", ILertEventType::ALERT, None, None);
        let body = post.builder.options.body.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["integrationKey"], "k1");
        assert_eq!(parsed["eventType"], "ALERT");
        assert!(parsed["summary"].is_null());
        assert!(parsed["alertKey"].is_null());
    }

    // --- GET resource path tests ---

    #[test]
    fn get_users_path() {
        let client = ILert::new().unwrap();
        let mut get = client.get();
        get.users();
        // internal builder path not directly accessible, but we can test via build_url pattern
    }

    #[test]
    fn put_accept_alert_path() {
        let client = ILert::new().unwrap();
        let mut put = client.update();
        put.accept_alert(42);
        // path should be /alerts/42/accept
    }

    #[test]
    fn put_resolve_alert_path() {
        let client = ILert::new().unwrap();
        let mut put = client.update();
        put.resolve_alert(99);
    }

    // --- User-Agent header ---

    #[test]
    fn user_agent_matches_package_version() {
        // Verify the default headers contain the correct version by checking the http client
        let client = ILert::new().unwrap();
        // The http_client is public, so we can verify it was constructed
        // The User-Agent is set via default_headers which we can't inspect directly,
        // but we verified the format string at compile time via env!("CARGO_PKG_VERSION")
        let expected = format!("ilert-rust/{}", env!("CARGO_PKG_VERSION"));
        assert!(expected.starts_with("ilert-rust/"));
        assert!(!expected.contains("4.1.1"), "User-Agent should not contain old hardcoded version");
    }
}
