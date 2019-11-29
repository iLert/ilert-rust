use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use base64;
use reqwest::StatusCode;
use serde_json::{Result, Value};
use serde_json::json;

use crate::ilert::ILert;
use crate::ilert_error::{ILertResult, ILertError};
use std::error::Error;

pub enum ILertEventType {
    ALERT,
    ACCEPT,
    RESOLVE,
}

impl ILertEventType {

    pub fn as_str(&self) -> &str {
        match self {
            &ILertEventType::ALERT => "ALERT",
            &ILertEventType::ACCEPT => "ACCEPT",
            &ILertEventType::RESOLVE => "RESOLVE",
        }
    }

    pub fn from_str(val: &str) -> ILertResult<ILertEventType> {
        match val {
            "ALERT" => Ok(ILertEventType::ALERT),
            "ACCEPT" => Ok(ILertEventType::ACCEPT),
            "RESOLVE" => Ok(ILertEventType::RESOLVE),
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
            .append("X-Api-Key", HeaderValue::from_str(token.as_str()).unwrap()),
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
    fn event(&mut self, api_key: &str, event_type: ILertEventType, summary: &str,
        details: Option<String>, incident_key: Option<String>) -> Box<&dyn BaseRequestExecutor>;
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
        dbg!(options.clone());

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
                return Err(ILertError::new(err.description()));
            },
        };

        let body_result = response.text();
        let body_raw = match body_result {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        let body_json = match body_raw.clone() {
            Some(raw_value) =>
                match response.headers().get("content-type") {
                    Some(ct_value) =>
                        if ct_value.eq(&"application/json") {
                            let parsed_json_result = serde_json::from_str(raw_value.as_str());
                            match parsed_json_result {
                                Ok(parsed_json) => Some(parsed_json),
                                Err(err) => {
                                    return Err(ILertError::new(err.description()));
                                },
                            }
                        } else {
                            None
                        },
                    None => None,
                },
            None => None,
        };

        dbg!(Ok(BaseRequestResult::new(
            url,
            response.status(),
            response.headers().clone(),
            body_raw,
            body_json,
        )))
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
        dbg!(options.clone());

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
                return Err(ILertError::new(err.description()));
            },
        };

        let body_result = response.text();
        let body_raw = match body_result {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        let body_json = match body_raw.clone() {
            Some(raw_value) =>
                match response.headers().get("content-type") {
                    Some(ct_value) =>
                        if ct_value.eq(&"application/json") {
                            let parsed_json_result = serde_json::from_str(raw_value.as_str());
                            match parsed_json_result {
                                Ok(parsed_json) => Some(parsed_json),
                                Err(err) => {
                                    return Err(ILertError::new(err.description()));
                                },
                            }
                        } else {
                            None
                        },
                    None => None,
                },
            None => None,
        };

        dbg!(Ok(BaseRequestResult::new(
            url,
            response.status(),
            response.headers().clone(),
            body_raw,
            body_json,
        )))
    }
}

impl EventApiResource for PostRequestBuilder<'_> {

    fn event(&mut self, api_key: &str, event_type: ILertEventType, summary: &str, details: Option<String>, incident_key: Option<String>) -> Box<&dyn BaseRequestExecutor> {

        let json_body = json!({
            "apiKey": api_key,
            "eventType": event_type.as_str(),
            "summary": summary,
            "details": details,
            "incidentKey": incident_key,
        });

        self.builder.set_path("/events");
        self.builder.set_body(json_body.to_string().as_str());
        Box::new(self)
    }
}
