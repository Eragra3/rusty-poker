#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;

mod database;
mod poker;

use database::MockDatabase;
use poker::{Vote, VoteValue, Voting};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[get("/")]
fn get_hello() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn get_version() -> &'static str {
    VERSION
}

#[get("/<voting_id>")]
fn get_votes(voting_id: i32) -> Json<Vec<Vote>> {
    let db = MockDatabase::new();
    let votes = db.get_votes(voting_id);
    Json(match votes {
        Some(votes) => votes.clone(),
        _ => Vec::new(),
    })
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![get_hello])
        .mount("/version", routes![get_version])
        .mount("/votes", routes![get_votes])
        .launch();
}
