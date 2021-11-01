use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use base64;
use reqwest::StatusCode;
use serde_json::{Result, Value};
use serde_json::json;
use serde_derive::{Deserialize, Serialize};

use crate::ilert::ILert;
use crate::ilert_error::{ILertResult, ILertError};
use std::error::Error;

pub enum ILertEventType {
    ALERT,
    ACCEPT,
    RESOLVE,
    COMMENT
}

pub enum ILertPriority {
    HIGH,
    LOW,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventImage {
    pub src: String,
    pub href: Option<String>,
    pub alt: Option<String>
}

impl EventImage {
    pub fn new(src: &str) -> EventImage {
        EventImage {
            src: src.to_string(),
            href: None,
            alt: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventLink {
    pub href: String,
    pub text: Option<String>
}

impl EventLink {
    pub fn new(src: &str) -> EventLink {
        EventLink {
            href: src.to_string(),
            text: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventComment {
    pub creator: String,
    pub content: String
}

impl EventComment {
    pub fn new(creator: &str, content: &str) -> EventComment {
        EventComment {
            creator: creator.to_string(),
            content: content.to_string()
        }
    }
}

impl ILertEventType {

    pub fn as_str(&self) -> &str {
        match self {
            &ILertEventType::ALERT => "ALERT",
            &ILertEventType::ACCEPT => "ACCEPT",
            &ILertEventType::RESOLVE => "RESOLVE",
            &ILertEventType::COMMENT => "COMMENT",
        }
    }

    pub fn from_str(val: &str) -> ILertResult<ILertEventType> {
        match val {
            "ALERT" => Ok(ILertEventType::ALERT),
            "ACCEPT" => Ok(ILertEventType::ACCEPT),
            "RESOLVE" => Ok(ILertEventType::RESOLVE),
            "COMMENT" => Ok(ILertEventType::COMMENT),
            _ => Err(ILertError::new("Unsupported type value.")),
        }
    }
}

impl ILertPriority {

    pub fn as_str(&self) -> &str {
        match self {
            &ILertPriority::HIGH => "HIGH",
            &ILertPriority::LOW => "LOW",
        }
    }

    pub fn from_str(val: &str) -> ILertResult<ILertPriority> {
        match val {
            "HIGH" => Ok(ILertPriority::HIGH),
            "LOW" => Ok(ILertPriority::LOW),
            _ => Err(ILertError::new("Unsupported type value.")),
        }
    }
}

#[derive(Debug, Clone)]
struct BaseRequestOptions {
    path: Option<String>,
    url: Option<String>,
    headers: HeaderMap,
    body: Option<String>,
}

impl BaseRequestOptions {
    pub fn new() -> BaseRequestOptions {
        BaseRequestOptions {
            path: None,
            url: None,
            headers: HeaderMap::new(),
            body: None
        }
    }
}

#[derive(Debug, Clone)]
struct BaseRequestBuilder<'a> {
    _ilert: &'a ILert,
    options: BaseRequestOptions,
}

impl<'a> BaseRequestBuilder<'a> {

    fn new(_ilert: &'a ILert) -> BaseRequestBuilder<'a> {
        BaseRequestBuilder {
            _ilert,
            options: BaseRequestOptions::new(),
        }
    }

    fn set_path(&mut self, path: &str) -> () {
        self.options.path = Some(path.to_string());
    }

    fn set_body(&mut self, body: &str) -> () {
        self.options.body = Some(body.to_string());
    }
}

#[derive(Debug)]
pub struct BaseRequestResult {
    pub url: String,
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body_raw: Option<String>,
    pub body_json: Option<Value>,
}

impl BaseRequestResult {

     fn new(url: String, status: StatusCode, headers: HeaderMap, body_raw: Option<String>, body_json: Option<Value>) -> BaseRequestResult {
        BaseRequestResult {
            url,
            status,
            headers,
            body_raw,
            body_json,
        }
    }
}

pub trait BaseRequestExecutor {
    fn execute(&self) -> ILertResult<BaseRequestResult>;
}

fn prepare_generic_request_builder (builder: &BaseRequestBuilder) -> ILertResult<BaseRequestOptions> {

    let ilertref = builder._ilert;
    let mut options = builder.options.clone();

    if builder.options.path.is_none() {
        return Err(ILertError::new("Failed to build url, path missing."));
    }

    let url = ilertref.build_url(builder.options.path.as_ref().unwrap().as_str());
    options.url = Some(url);

    match ilertref.api_token.clone() {
        Some(token) => options.headers
            .append("Authorization", HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap()),
        None => false,
    };

    match ilertref.auth_user.clone() {
        Some(user) => match ilertref.auth_psw.clone() {
            Some(psw) => {
                let basic_string = format!("{}:{}", user.as_str(), psw.as_str());
                let basic_auth_string = format!("Basic {}", base64::encode(basic_string.as_str()));
                options.headers
                    .append("Authorization", HeaderValue::from_str(basic_auth_string.as_str()).unwrap());
            },
            None => (),
        },
        None => (),
    };

    Ok(options)
}

/* ### API Implementations ### */

pub trait HeartbeatApiResource {
    fn heartbeat(&mut self, key: &str) -> Box<&dyn BaseRequestExecutor>;
}

pub trait UserApiResource {
    fn users(&mut self) -> Box<&dyn BaseRequestExecutor>;
    fn user(&mut self, id: i64) -> Box<&dyn BaseRequestExecutor>;
}

pub trait ScheduleApiResource {
    fn schedules(&mut self) -> Box<&dyn BaseRequestExecutor>;
    fn schedule(&mut self, id: i64) -> Box<&dyn BaseRequestExecutor>;
    fn schedule_shifts(&mut self, id: i64) -> Box<&dyn BaseRequestExecutor>;
}

pub trait EventApiResource {

    fn event(&mut self, api_key: &str, event_type: ILertEventType, summary: Option<String>, alert_key: Option<String>) -> Box<&dyn BaseRequestExecutor>;

    fn event_with_details(&mut self, api_key: &str, event_type: ILertEventType, summary: Option<String>,
            alert_key: Option<String>, details: Option<String>, priority: Option<ILertPriority>, images: Option<Vec<EventImage>>,
            links: Option<Vec<EventLink>>, custom_details: Option<serde_json::Value>, routing_key: Option<String>) -> Box<&dyn BaseRequestExecutor>;

    fn event_with_comment(&mut self, api_key: &str, alert_key: Option<String>, comments: Option<Vec<EventComment>>) -> Box<&dyn BaseRequestExecutor>;
}

/* ### GET ### */

#[derive(Debug, Clone)]
pub struct GetRequestBuilder<'a> {
    builder: BaseRequestBuilder<'a>,
}

impl<'a> GetRequestBuilder<'a> {

    pub fn new(_ilert: &'a ILert) -> GetRequestBuilder<'a> {
        GetRequestBuilder {
            builder: BaseRequestBuilder::new(_ilert),
        }
    }
}

impl BaseRequestExecutor for GetRequestBuilder<'_> {

    fn execute(&self) -> ILertResult<BaseRequestResult> {

        let options_result = prepare_generic_request_builder(&self.builder);
        if options_result.is_err() {
            return Err(options_result.unwrap_err());
        }
        let options = options_result.unwrap();

        if options.url.is_none() {
            return Err(ILertError::new("Failed to build url."));
        }
        let url = options.url.unwrap();

        let response_result = self.builder._ilert.http_client
            .get(url.as_str())
            .headers(options.headers)
            .send();

        let mut response = match response_result {
            Ok(value) => value,
            Err(err) => {
                return Err(ILertError::new(err.to_string().as_str()));
            },
        };

        let response_status = response.status().clone();
        let response_headers = response.headers().clone();

        let body_raw = match response.text() {
            Ok(value) => Some(value.clone()),
            Err(_) => None,
        };

        let body_json = match body_raw.clone() {
            Some(raw_value) =>
                match response_headers.get("content-type") {
                    Some(ct_value) =>
                        if ct_value.eq(&"application/json") {
                            let parsed_json_result = serde_json::from_str(raw_value.as_str());
                            match parsed_json_result {
                                Ok(parsed_json) => Some(parsed_json),
                                Err(err) => {
                                    return Err(ILertError::new(err.to_string().as_str()));
                                },
                            }
                        } else {
                            None
                        },
                    None => None,
                },
            None => None,
        };

        Ok(BaseRequestResult::new(
            url,
            response_status,
            response_headers,
            body_raw,
            body_json,
        ))
    }
}

impl HeartbeatApiResource for GetRequestBuilder<'_> {

    fn heartbeat(&mut self, key: &str) -> Box<&dyn BaseRequestExecutor> {
        self.builder.set_path(format!("/heartbeats/{}", key).as_str());
        Box::new(self)
    }
}

impl UserApiResource for GetRequestBuilder<'_> {

    fn users(&mut self) -> Box<&dyn BaseRequestExecutor> {
        self.builder.set_path("/users");
        Box::new(self)
    }

    fn user(&mut self, id: i64) -> Box<&dyn BaseRequestExecutor> {
        self.builder.set_path(format!("/users/{}", id).as_str());
        Box::new(self)
    }
}

impl ScheduleApiResource for GetRequestBuilder<'_> {

    fn schedules(&mut self) -> Box<&dyn BaseRequestExecutor> {
        self.builder.set_path("/schedules");
        Box::new(self)
    }

    fn schedule(&mut self, id: i64) -> Box<&dyn BaseRequestExecutor> {
        self.builder.set_path(format!("/schedules/{}", id).as_str());
        Box::new(self)
    }

    fn schedule_shifts(&mut self, id: i64) -> Box<&dyn BaseRequestExecutor> {
        self.builder.set_path(format!("/schedules/{}/shifts", id).as_str());
        Box::new(self)
    }
}

/* ### POST ### */

#[derive(Debug, Clone)]
pub struct PostRequestBuilder<'a> {
    builder: BaseRequestBuilder<'a>,
}

impl<'a> PostRequestBuilder<'a> {

