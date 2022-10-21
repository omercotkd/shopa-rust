#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod api;
mod db;
mod helpers;
mod models;
mod services;

use api::routes::{lists, login, register};

#[launch]
pub async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::init())
        .mount(
            "/api/list",
            routes![
                lists::get_list,
                lists::add_item_to_list,
                lists::delete_items_from_list,
                lists::create_list
            ],
        )
        .mount(
            "/api/login",
            routes![login::login_with_email, login::login_with_phone],
        )
        .mount("/api/register", routes![register::register_new_user])
}
