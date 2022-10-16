#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod api;
mod db;
mod helpers;
mod services;
mod models;

use api::routes::lists;

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
        .mount("/api/login", routes![])
}