    pub fn new(_ilert: &'a ILert, body: &str) -> PostRequestBuilder<'a> {
        PostRequestBuilder {
            builder: BaseRequestBuilder::new(_ilert),
        }
    }
}

impl BaseRequestExecutor for PostRequestBuilder<'_> {

    fn execute(&self) -> ILertResult<BaseRequestResult> {

        let options_result = prepare_generic_request_builder(&self.builder);
        if options_result.is_err() {
            return Err(options_result.unwrap_err());
        }
        let options = options_result.unwrap();

        if options.url.is_none() {
            return Err(ILertError::new("Failed to build url."));
        }
        let url = options.url.unwrap();

        let mut response_result = self.builder._ilert.http_client
            .post(url.as_str())
            .headers(options.headers);

        response_result = match options.body {
            Some(value) => response_result.body(value),
            None => response_result,
        };

        let mut response = match response_result.send() {
            Ok(value) => value,
            Err(err) => {
                return Err(ILertError::new(err.to_string().as_str()));
            },
        };

        let response_status = response.status().clone();
        let response_headers = response.headers().clone();

        let body_raw = match response.text() {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        let body_json = match body_raw.clone() {
            Some(raw_value) =>
                match response_headers.get("content-type") {
                    Some(ct_value) =>
                        if ct_value.eq(&"application/json") {
                            let parsed_json_result = serde_json::from_str(raw_value.as_str());
                            match parsed_json_result {
                                Ok(parsed_json) => Some(parsed_json),
                                Err(err) => {
                                    return Err(ILertError::new(err.to_string().as_str()));
                                },
                            }
                        } else {
                            None
                        },
                    None => None,
                },
            None => None,
        };

        Ok(BaseRequestResult::new(
            url,
            response_status,
            response_headers,
            body_raw,
            body_json,
        ))
    }
}

impl EventApiResource for PostRequestBuilder<'_> {

