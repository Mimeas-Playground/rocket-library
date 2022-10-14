#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use rocket::{fs::{FileServer, relative}, serde::{Serialize, Deserialize}};

#[cfg(test)]
mod tests;
mod api;
mod data_store;

#[launch]
fn rocket() -> _{
   rocket::build()
    .mount("/", FileServer::from(relative!("web")))
    .mount("/api", routes![api::books, api::add_book])
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(crate="rocket::serde")]
pub struct Book {
    pub title: String
}
