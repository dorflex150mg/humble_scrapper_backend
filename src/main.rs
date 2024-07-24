use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)] //to/from json
struct User {
    name: String,
}

type SimDb = Arc<Mutex<HashMap<u64, User>>>;

#[actix_web::get("/hello/{id}")]
async fn hello(user_id: web::Path<u64>) -> impl Responder { //formats from endpoint into Responder
    format!("Hello, {user_id}!")
}



#[actix_web::get("/get_users")]
async fn get_users(db: web::Data<SimDb>) -> impl Responder { 
    let db = db.lock().unwrap();
    let users = db.clone().into_values().collect::<Vec<User>>();
    let str_users: Vec<String> = users.iter().map(|user| {
                                       user.name.clone()
                                    }).collect();
                                      
    format!("Enrolled users: \n{}", str_users.join("\n"))
}
        

#[derive(Serialize)]
struct CreateUserResponse {
    id: u64,
    name: String,
}

#[actix_web::post("/users")]
async fn create_user(user_data: web::Json<User>,
                     db: web::Data<SimDb> )-> impl Responder {
    let mut db = db.lock().unwrap();
    let id = match db.keys().max() {
        Some(n) => n + 1,
        None => 0,
    };
    let name = user_data.name.clone(); //Data can accessed like the regular struct
    db.insert(id, user_data.into_inner()); //into_inner deserializes json
    println!("Inserted user {} on id {}", name.clone(), id);
    HttpResponse::Created().json(
        CreateUserResponse {
            id,
            name,
        }
    )
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Running on {port}");

    let sim_db: SimDb = Arc::new(Mutex::new(HashMap::<u64, User>::new()));

    HttpServer::new(move || { 
        let app_data = web::Data::new(sim_db.clone()); //a struct that represents data
        App::new()
            .service(hello) // enrolls a function into the app
            .service(create_user)
            .service(get_users)
            .app_data(app_data) //enrolls data "type" into the app
    }) 
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
