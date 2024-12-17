use reqwest::blocking::*;
use log::*;
use serde::Deserialize;

const OAUTH_TOKEN : (&str, &str)= (
    "3f69e56c7649492c8cc29f1af08a8a12", // Client
    "b51ee9cb12234f50a69efa67ef53812e", // Secret
);

#[derive(Debug)]
pub struct DeviceCredentials {
    access_token : String,
    account_id : String,
    device_id : String,
}
impl DeviceCredentials {
    pub fn new() -> DeviceCredentials {
        DeviceCredentials{
            device_id: String::new(),
            account_id: String::new(),
            access_token: String::new(),
        }
    }
    pub fn get_access_token_and_account_id(&mut self, http_client: &Client, authcode: &str) {
        
        if authcode.is_empty() {
            error!("Authentication Code cannot be empty");
            return;
        }
        
        #[derive(Debug, Deserialize)]
        struct ResponseStruct {
            access_token: String,
            expires_in: u16,
            expires_at: String,
            token_type: String,
            refresh_token: String,
            refresh_expires: u16,
            refresh_expires_at: String,
            account_id: String,
            client_id: String,
            internal_client: bool,
            client_service: String,
            scope: Vec<String>,
            displayName: String,
            app: String,
            in_app_id: String,
            product_id: String,
            application_id: String,
            acr: String,
            auth_time: String
        }

        let url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
        let mut req_body: String = String::from("grant_type=authorization_code&code=").trim().to_string();
        req_body.push_str(authcode.trim());

        let response = Client::post(&http_client, url)
            .body(req_body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization",
                    "Basic M2Y2OWU1NmM3NjQ5NDkyYzhjYzI5ZjFhZjA4YThhMTI6YjUxZWU5Y2IxMjIzNGY1MGE2OWVmYTY3ZWY1MzgxMmU=")
            .send();

        match response {
            Ok(response_data) => {
                match response_data.json::<ResponseStruct>().ok() {
                    Some(response) => {
                        self.access_token = response.access_token;
                        self.account_id = response.account_id;
                    },
                    None => {
                        error!("Failed to get access_token!");
                    }
                }
            },
            _ => { error!("Failed to get access token!"); },
        }
    }
    
    pub fn get_device_auth(&mut self, http_client: &Client) {
        
        if self.access_token.is_empty() || self.account_id.is_empty() {
            error!("Device access token cannot be empty!");
            return;
        }

        let mut url : String = String::from("https://account-public-service-prod.ol.epicgames.com/account/api/public/account/");
        url.push_str(self.account_id.as_str());
        url.push_str("/deviceAuth");

        /*
        let it : String = String::new;
        it.starts_with("one thing");
        // Just discovered String.starts_with() KEKW
        */
        
        #[derive(Debug, Deserialize)]
        struct CreatedObject {
            location: String,
            ipAddress: String,
            dateTime: String,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseStruct {
            deviceId: String,
            accountId: String,
            secret: String,
            userAgent: String,
            created: CreatedObject,
        }

        let mut bearer_header = String::from("Bearer ");
        bearer_header.push_str(self.access_token.as_str());

        let response = Client::post(&http_client, url)
            .header("Authorization", bearer_header.as_str())
            .send();

        match response {
            Ok(response_data) => {
                match response_data.json::<ResponseStruct>().ok() {
                    Some(response_json) => {
                        self.device_id = response_json.deviceId;
                    }
                    None => {
                        error!("Failed to parse device_id!");
                    },
                }
            },
            _ => { error!("Failed to get device_id!"); },
        }
    }
    pub fn get_device_auth_from(&mut self, access_token: &str, account_id: &str) {
        
        if access_token.is_empty() || account_id.is_empty() {
            error!("Device access token cannot be empty!");
            return;
        }
        
        let mut url : String = String::from("https://account-public-service-prod.ol.epicgames.com/account/api/public/account/");
        url.push_str(account_id.as_str());
        url.push_str("/deviceAuth");

        /*
        let it : String = String::new;
        it.starts_with("one thing");
        // Just discovered String.starts_with() KEKW
        */

        #[derive(Debug, Deserialize)]
        struct CreatedObject {
            location: String,
            ipAddress: String,
            dateTime: String,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseStruct {
            deviceId: String,
            accountId: String,
            secret: String,
            userAgent: String,
            created: CreatedObject,
        }

        let mut bearer_header = String::from("Bearer ");
        bearer_header.push_str(access_token.as_str());

        let response = Client::post(&http_client, url)
            .header("Authorization", bearer_header.as_str())
            .send();

        match response {
            Ok(response_data) => {
                match response_data.json::<ResponseStruct>().ok() {
                    Some(response_json) => {
                        self.device_id = response_json.deviceId;
                    }
                    None => {
                        error!("Failed to parse device_id!");
                    },
                }
            },
            _ => { error!("Failed to get device_id!"); },
        }
    }
    
    pub fn get_exchange_code() {
        todo!();
    }
}
