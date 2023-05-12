# ilert-rust &emsp;  [![Latest Version]][crates.io] [![ilert: rustc 1.13+]][Rust 1.13]

[Latest Version]: https://img.shields.io/crates/v/ilert.svg
[crates.io]: https://crates.io/crates/ilert
[ilert: rustc 1.13+]: https://img.shields.io/badge/ilert-rustc_1.13+-lightgray.svg
[Rust 1.13]: https://blog.rust-lang.org/2016/11/10/Rust-1.13.html
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

**The official ilert API bindings.**

## In action

```rust
use ilert::ilert::ILert;
use ilert::ilert_builders::{UserGetApiResource, EventApiResource, ILertEventType};

let mut client = ILert::new().unwrap();
client.auth_via_token("your-api-token").unwrap();

// create a new alert via event

client
    .create()
    .event(
        "44c7afdc-0b3e-4344-b48a-5378a963231f",
        ILertEventType::ALERT,
        "Host srv/mail01 is CRITICAL", None)
    .execute();

// accept alert

let accept_result = client
    .update()
    .accept_alert(123)
    .execute()
    .unwrap();

// resolve alert

let resolve_result = client
    .update()
    .resolve_alert(123)
    .execute()
    .unwrap();

// fetch users

let user_result = client
    .get()
    .skip(5)
    .limit(10)
    .filter("role", "RESPONDER")
    .filter("role", "USER")
    .users()
    .execute()
    .unwrap();

// ping a heartbeat

client
    .get()
    .heartbeat("43c7afdc-0b3e-4344-b48a-5379a963241f")
    .execute();

// create detailed alert via event

client
.create()
.event_with_details(
    "8972f0d5d8c9cde78d79b6cc8fd",
    ILertEventType::ALERT,
    Some("Host srv/mail01 is CRITICAL".to_string()),
    Some("bratwurst".to_string()),
    Some("some detail message".to_string()),
    Some(ILertPriority::LOW),
    Some(vec![EventImage::new("https://i.giphy.com/media/VRhsYYBw8AE36/giphy.webp")]),
    Some(vec![]),
    Some(json!({"hehe": "test"})),
    None)
.execute()
.unwrap();

// add comment to alert via event

client
    .create()
    .event_with_comment(
        "8972f0d5d8c9cde78d79b6cc8fd",
        Some("bratwurst".to_string()),
        Some(vec![EventComment::new("Peter Parker", "a comment ![alt text picture](https://i.giphy.com/media/VRhsYYBw8AE36/giphy.webp)")]))
    .execute()
    .unwrap();

// resolve alert via event

client
    .create()
    .event("8972f0d5d8c9cde78d79b6cc8fd",
        ILertEventType::RESOLVE, None,
        Some("bratwurst".to_string()))
    .execute()
    .unwrap();
```

## Getting help

We are happy to respond to [GitHub issues][issues] as well.

[issues]: https://github.com/iLert/ilert-rust/issues/new

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in ilert-rust by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>

