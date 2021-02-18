use rocket::{Route, get, State, Response};
use rocket::response::status::BadRequest;
use sailfish::TemplateOnce;
use rocket_contrib::uuid::Uuid;
use tokio::sync::RwLockReadGuard;

use crate::common::{User, Room, DB};

fn respond<T: TemplateOnce>(t: T) -> Result<Response<'static>, BadRequest<&'static str>> {
    let rsp = t
        .render_once()
        .map_err(|_| BadRequest(Some("Bad request")))?;
    rocket::Response::build()
        .header(rocket::http::ContentType::HTML)
        .sized_body(rsp.len(), std::io::Cursor::new(rsp))
        .ok()
}

pub fn routes() -> Vec<Route> {
    rocket::routes![
        index,
        index_logged_in,
        login,
        watch,
        stream
    ]
}

#[derive(TemplateOnce)]
#[template(path = "index_logged_in.stpl")]
struct IndexLoggedInTemplate<'a> {
    user: &'a User,
    rooms: &'a RwLockReadGuard<'a, Vec<Room>>
}

#[get("/", rank = 1)]
async fn index_logged_in(user: User, db: State<'_, DB>) -> Result<Response<'static>, BadRequest<&'static str>> {
    respond(IndexLoggedInTemplate{
        user: &user,
        rooms: &db.rooms.read().await
    })
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct IndexTemplate{}

#[get("/", rank = 2)]
fn index() -> Result<Response<'static>, BadRequest<&'static str>> {
    respond(IndexTemplate{})
}

#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
struct LoginTemplate{}

#[get("/login")]
fn login() -> Result<Response<'static>, BadRequest<&'static str>> {
    respond(LoginTemplate{})
}

#[derive(TemplateOnce)]
#[template(path = "watch.stpl")]
struct WatchTemplate{
    uuid: Box<String>
}

#[get("/watch/<id>")]
fn watch(id: Uuid) -> Result<Response<'static>, BadRequest<&'static str>> {
    respond(WatchTemplate{
        uuid: Box::new(id.to_string())
    })
}

#[derive(TemplateOnce)]
#[template(path = "stream.stpl")]
struct StreamTemplate{
    uuid: Box<String>
}

#[get("/stream/<id>")]
async fn stream(user: User, id: Uuid, db: State<'_, DB>) -> Result<Response<'static>, BadRequest<&'static str>> {
    let rooms = db.rooms.read().await;
    let index = rooms.iter().position(|r| *r.uuid == id.to_string());
    let index = match index {
        Some(i) => i,
        _ => return Err(BadRequest(Some("Uuid not known")))
    };
    if rooms[index].owner == user.username {
        respond(StreamTemplate{
            uuid: Box::new(id.to_string())
        })
    } else {
        Err(BadRequest(Some("You are not the owner")))
    }
}