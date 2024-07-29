use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use env_logger::Logger;


mod api;
mod repository;
mod model;

use crate::repository::db::DbHandle;
use crate::api::item::post_item;
use crate::api::item::get_item;

use api::agent::{
    create_agent,
    get_agents,
};


type Db = Arc<Mutex<DbHandle>>;

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

    let db: Db = Arc::new(Mutex::new(DbHandle::new(String::from("scraper.db")).unwrap()));
    HttpServer::new(move || { 
        //let logger = Logger::default();
        let db_handle = web::Data::new(db.clone()); //a struct that represents data
        App::new()
            //.wrap(logger)
            //.service(hello) // enrolls a function into the app
            .service(post_item)
            .service(get_item)
            .service(create_agent)
            .service(get_agents)
            .app_data(db_handle) //enrolls data "type" into the app
    }) 
        .bind((ip, port))?
        .run()
        .await
}
