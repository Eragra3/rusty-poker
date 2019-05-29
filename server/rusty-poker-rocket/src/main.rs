#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};
use rocket_contrib::json::Json;

use rusty_poker::database::MockDatabase;
use rusty_poker::poker::{Vote, VoteValue, Voting};


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
fn get_votes(voting_id: i32) -> Option<Json<Vec<Vote>>> {
    let db = MockDatabase::new();
    let votes = db.get_votes(voting_id);
    match votes {
        Some(votes) => Some(Json(votes.clone())),
        _ => None
    }
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![get_hello])
        .mount("/version", routes![get_version])
        .mount("/votes", routes![get_votes])
        .launch();
}
