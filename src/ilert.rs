use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use reqwest::ClientBuilder;
use reqwest::header;
use std::time::Duration;
use log::{debug};

use crate::ilert_builders::{DeleteRequestBuilder, GetRequestBuilder, PostRequestBuilder, PutRequestBuilder};
use crate::ilert_error::{ILertResult, ILertError};
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ILert {
    host: String,
    api_ep: String,
    pub api_token: Option<String>,
    pub auth_user: Option<String>,
    pub auth_psw: Option<String>,
    pub http_client: Client,
}

impl ILert {

    pub fn new() -> ILertResult<ILert> {
        let http_client_result = ILert::get_http_client(25);
        match http_client_result {
            Err(err) => Err(ILertError::new(err.to_string().as_str())),
            Ok(http_client) => Ok(ILert {
                host: "https://api.ilert.com".to_string(),
                api_ep: "/api".to_string(),
                api_token: None,
                auth_user: None,
                auth_psw: None,
                http_client,
            })
        }
    }

    pub fn new_with_opts(host: Option<&str>, timeout_sec: Option<u64>) -> ILertResult<ILert> {
        let http_client_result = ILert::get_http_client(timeout_sec.unwrap_or(25));
        match http_client_result {
            Err(err) => Err(ILertError::new(err.to_string().as_str())),
            Ok(http_client) => Ok(ILert {
                host: host.unwrap_or("https://api.ilert.com").to_string(),
                api_ep: "/api".to_string(),
                api_token: None,
                auth_user: None,
                auth_psw: None,
                http_client,
            })
        }
    }

    fn get_default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append("User-Agent", HeaderValue::from_str("ilert-rust/3.1.0").unwrap());
        headers.append("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.append("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers
    }

    fn get_http_client(timeout_sec: u64) -> reqwest::Result<Client> {

        let headers = ILert::get_default_headers();
        let http_client_result = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(timeout_sec))
            .redirect(Policy::none())
            .default_headers(headers)
            .build();

        http_client_result
    }

    pub fn auth_via_token(&mut self, api_token: &str) -> ILertResult<&mut ILert> {
        self.api_token = Some(api_token.to_string());
        Ok(self)
    }

    pub fn auth_via_user(&mut self, auth_user: &str, auth_psw: &str) -> ILertResult<&mut ILert> {
        self.auth_user = Some(auth_user.to_string());
        self.auth_psw = Some(auth_psw.to_string());
        Ok(self)
    }

    pub fn build_url(&self, path: &str) -> String {
        let url = format!("{}{}{}", self.host.as_str(), self.api_ep.as_str(), path);
        debug!("{}", url);
        url
    }

    pub fn get(&self) -> GetRequestBuilder {
        GetRequestBuilder::new(self)
    }

    #[deprecated(since="3.0.0", note="please use `create()` instead")]
    pub fn post(&self) -> PostRequestBuilder {
        PostRequestBuilder::new(self, "{}")
    }

    pub fn create(&self) -> PostRequestBuilder {
        PostRequestBuilder::new(self, "{}")
    }

    pub fn update(&self) -> PutRequestBuilder {
        PutRequestBuilder::new(self, "{}")
    }

    pub fn delete(&self) -> DeleteRequestBuilder {
        DeleteRequestBuilder::new(self)
    }
}
