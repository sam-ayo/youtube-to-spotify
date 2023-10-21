pub fn get_client_id() -> String {
    let client_id: String = match std::env::var("CLIENT_ID") {
        Ok(id) => id,
        Err(e) => panic!("{}", e),
    };
    client_id
}

pub fn get_client_secret() -> String {
    let client_secret: String = match std::env::var("CLIENT_SECRET") {
        Ok(secret) => secret,
        Err(e) => panic!("{}", e),
    };
    client_secret
}

pub fn get_redirect_uri() -> String {
    let redirect_uri: String = match std::env::var("REDIRECT_URI") {
        Ok(uri) => uri,
        Err(e) => panic!("{}", e),
    };
    redirect_uri
}
