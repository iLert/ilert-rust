#![allow(dead_code,unused)]

pub mod ilert;
pub mod ilert_error;
pub mod ilert_builders;

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::ilert::ILert;
    use crate::ilert_builders::{
        UserApiResource,
        EventApiResource,
        ScheduleApiResource, 
        HeartbeatApiResource,
        ILertEventType,
        ILertPriority,
        EventImage
    };

    #[test]
    fn init() -> () {
        env_logger::init();
    }

    #[test]
    fn user_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();
        client.auth_via_user("chris@chris", "chris").unwrap();

        let user_result = client
            .get()
            .users()
            .execute()
            .unwrap();

        assert_eq!(user_result.status, 200);
    }

    #[test]
    fn schedule_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();
        client.auth_via_user("chris@chris", "chris").unwrap();

        let schedule_result = client
            .get()
            .schedule_shifts(99)
            .execute()
            .unwrap();

        assert_eq!(schedule_result.status, 404);
    }

    #[test]
    fn create_and_resolve_event_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();

        let event_result = client
            .post()
            .event_with_details(
                "il1api0220953b09684c9e4fe8972f0d5d8c9cde78d79b6cc8fd",
                ILertEventType::ALERT, "Host srv/mail01 is CRITICAL",
                Some("bratwurst".to_string()),
                Some("some detail message".to_string()),
                Some(ILertPriority::LOW),
                Some(vec![EventImage::new("https://i.giphy.com/media/VRhsYYBw8AE36/giphy.webp")]),
                Some(vec![]),
                Some(json!({"hehe": "test"}))
            )
            .execute()
            .unwrap();

        assert_eq!(event_result.status, 200);

        let resolve_result = client
            .post()
            .event("il1api0220953b09684c9e4fe8972f0d5d8c9cde78d79b6cc8fd",
                   ILertEventType::RESOLVE, "Host srv/mail01 is CRITICAL",
                    Some("bratwurst".to_string()))
            .execute()
            .unwrap();

        assert_eq!(resolve_result.status, 200);
    }

    #[test]
    fn heartbeat_test() {

        let mut client = ILert::new_with_opts(Some("http://localhost:8080"), Some(10)).unwrap();

        let heartbeat_result = client
            .get()
            .heartbeat("43c7afdc-0b3e-4344-b48a-5379a963241f")
            .execute()
            .unwrap();

        assert_eq!(heartbeat_result.status, 202);
    }
}
