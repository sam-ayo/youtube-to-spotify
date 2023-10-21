use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct AccessTokenRequest<'a> {
    pub grant_type: &'a str,
    pub code: &'a str,
    pub redirect_uri: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub refresh_token: String,
    pub scope: String,
}
