use crate::auth::GameAuthenticationError::{IllegalParameter, IllegalRequest, IllegalResponse};
use log::*;
use reqwest::{blocking::*, header::*, StatusCode};
use serde::*;
use std::collections::HashMap;
use std::{error::Error, fmt::Display};

const OAUTH_TOKEN: (&str, &str) = (
    "3f69e56c7649492c8cc29f1af08a8a12", // Client
    "b51ee9cb12234f50a69efa67ef53812e", // Secret
);

/// Describes possible errors that may come from the functions and methods in this module.
#[derive(Debug)]
pub enum GameAuthenticationError {
    IllegalParameter(String),
    IllegalRequest(String),
    IllegalResponse(String),
    RequestFailed,
}

impl Display for GameAuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GameAuthenticationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!();
        match self {
            IllegalParameter(reason) => Some(&IllegalParameter(reason.to_owned())),
            IllegalResponse(reason) => Some(&IllegalResponse(reason.to_owned())),
            GameAuthenticationError::IllegalRequest(reason) => {
                Some(&IllegalRequest(reason.to_owned()))
            }
            _ => None,
        }
    }
}

pub enum ClientType {
    ANDROID,
    GENERIC,
    PC,
}

#[derive(Debug)]
pub struct AccessToken(pub String);

impl AccessToken {
    pub fn new() -> AccessToken {
        AccessToken(String::new())
    }

