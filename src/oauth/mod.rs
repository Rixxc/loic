use serde::Deserialize;
use rocket::{Route, State, get, http::{Cookie, CookieJar, SameSite}, response::Redirect};

use crate::common::LocalConfig;

pub fn routes() -> Vec<Route> {
    rocket::routes![
        login_redirect,
        callback
    ]
}

#[get("/login")]
fn login_redirect(config: State<LocalConfig>) -> Redirect {
    let redirect_url = format!("{}/oauth/callback", config.base_url);
    Redirect::to(format!("{}/oauth/authorize?response_type=code&redirect_uri={}&scope=read_user&client_id={}", config.gitlab_oauth_base_url, redirect_url, config.gitlab_oauth_client_id))
}

#[derive(Deserialize)]
struct AccessTokenAnswer {
    access_token: String
}

#[derive(Deserialize)]
struct UserInfo {
    username: String
}

#[get("/callback?<code>")]
async fn callback(config: State<'_, LocalConfig>, code: String, cookies: & CookieJar<'_>) -> Redirect {
    let redirect_url = format!("{}/oauth/callback", config.base_url);
    let url = format!("{}/oauth/token?client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}", 
        config.gitlab_oauth_base_url,
        config.gitlab_oauth_client_id,
        config.gitlab_oauth_client_secret,
        code,
        redirect_url); 
        
    let client = reqwest::Client::new();
    let res = match client.post(&url).send().await {
        Ok(res) => res,
        _ => return Redirect::to("/")
    };

    if let Ok(answer) = res.json::<AccessTokenAnswer>().await {
        let url = format!("{}/api/v4/user", config.gitlab_oauth_base_url);
        
        let client = reqwest::Client::new();
        let res = match client.get(&url).bearer_auth(answer.access_token).send().await {
            Ok(res) => res,
            _ => return Redirect::to("/")
        };

        if let Ok(user) = res.json::<UserInfo>().await {
            let cookie = Cookie::build("username", user.username)
            .http_only(true)
            .same_site(SameSite::Strict)
            .secure(true)
            .permanent()
            .finish();
            cookies.add_private(cookie);
        }
    }

    Redirect::to("/")
}