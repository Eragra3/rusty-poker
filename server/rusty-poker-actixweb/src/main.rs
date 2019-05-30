use actix_web::middleware::Logger;
use actix_web::web::{resource, Data, Json, Path};
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

use serde::{Deserialize};

use env_logger;

use rusty_poker::database::{MockDatabase, PokerDatabase};
use rusty_poker::poker::VoteValue;

use std::sync::{Mutex, Arc};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn hello_get() -> impl Responder {
    "Hello, world!"
}

fn version_get() -> impl Responder {
    VERSION
}

#[derive(Deserialize)]
struct SetVote {
    value: VoteValue,
}

fn vote_set(params: Path<(i32, i32)>, body: Json<SetVote>, data: Data<Arc<Mutex<MockDatabase>>>) -> HttpResponse {
    let vote = data
        .lock()
        .unwrap()
        .set_vote_value(params.0, params.1, body.value);
    match vote {
        Ok(vote) => HttpResponse::Ok().json(vote),
        Err(msg) => HttpResponse::UnprocessableEntity().body(msg),
    }
}

fn voting_create(data: Data<Arc<Mutex<MockDatabase>>>) -> HttpResponse {
    let voting = data
        .lock()
        .unwrap()
        .create_voting();
    match voting {
        Ok(voting) => HttpResponse::Ok().json(voting),
        Err(_msg) => HttpResponse::InternalServerError().finish(),
    }
}

fn voting_get(params: Path<i32>, data: Data<Arc<Mutex<MockDatabase>>>) -> HttpResponse {
    let voting_id = params.into_inner();
    let votes = data.lock().unwrap().get_voting(voting_id);
    match votes {
        Some(votes) => HttpResponse::Ok().json(votes),
        _ => HttpResponse::NotFound().finish(),
    }
}

fn voting_get_votes(params: Path<i32>, data: Data<Arc<Mutex<MockDatabase>>>) -> HttpResponse {
    let voting_id = params.into_inner();
    let votes = data.lock().unwrap().get_votes(voting_id);
    match votes {
        Some(votes) => HttpResponse::Ok().json(votes),
        _ => HttpResponse::NotFound().finish(),
    }
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    let data = Arc::new(Mutex::new(MockDatabase::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(data.clone())
            .service(resource("/hello").route(web::get().to(hello_get)))
            .service(resource("/version").route(web::get().to(version_get)))
            .service(resource("/voting/{id}").route(web::get().to(voting_get)))
            .service(resource("/voting/{id}/votes").route(web::get().to(voting_get_votes)))
            .service(resource("/voting").route(web::post().to(voting_create)))
            .service(resource("/voting/{voting_id}/vote/{vote_id}/set_vote").route(web::post().to(vote_set)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
}