    pub fn from(access_token: AccessToken) -> AccessToken {
        AccessToken(access_token.0)
    }
    pub fn from_authcode(
        http_client: &reqwest::blocking::Client,
        authcode: &str,
        persistent_credentials: Option<&mut PersistentCredentials>,
        client_type: ClientType,
    ) -> Result<AccessToken, Box<dyn Error>> {
        if authcode.is_empty() {
            error!("Authentication code for access token cannot be empty");
            Err(Box::new(GameAuthenticationError::IllegalParameter(
                "Authentication code for access token cannot be empty".to_string(),
            )))?;
        }

        #[derive(Debug, Deserialize)]
        struct ResponseContent {
            access_token: String,
            account_id: String,
        }

        const URL: &str =
            "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";

        /* Request Headers */
        let mut request_headers = HeaderMap::new();
        request_headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        request_headers.insert(
            "Authorization",
            HeaderValue::from_static(match client_type {
                ClientType::ANDROID => "Basic M2Y2OWU1NmM3NjQ5NDkyYzhjYzI5ZjFhZjA4YThhMTI6YjUxZWU5Y2IxMjIzNGY1MGE2OWVmYTY3ZWY1MzgxMmU=",
                ClientType::PC => "BASIC MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y=",
                ClientType::GENERIC => "Basic MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y=",
            }),
        );

        /* Request Body */
        // "grant_type=authorization_code&code=<authcode>"
        let request_body: reqwest::blocking::Body = reqwest::blocking::Body::from(
            "grant_type=authorization_code&code=".trim().to_owned() + authcode.trim(),
        );

        /* Response Fetching */
        let response = http_client
            .post(URL)
            .headers(request_headers)
            .body(request_body)
            .send()?;

        info!("Got HTTP response: {:?}", response);
        info!("Got HTTP response headers: {:?}", response.headers());
        match response.status() {
            StatusCode::OK => {
                let response_content: ResponseContent = response.json::<ResponseContent>()?;

                // WARN: Side effect here!!!
                if persistent_credentials.is_some() {
                    persistent_credentials.unwrap().account_id = response_content.account_id;
                }

                return Ok(AccessToken(response_content.access_token));
            }

            StatusCode::INTERNAL_SERVER_ERROR => {
                error!("Internal Server Error when fetching Access Token with Authentication Code");
                Err(Box::new(GameAuthenticationError::RequestFailed))?
            }
            StatusCode::UNAUTHORIZED => {
                error!("Unauthorized action when fetching Access Token with Authentication Code");
                Err(Box::new(GameAuthenticationError::IllegalRequest(
                    "Unauthorized action when fetching Access Token with Authentication Code"
                        .to_string(),
                )))?
            }
            _ => {
                error!("Unsupported response when fetching Access Token with Authentication Code");
                error!("Got response {:?}", response.text().unwrap_or_default());
                Err(Box::new(GameAuthenticationError::IllegalResponse(
                    "Unsupported response when fetching Access Token with Authentication Code"
                        .to_string(),
                )))?
            }
        }
    }
    pub fn from_persistent_credentials(
        http_client: &reqwest::blocking::Client,
        credentials: &PersistentCredentials,
        client_type: ClientType,
    ) -> Result<AccessToken, Box<dyn Error>> {
        // Checking credentials' content
        {
            if credentials.account_id.is_empty() {
                Err(Box::new(IllegalParameter(
                    "credentials.account_id cannot be empty".to_string(),
                )))?;
            }
            if credentials.device_id.is_empty() {
                Err(Box::new(IllegalParameter(
                    "credentials.device_id cannot be empty".to_string(),
                )))?;
            }
            if credentials.secret.is_empty() {
                Err(Box::new(IllegalParameter(
                    "credentials.secret cannot be empty".to_string(),
                )))?
            }
        }

        #[derive(Debug, Deserialize)]
        struct ResponseContent {
            access_token: String,
        }

        const URL: &str =
            "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";

        let mut request_headers: HeaderMap = HeaderMap::new();
        request_headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        request_headers.insert(
            "Authorization",
            HeaderValue::from_static(match client_type {
                ClientType::ANDROID => "Basic M2Y2OWU1NmM3NjQ5NDkyYzhjYzI5ZjFhZjA4YThhMTI6YjUxZWU5Y2IxMjIzNGY1MGE2OWVmYTY3ZWY1MzgxMmU=",
                ClientType::PC => "BASIC MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y=",
                ClientType::GENERIC => "Basic MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y=",
            })
        );

        let request_body: reqwest::blocking::Body = reqwest::blocking::Body::from(
            "grant_type=device_auth".to_string()
                + "&account_id=".trim()
                + &credentials.account_id
                + "&device_id=".trim()
                + &credentials.device_id
                + "&secret=".trim()
                + &credentials.secret,
        );

        let response = http_client
            .post(URL)
            .headers(request_headers)
            .body(request_body)
            .send()?;

        match response.status() {
            StatusCode::OK => {
                let response_content: ResponseContent = response.json::<ResponseContent>()?;
                return Ok(AccessToken(response_content.access_token));
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                error!(
                    "Internal Server Error when fetching Access Token with Persistent Credentials"
                );
                Err(Box::new(GameAuthenticationError::RequestFailed))?
            }
            StatusCode::UNAUTHORIZED => {
                error!(
                    "Unauthorized action when fetching Access Token with Persistent Credentials"
                );
                Err(Box::new(GameAuthenticationError::IllegalRequest(
                    "Unauthorized action when fetching Access Token with Persistent Credentials"
                        .to_string(),
                )))?
            }
            _ => {
                error!(
                    "Unsupported response when fetching Access Token with Persistent Credentials"
                );
                error!("Got response:\n {:?}", response.text().unwrap_or_default());
                Err(Box::new(GameAuthenticationError::IllegalResponse(
                    "Unsupported response when fetching Access Token with Persistent Credentials"
                        .to_string(),
                )))?
            }
        }
    }
    pub fn from_exchange_code(
        http_client: &Client,
        exchange_code: &ExchangeCode,
    ) -> Result<AccessToken, Box<dyn Error>> {
        if exchange_code.0.is_empty() {
            Err(Box::new(GameAuthenticationError::IllegalParameter(
                "exchange_code cannot be empty".to_owned(),
            )))?
        }

        #[derive(Debug, Deserialize)]
        struct ResponseContent {
            access_token: String,
        }

        const URL: &str =
            "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";

        let mut request_headers: HeaderMap = HeaderMap::new();
        request_headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        request_headers.insert(
            "Authorization",
            HeaderValue::from_static("Basic MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y="),
        );

        let request_body: reqwest::blocking::Body = reqwest::blocking::Body::from(
            "grant_type=exchange_code".to_string() + "&exchange_code=" + &exchange_code.0,
        );

        let response = http_client
            .post(URL)
            .headers(request_headers)
            .body(request_body)
            .send()?;

        match response.status() {
            StatusCode::OK => {
                let response_content: ResponseContent = response.json::<ResponseContent>()?;
                Ok(AccessToken(response_content.access_token))
            }

            StatusCode::INTERNAL_SERVER_ERROR => {
                error!("Internal Server Error when fetching Access Token with Exchange Code");
                Err(Box::new(GameAuthenticationError::RequestFailed))?
            }
            StatusCode::UNAUTHORIZED => {
                error!("Unauthorized action when fetching Access Token with Exchange Code");
                Err(Box::new(GameAuthenticationError::IllegalRequest(
                    "Unauthorized action when fetching Access Token with Exchange Code".to_string(),
                )))?
            }
            _ => {
                error!("Unsupported response when fetching Access Token with Exchange Code");
                error!("Got response {:?}", response.text().unwrap_or_default());
                Err(Box::new(GameAuthenticationError::IllegalResponse(
                    "Unsupported response when fetching Access Token with Exchange Code"
                        .to_string(),
                )))?
            }
        }
    }
}

