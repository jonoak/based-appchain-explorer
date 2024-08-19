extern crate actix_web;
extern crate ethers;
extern crate serde;
extern crate tokio;
use actix_web::{web, App, HttpServer, Responder};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct L2 {
    id: String,
    name: String,
    config: String,
}

struct AppState {
    l2s: Mutex<Vec<L2>>,
}

async fn get_l2s(data: web::Data<AppState>) -> impl Responder {
    let l2s = data.l2s.lock().unwrap();
    web::Json(&*l2s)
}

async fn create_l2(l2_info: web::Json<L2>, data: web::Data<AppState>) -> impl Responder {
    let mut l2s = data.l2s.lock().unwrap();
    l2s.push(l2_info.into_inner());
    web::Json("L2 Created")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        l2s: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/api/v1/l2s", web::get().to(get_l2s))
            .route("/api/v1/l2s", web::post().to(create_l2))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
