//! AlphaIru
//! Twitch Reader
//!
//! auth/oauth.rs
//! 
//! This is the module for twitch authentication,
//! and this one handles the getting the token
//!
//!


use std::collections::HashMap;
use std::env;

use open::that;
use tiny_http::{
    Header,
    Method,
    Response,
    Server,
};
use twitch_oauth2::{
    ClientId,
    ClientSecret,
    Scope::{
        ChatRead,
        ChatEdit,
    },
    url::Url,
    UserTokenBuilder,
};


pub async fn get_token() -> Result<(String, String), Box<dyn std::error::Error>> {
   let reqwest = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let client_id = env::var("TWITCH_CLIENT_ID")
        .expect("TWITCH_CLIENT_ID must be set!");
    let client_secret = env::var("TWITCH_CLIENT_SECRET")
        .expect("TWITCH_CLIENT_SECRET must be set!");
    let redirect_uri = env::var("TWITCH_REDIRECT_URI")
        .expect("TWITCH_REDIRECT_URI must be set!");

    let mut builder = UserTokenBuilder::new(
        ClientId::from(client_id),
        ClientSecret::from(client_secret),
        Url::parse(&redirect_uri)?,
    )
    .force_verify(true)
    .set_scopes(vec![
        ChatRead,
        ChatEdit
    ]);

    let (url, _) = builder.generate_url();
    that(url.as_str())?;

    let parsed_url = Url::parse(&redirect_uri)?;
    let addr = format!(
        "{}:{}",
        parsed_url.host_str().unwrap_or("127.0.0.1"),
        parsed_url.port().unwrap_or(80),
    );

    let server = Server::http(addr)
        .map_err(|e| -> Box<dyn std::error::Error> {e})?;

    let code_url = wait_for_callback(&server, &redirect_uri)?;
    let given_url = Url::parse(&code_url)?;

    let map: HashMap<_, _> = given_url.query_pairs().collect();
    
    let code = map.get("code").ok_or("No code")?;
    let state = map.get("state").ok_or("No state")?;

    if !builder.csrf_is_valid(state) {
        return Err("Invalid CSFR state".into());
    }

    let token = builder.get_user_token(&reqwest, state, code).await?;

    let access_token = token.access_token.secret().to_string();
    let refresh_token = token
        .refresh_token
        .as_ref()
        .map(|r| r.secret().to_string())
        .unwrap_or_default();

    Ok((access_token, refresh_token))
}

fn wait_for_callback(
    server: &Server,
    redirect_uri: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let channel_name = env::var("TWITCH_USERNAME")
        .expect("TWITCH_USERNAME must be set!");

    for request in server.incoming_requests() {
        match request.method() {
            Method::Get => {
                let code_url = format!("{}{}", redirect_uri, request.url());
                let html_template = std::fs::read_to_string("web/oauth_success.html")?;
                let html_page = html_template.replace(
                    "__TWITCH_CHANNEL__",
                    &channel_name,
                );

                let content_type = Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"text/html; charset=utf-8"[..],
                )
                .map_err(|_| "failed to build Content-Type header")?;

                let response = Response::from_string(html_page)
                    .with_status_code(200)
                    .with_header(content_type);

                let _ = request.respond(response);
                return Ok(code_url);
            }
            _ => {
                let response = Response::from_string("Invalid request!");
                let _ = request.respond(response);
            }
        }
    }

    Err("No callback recieved!".into())
}

