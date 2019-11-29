#![allow(dead_code,unused)]

pub mod ilert;
pub mod ilert_error;
pub mod ilert_builders;

#[cfg(test)]
mod tests {

    use crate::ilert::ILert;
    use crate::ilert_builders::{UserApiResource, EventApiResource};
    use crate::ilert_builders::ILertEventType::ALERT;

    #[test]
    fn simple_integration_test() {

        let mut client = ILert::new(Some("http://localhost:8080"), Some(10)).unwrap();
        client.auth_via_user("chris@chris", "chris").unwrap();

        let user_result = client
            .get()
            .users()
            .execute()
            .unwrap();

        assert_eq!(user_result.status, 200);

        let event_result = client
            .post()
            .events("44c7afdc-0b3e-4344-b48a-5379a963231f",
            ALERT, "Host srv/mail01 is CRITICAL",
                    None,
                    Some("srv/mail01".to_string()))
            .execute()
            .unwrap();

        assert_eq!(event_result.status, 200);
    }
}
