use actix_web::{App, HttpServer, Responder, HttpResponse, http};
use actix_web::web::{Path, resource};

use rusty_poker::database::{MockDatabase};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn hello_get() -> &'static str {
    "Hello, world!"
}

fn version_get() -> &'static str {
    VERSION
}

fn votes_get(params: Path<i32>) -> HttpResponse {
    let voting_id = params.into_inner();
    let db = MockDatabase::new();
    let votes = db.get_votes(voting_id);
    match votes {
        Some(votes) => HttpResponse::Ok().json(votes.clone()),
        _ => HttpResponse::NotFound().finish()
    }
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(resource("/hello").to(hello_get))
            .service(resource("/version").to(version_get))
            .service(resource("/votes/{voting_id}").to(votes_get))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
}