/// Contains the account info that can be kept on the device to login in the game
#[derive(Debug)]
pub struct PersistentCredentials {
    pub account_id: String,
    device_id: String,
    secret: String,
}

impl PersistentCredentials {
    /// Creates a new empty instance of PersistentCredentials.
    pub fn new() -> PersistentCredentials {
        PersistentCredentials {
            account_id: String::new(),
            device_id: String::new(),
            secret: String::new(),
        }
    }

    /// Creates a new instance of PersistentCredentials with the given String parameters.
    pub fn from(account_id: String, device_id: String, secret: String) -> PersistentCredentials {
        PersistentCredentials {
            account_id,
            device_id,
            secret,
        }
    }

    /// Fetches the Persistent credentials
    pub fn fetch(
        http_client: &reqwest::blocking::Client,
        authcode: &str,
    ) -> Result<PersistentCredentials, Box<dyn Error>> {
        if authcode.is_empty() {
            Err(GameAuthenticationError::IllegalParameter(
                "Authorization code cannot be empty".to_string(),
            ))?
        }

        // PersistentCredentials to be returned by this function.
        let mut resulting_credentials = PersistentCredentials::new();

        // Getting access token and account id
        info!("Getting Access Token");
        let access_token: AccessToken = AccessToken::from_authcode(
            http_client,
            &authcode,
            Some(&mut resulting_credentials),
            ClientType::ANDROID,
        )?;

        // Getting device_id and secret
        {
            let url = format!("https://account-public-service-prod.ol.epicgames.com/account/api/public/account/{}/deviceAuth", resulting_credentials.account_id);

            let mut headers = HeaderMap::new();
            headers.insert(
                "Authorization",
                format!("Bearer {}", access_token.0).parse().unwrap(),
            );

            #[derive(Debug, Deserialize)]
            struct ResponseContent {
                deviceId: String,
                accountId: String,
                secret: String,
            }

            let response = http_client.post(&url).headers(headers).send()?;

            match response.status() {
                StatusCode::OK => {
                    let response_content: ResponseContent = response.json::<ResponseContent>()?;
                    resulting_credentials.device_id = response_content.deviceId;
                    resulting_credentials.account_id = response_content.accountId;
                    resulting_credentials.secret = response_content.secret;
                    Ok(resulting_credentials)
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    error!("Internal Server Error when fetching device_id and secret with Access Token");
                    Err(Box::new(GameAuthenticationError::RequestFailed))?
                }
                StatusCode::UNAUTHORIZED => {
                    error!(
                        "Unauthorized action when fetching device_id and secret with Access Token"
                    );
                    Err(Box::new(GameAuthenticationError::IllegalRequest(
                        "Unauthorized action when fetching device_id and secret with Access Token"
                            .to_string(),
                    )))?
                }
                _ => {
                    error!(
                        "Unsupported response when fetching device_id and secret with Access Token"
                    );
                    error!("Got response {:?}", response.text().unwrap_or_default());
                    Err(Box::new(GameAuthenticationError::IllegalResponse(
                        "Unsupported response when fetching device_id and secret with Access Token"
                            .to_string(),
                    )))?
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ExchangeCode(pub String);
impl ExchangeCode {
    pub fn new() -> ExchangeCode {
        ExchangeCode(String::new())
    }

    pub fn from_persistent_credentials(
        http_client: &reqwest::blocking::Client,
        access_token: &AccessToken,
        client_type: ClientType,
    ) -> Result<ExchangeCode, GameAuthenticationError> {
        #[derive(Debug, Deserialize)]
        struct ResponseStruct {
            code: String,
        }

        let mut bearer = HeaderMap::new();
        bearer.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", access_token.0)).unwrap(),
        );
        let query_content = [(
            "consumingClientId",
            match client_type {
                ClientType::ANDROID => "34a02cf8f4414e29b15921876da36f9a",
                ClientType::PC | ClientType::GENERIC => "ec684b8c687f479fadea3cb2ad83f5c6",
            },
        )];
        let url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/exchange";

        let response = http_client
            .get(url)
            .query(&query_content)
            .headers(bearer)
            .send();

        match response {
            Ok(response_data) => match response_data.json::<ResponseStruct>() {
                Ok(response_json) => {
                    return Ok(ExchangeCode(response_json.code));
                }
                _ => Err(GameAuthenticationError::IllegalResponse(
                    "Could not parse exchange code".to_string(),
                ))?,
            },
            _ => Err(GameAuthenticationError::RequestFailed)?,
        }
    }
}
