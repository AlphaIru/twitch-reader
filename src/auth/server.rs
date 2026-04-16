use std::env;
use std::error::Error;
use url::Url;

use tiny_http::{Response, Server};

pub fn wait_for_code() -> Result<String, Box<dyn Error>> {
    let server_url = env::var("SERVER_URL")
        .expect("Error: .env file not found or SERVER_URL must be set");
    let redirect_url = env::var("REDIRECT_URL")
        .expect("Error: .env file not found or REDIRECT_URL must be set");

    let server = Server::http(server_url).map_err(|e| e.to_string())?;
    let request = server.incoming_request().next().ok_or("Request Failed!")?;

    let url = Url::parse(&format!("{}{}", redirect_url, request.url()))?;

    let code = url.query_pairs()
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.into_owned())
        .ok_or("Code not found!")?;

    request.respond(Response::from_string("Authentication successful!"))?;
    Ok(code)
}