    fn event(&mut self, api_key: &str, event_type: ILertEventType, summary: Option<String>, alert_key: Option<String>) -> Box<&dyn BaseRequestExecutor> {

        let json_body = json!({
            "apiKey": api_key,
            "eventType": event_type.as_str(),
            "summary": summary,
            "alertKey": alert_key
        });

        self.builder.set_path("/events");
        self.builder.set_body(json_body.to_string().as_str());
        Box::new(self)
    }

    fn event_with_details(&mut self, api_key: &str, event_type: ILertEventType, summary: Option<String>,
                          alert_key: Option<String>, details: Option<String>, priority: Option<ILertPriority>, images: Option<Vec<EventImage>>,
        links: Option<Vec<EventLink>>, custom_details: Option<serde_json::Value>, routing_key: Option<String>) -> Box<&dyn BaseRequestExecutor> {

        let priority = match priority {
            Some(e_val) => Some(e_val.as_str().to_string()),
            None => None
        };

        let json_body = json!({
            "apiKey": api_key,
            "eventType": event_type.as_str(),
            "summary": summary,
            "alertKey": alert_key,
            "details": details,
            "priority": priority,
            "images": images,
            "links": links,
            "customDetails": custom_details,
            "routingKey": routing_key
        });

        self.builder.set_path("/events");
        self.builder.set_body(json_body.to_string().as_str());
        Box::new(self)
    }

    fn event_with_comment(&mut self, api_key: &str, alert_key: Option<String>, comments: Option<Vec<EventComment>>) -> Box<&dyn BaseRequestExecutor> {

        let json_body = json!({
            "apiKey": api_key,
            "eventType": ILertEventType::COMMENT.as_str(),
            "alertKey": alert_key,
            "comments": comments,
        });

        self.builder.set_path("/events");
        self.builder.set_body(json_body.to_string().as_str());
        Box::new(self)
    }
}
