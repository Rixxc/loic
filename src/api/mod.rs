use rocket::{Route, State, post};
use rocket::response::Redirect;
use rocket::response::status::BadRequest;
use rocket::http::{Cookie, CookieJar, Status, SameSite};
use rocket::request::{FromForm, Form};
use uuid::Uuid;

use crate::common::{User, DB, Room, LocalConfig};

pub fn routes() -> Vec<Route> {
    rocket::routes![
        login,
        logout,
        new,
        delete
    ]
}

#[derive(FromForm)]
struct LoginForm {
    username: String,
    token: String
}

#[post("/login", data="<login>")]
fn login(login: Form<LoginForm>, cookies: &CookieJar, config: State<LocalConfig>) -> Result<Redirect, Status> {
    if login.token == config.login_token {
        let cookie = Cookie::build("username", login.username.clone())
            .http_only(true)
            .same_site(SameSite::Strict)
            .secure(true)
            .permanent()
            .finish();
        cookies.add_private(cookie);
        Ok(Redirect::to("/"))
    } else {
        Err(Status::Unauthorized)
    }
}

#[post("/logout")]
fn logout(cookies: &CookieJar) -> Redirect{
    cookies.remove_private(Cookie::named("username"));
    Redirect::to("/")
}

#[derive(FromForm)]
struct NewForm {
    name: String
}

#[post("/new", data="<room>")]
async fn new(user: User, db: State<'_, DB>, room: Form<NewForm>) -> Redirect {
    let uuid = Uuid::new_v4();
    let mut rooms = db.rooms.write().await;
    let mut name = room.name.clone();

    if name.is_empty() {
        name = "unnamed room".to_string();
    }

    rooms.push(Room{uuid: Box::new(uuid.to_string()), name: Box::new(name), owner: user.username});
    Redirect::to(format!("/stream/{}", uuid))
}

#[derive(FromForm)]
struct DeleteForm {
    uuid: String
}

#[post("/delete", data="<room>")]
async fn delete(user: User, db: State<'_, DB>, room: Form<DeleteForm>) -> Result<Redirect, BadRequest<&'static str>> {
    let mut rooms = db.rooms.write().await;
    let index = rooms.iter().position(|r| *r.uuid == room.uuid);
    let index = match index {
        Some(i) => i,
        _ => return Err(BadRequest(Some("Uuid not known")))
    };
    if rooms[index].owner == user.username {
        rooms.remove(index);
        Ok(Redirect::to("/"))
    } else {
        Err(BadRequest(Some("Not your room")))
    }
}