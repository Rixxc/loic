use common::{DB, LocalConfig};
use rocket_contrib::serve::StaticFiles;
use rocket::fairing::AdHoc;

mod front;
mod api;
mod common;
mod oauth;

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
        .mount("/oauth", oauth::routes())
}
