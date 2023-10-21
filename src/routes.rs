use crate::auth::{AccessTokenRequest, OAuthResponse};
use crate::client::{get_client_id, get_client_secret, get_redirect_uri};
use base64::{engine::general_purpose, Engine as _};
use rocket::serde::json::Json;
use rocket::{http::uri::Origin, response::Redirect, Route};

const BASE: &str = "https://accounts.spotify.com";

#[get("/login")]
pub fn index() -> Redirect {
    let scope = "playlist-modify-private%20playlist-modify-public";
    let client_id = get_client_id();
    let redirect_uri = get_redirect_uri();

    let spotify_auth_url = format!("{BASE}/authorize?response_type=code&client_id={client_id}&scope={scope}&redirect_uri={redirect_uri}");
    let redirect = Redirect::to(spotify_auth_url);
    redirect
}

#[get("/callback")]
pub async fn callback(uri: &Origin<'_>) -> Json<OAuthResponse> {
    let query: Vec<_> = uri.query().unwrap().segments().collect();
    let code = query[0].1;

    let access_token_request_body = AccessTokenRequest {
        grant_type: "authorization_code",
        code,
        redirect_uri: &get_redirect_uri(),
    };

    let data_to_send =
        serde_urlencoded::to_string(access_token_request_body).expect("serialize issue");

    let secret: String = format!("{}:{}", get_client_id(), get_client_secret());

    let client = reqwest::Client::new();
    let response_result = client
        .post(format!("{BASE}/api/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header(
            "Authorization",
            "Basic ".to_string() + &general_purpose::STANDARD.encode(secret),
        )
        .body(data_to_send)
        .send()
        .await
        .unwrap();

    let result = response_result.json::<OAuthResponse>().await.unwrap();
    return Json(result);
}

pub fn routes() -> Vec<Route> {
    routes![index, callback]
}
