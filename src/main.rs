use common::DB;
use rocket_contrib::serve::StaticFiles;
use serde::Deserialize;
use rocket::fairing::AdHoc;

mod front;
mod api;
mod common;

#[derive(Deserialize)]
struct LocalConfig{
    login_token: String,
}

#[rocket::launch]
async fn rocket() -> _{
    let db = DB::new();

    let mut static_dir = std::env::current_dir().unwrap();
    static_dir.push("static");
    rocket::ignite()
        .manage(db)
        .attach(AdHoc::config::<LocalConfig>())
        .mount("/", front::routes())
        .mount("/static", StaticFiles::from(static_dir))
        .mount("/api", api::routes())
}
