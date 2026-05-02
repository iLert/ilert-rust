use reqwest::redirect::Policy;
use reqwest::ClientBuilder;
use reqwest::header;
use std::time::Duration;
use log::{debug};

use crate::ilert_builders::{DeleteRequestBuilder, GetRequestBuilder, HeadRequestBuilder,
                            PostRequestBuilder, PutRequestBuilder};
use crate::ilert_error::{ILertResult, ILertError};
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ILert {
    host: String,
    hbt_host: String,
    api_ep: String,
    pub api_token: Option<String>,
    pub auth_user: Option<String>,
    pub auth_psw: Option<String>,
    pub http_client: reqwest::Client,
}

impl ILert {

    pub fn new() -> ILertResult<ILert> {
        ILert::new_with_opts(None, None, None, None)
    }

    pub fn new_with_opts(host: Option<&str>, hbt_host: Option<&str>, timeout_sec: Option<u64>, caller_agent: Option<&str>) -> ILertResult<ILert> {
        let http_client_result = ILert::get_http_client(timeout_sec.unwrap_or(25), caller_agent);
        match http_client_result {
            Err(err) => Err(ILertError::new(err.to_string().as_str())),
            Ok(http_client) => Ok(ILert {
                host: host.unwrap_or("https://api.ilert.com").to_string(),
                hbt_host: hbt_host.unwrap_or("https://beat.ilert.com").to_string(),
                api_ep: "/api".to_string(),
                api_token: None,
                auth_user: None,
                auth_psw: None,
                http_client,
            })
        }
    }

    fn get_default_headers(caller_agent: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let ua = match caller_agent {
            Some(caller) => format!("{} ilert-rust/{}", caller, env!("CARGO_PKG_VERSION")),
            None => format!("ilert-rust/{}", env!("CARGO_PKG_VERSION")),
        };
        headers.append("User-Agent", HeaderValue::from_str(&ua).unwrap());
        headers.append("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.append("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers
    }

    fn get_http_client(timeout_sec: u64, caller_agent: Option<&str>) -> reqwest::Result<reqwest::Client> {
        let headers = ILert::get_default_headers(caller_agent);

        reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_sec))
            .redirect(Policy::none())
            .default_headers(headers)
            .build()
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

    pub fn build_hbt_url(&self, path: &str) -> String {
        let url = format!("{}{}{}", self.hbt_host.as_str(), self.api_ep.as_str(), path);
        debug!("{}", url);
        url
    }

    pub fn head(&self) -> HeadRequestBuilder<'_> {
        HeadRequestBuilder::new(self)
    }

    pub fn get(&self) -> GetRequestBuilder<'_> {
        GetRequestBuilder::new(self)
    }

    #[deprecated(since="3.0.0", note="please use `create()` instead")]
    pub fn post(&self) -> PostRequestBuilder<'_> {
        PostRequestBuilder::new(self, "{}")
    }

    pub fn create(&self) -> PostRequestBuilder<'_> {
        PostRequestBuilder::new(self, "{}")
    }

    pub fn update(&self) -> PutRequestBuilder<'_> {
        PutRequestBuilder::new(self, "{}")
    }

    pub fn delete(&self) -> DeleteRequestBuilder<'_> {
        DeleteRequestBuilder::new(self)
    }
}
