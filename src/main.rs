use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket::{http::uri::Origin, response::Redirect};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;
extern crate base64;

const BASE: &str = "https://accounts.spotify.com";

fn get_client_id() -> String {
    let client_id: String = match std::env::var("CLIENT_ID") {
        Ok(id) => id,
        Err(e) => panic!("{}", e),
    };
    client_id
}

fn get_client_secret() -> String {
    let client_secret: String = match std::env::var("CLIENT_SECRET") {
        Ok(secret) => secret,
        Err(e) => panic!("{}", e),
    };
    client_secret
}

fn get_redirect_uri() -> String {
    let redirect_uri: String = match std::env::var("REDIRECT_URI") {
        Ok(uri) => uri,
        Err(e) => panic!("{}", e),
    };
    redirect_uri
}

#[get("/login")]
fn index() -> Redirect {
    let scope = "playlist-modify-private%20playlist-modify-public";
    let client_id = get_client_id();
    let redirect_uri = get_redirect_uri();

    let spotify_auth_url = format!("{BASE}/authorize?response_type=code&client_id={client_id}&scope={scope}&redirect_uri={redirect_uri}");
    let redirect = Redirect::to(spotify_auth_url);
    redirect
}

#[derive(Serialize, Deserialize)]
struct AccessTokenRequest<'a> {
    grant_type: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct OAuthResponse {
    access_token: String,
    token_type: String,
    expires_in: u16,
    refresh_token: String,
    scope: String,
}

use std::error::Error;

#[get("/callback")]
async fn callback(uri: &Origin<'_>) -> Json<OAuthResponse> {
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

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index, callback])
}
