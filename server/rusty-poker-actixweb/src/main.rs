use actix_web::middleware::Logger;
use actix_web::web::{resource, Data, Json, Path};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use serde::Deserialize;

use uuid::Uuid;

use env_logger;

use rusty_poker::database::{MockDatabase, PokerDatabase};
use rusty_poker::poker::VoteValue;

use std::sync::{Arc, Mutex};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn hello_get() -> impl Responder {
    "Hello, world!"
}

fn version_get() -> impl Responder {
    VERSION
}

#[derive(Deserialize)]
struct SetVoteCommand {
    voter_uuid: Uuid,
    value: VoteValue,
}

fn vote_set(
    params: Path<Uuid>,
    body: Json<SetVoteCommand>,
    data: Data<Arc<Mutex<MockDatabase>>>,
) -> HttpResponse {
    let voting_uuid = params.into_inner();
    let vote = data
        .lock()
        .unwrap()
        .set_vote_value(voting_uuid, body.voter_uuid, body.value);

    match vote {
        Ok(vote) => HttpResponse::Ok().json(vote),
        Err(msg) => HttpResponse::UnprocessableEntity().body(msg),
    }
}

#[derive(Deserialize)]
struct JoinVotingCommand {
    user_uuid: Uuid,
}

fn voting_join(
    params: Path<Uuid>,
    body: Json<JoinVotingCommand>,
    data: Data<Arc<Mutex<MockDatabase>>>,
) -> HttpResponse {
    let voting_uuid = params.into_inner();
    let command = body.into_inner();

    let user = data
        .lock()
        .unwrap()
        .join_voting(command.user_uuid, voting_uuid);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_msg) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct CreateVotingCommand {
    host_uuid: Uuid,
    title: String,
}

fn voting_create(
    body: Json<CreateVotingCommand>,
    data: Data<Arc<Mutex<MockDatabase>>>,
) -> HttpResponse {
    let command = body.into_inner();
    let voting = data
        .lock()
        .unwrap()
        .create_voting(command.host_uuid, command.title);

    match voting {
        Ok(voting) => HttpResponse::Ok().json(voting),
        Err(_msg) => HttpResponse::InternalServerError().finish(),
    }
}

fn voting_get(params: Path<Uuid>, data: Data<Arc<Mutex<MockDatabase>>>) -> HttpResponse {
    let voting_id = params.into_inner();
    let votes = data.lock().unwrap().get_voting(voting_id);

    match votes {
        Some(votes) => HttpResponse::Ok().json(votes),
        _ => HttpResponse::NotFound().finish(),
    }
}

fn voting_get_votes(params: Path<Uuid>, data: Data<Arc<Mutex<MockDatabase>>>) -> HttpResponse {
    let voting_id = params.into_inner();
    let votes = data.lock().unwrap().get_votes(voting_id);

    match votes {
        Some(votes) => HttpResponse::Ok().json(votes),
        _ => HttpResponse::NotFound().finish(),
    }
}

#[derive(Deserialize)]
struct CreateUserCommand {
    name: String,
}

fn user_create(
    body: Json<CreateUserCommand>,
    data: Data<Arc<Mutex<MockDatabase>>>,
) -> HttpResponse {
    let command = body.into_inner();

    let user = data.lock().unwrap().create_user(command.name);
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_msg) => HttpResponse::InternalServerError().finish(),
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
            .service(resource("/voting").route(web::post().to(voting_create)))
            .service(resource("/voting/{id}").route(web::get().to(voting_get)))
            .service(resource("/voting/{id}/votes").route(web::get().to(voting_get_votes)))
            .service(resource("/voting/{id}/join").route(web::post().to(voting_join)))
            .service(resource("/voting/{id}/vote").route(web::post().to(vote_set)))
            .service(resource("/user").route(web::post().to(user_create)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
}
