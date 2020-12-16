use reqwest::Client;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use crate::config;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "accessTokenExpirationTimestampMs")]
    pub expires_at: u64,
    #[serde(rename = "isAnonymous")]
    pub is_anonymous: bool,
}

pub fn fetch_illegal_access_token() -> Option<TokenInfo> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    let path = config::config_path("cookie.txt");
    let mut contents = std::fs::read_to_string(path)
        .map_err(|e| format!("unable to read: {}", e)).unwrap();
    contents.pop(); // remove newline from the end
    debug!("found user cookies: \"{}\"", contents.clone());
    headers.insert(COOKIE, HeaderValue::from_str(contents.as_str()).unwrap());

    let mut response = client.get("https://open.spotify.com/get_access_token").headers(headers).send().expect("send request failed");
    if response.status().is_success() {
        let token_info: TokenInfo = response.json().unwrap();
        debug!("access_token is: {}", token_info.access_token);
        Some(token_info)
    } else {
        error!("failed retrieving token, error: {:?}", response);
        None
    }
}
