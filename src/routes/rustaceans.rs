use rocket::serde::json::Json;
use rocket::response::status;
use crate::models::{NewRustacean, Rustacean};

#[rocket::get("/rustaceans")]
pub fn get_rustaceans() {
    // return state content
}

#[rocket::get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) {

}

#[rocket::post("/rustaceans", format="json", data="<new_rustacean>")]
pub fn create_rustacean(new_rustacean: Json<NewRustacean>) {

}

#[rocket::put("/rustaceans/<id>", format="json", data="<rustacean>")]
pub fn update_rustacean(id: i32, rustacean: Json<Rustacean>) {

}

#[rocket::delete("/rustaceans/<id>")]
pub fn delete_rustacean(id: i32) {

}