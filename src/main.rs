use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use env_logger::Logger;


mod api;
mod repository;
mod model;

use crate::repository::db::DbHandle;

use api::agent::{
    create_agent,
    get_agents,
};


#[derive(Clone, Serialize, Deserialize)] //to/from json
struct Agent {
    name: String,
}

type SimDb = Arc<Mutex<DbHandle>>;

//#[actix_web::get("/hello/{id}")]
//async fn hello(user_id: web::Path<u64>) -> impl Responder { //formats from endpoint into Responder
//    format!("Hello, {user_id}!")
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let port = 8080;
    let ip = "127.0.0.1";
    println!("Running on {port}");

    let sim_db: SimDb = Arc::new(Mutex::new(DbHandle::new(String::from("scraper.db")).unwrap()));

    HttpServer::new(move || { 
        //let logger = Logger::default();
        let app_data = web::Data::new(sim_db.clone()); //a struct that represents data
        App::new()
            //.wrap(logger)
            //.service(hello) // enrolls a function into the app
            .service(create_agent)
            .service(get_agents)
            .app_data(app_data) //enrolls data "type" into the app
    }) 
        .bind((ip, port))?
        .run()
        .await
}
