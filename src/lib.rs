#![allow(dead_code,unused)]

pub mod ilert;
pub mod ilert_error;
pub mod ilert_builders;

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::ilert::ILert;
    use crate::ilert_builders::{UserGetApiResource, EventApiResource, ScheduleGetApiResource, HeartbeatApiResource, ILertEventType, ILertPriority, EventImage, EventComment, AlertGetApiResource, AlertPutApiResource};

    #[tokio::test]
    async fn init() -> () {
        env_logger::init();
    }

    #[tokio::test]
    async fn user_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();
        client.auth_via_user("chris@chris", "chris").unwrap();

        let user_result = client
            .get()
            .skip(0)
            .limit(10)
            .users()
            .execute()
            .await
            .unwrap();

        assert_eq!(user_result.status, 200);
    }

    #[tokio::test]
    async fn alert_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();
        client.auth_via_user("chris@chris", "chris").unwrap();

        let alert_result = client
            .get()
            .skip(0)
            .limit(10)
            .filter("states", "ACCEPTED")
            .filter("states", "RESOLVED")
            .alerts()
            .execute()
            .await
            .unwrap();

        assert_eq!(alert_result.status, 200);
    }

    #[tokio::test]
    async fn schedule_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();
        client.auth_via_user("chris@chris", "chris").unwrap();

        let schedule_result = client
            .get()
            .schedule_shifts(99)
            .execute()
            .await
            .unwrap();

        assert_eq!(schedule_result.status, 404);
    }

    #[tokio::test]
    async fn create_comment_and_resolve_event_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();

        let event_result = client
            .create()
            .event_with_details(
                "il1api0220953b09684c9e4fe8972f0d5d8c9cde78d79b6cc8fd",
                ILertEventType::ALERT,
                Some("Host srv/mail01 is CRITICAL".to_string()),
                Some("bratwurst".to_string()),
                Some("some detail message".to_string()),
                Some(ILertPriority::LOW),
                Some(vec![EventImage::new("https://i.giphy.com/media/VRhsYYBw8AE36/giphy.webp")]),
                Some(vec![]),
                Some(json!({"hehe": "test"})),
                None
            )
            .execute()
            .await
            .unwrap();

        assert_eq!(event_result.status, 202);

        let event_comment_result = client
            .create()
            .event_with_comment(
                "il1api0220953b09684c9e4fe8972f0d5d8c9cde78d79b6cc8fd",
                Some("bratwurst".to_string()),
                Some(vec![EventComment::new("Peter Parker",
                                            "a comment ![alt text picture](https://i.giphy.com/media/VRhsYYBw8AE36/giphy.webp) salut")])
            )
            .execute()
            .await
            .unwrap();

        assert_eq!(event_comment_result.status, 202);

        let resolve_result = client
            .create()
            .event("il1api0220953b09684c9e4fe8972f0d5d8c9cde78d79b6cc8fd",
                   ILertEventType::RESOLVE, None,
                    Some("bratwurst".to_string()))
            .execute()
            .await
            .unwrap();

        assert_eq!(resolve_result.status, 202);
    }

    #[tokio::test]
    async fn heartbeat_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();

        let heartbeat_result = client
            .get()
            .heartbeat("43c7afdc-0b3e-4344-b48a-5379a963241f")
            .execute()
            .await
            .unwrap();

        assert_eq!(heartbeat_result.status, 202);
    }
}
