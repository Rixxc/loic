use rocket::request::{Request, FromRequest, Outcome};
use tokio::sync::RwLock;

pub struct DB{
    pub rooms: RwLock<Vec<Room>>
}

impl DB {
    pub fn new() -> Self{
        Self {
            rooms: RwLock::new(Vec::new())
        }
    }
}

pub struct Room{
    pub uuid: Box<String>,
    pub name: Box<String>,
    pub owner: Box<String>
}

#[derive(Clone)]
pub struct User{
    pub username: Box<String>
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<User, ()> {
        let user = request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|username| User{username: Box::new(username)});

        match user {
            Some(u) => Outcome::Success(u),
            None => Outcome::Forward(())
        }

    }
}
