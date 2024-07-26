use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    Responder,
    web,
    //http::{header::ContextType, StatusCode}
};

use std::sync::{Arc, Mutex};
use actix_web::{};
use serde::{Serialize, Deserialize};

use crate::Agent;
use crate::repository::db::DbHandle;

#[derive(Serialize)]
struct CreateAgentResponse {
    id: u64,
}

#[actix_web::get("/get_agents")]
async fn get_agents(db: web::Data<DbHandle>) -> impl Responder { 
    //let db = db.lock().unwrap();
    //let agents = db.clone().into_values().collect::<Vec<Agent>>();
    //let str_agents: Vec<String> = agents.iter().map(|agent| {
    //                                   agent.id.to_string()
    //                                }).collect();
    //                                  
    //format!("Enrolled agents: \n{}", str_agents.join("\n"))
    format!("...")
}



#[actix_web::post("/agents")]
async fn create_agent(agent_data: web::Json<Agent>,
                     db: web::Data<Arc<Mutex<DbHandle>>> )-> impl Responder {
    let id = 1;
    //let mut db = db.lock().unwrap();
    //let id = match db.keys().max() {
    //    Some(n) => n + 1,
    //    None => 0,
    //};
    //let name = agent_data.name.clone(); //Data can accessed like the regular struct
    //db.insert(id, agent_data.into_inner()); //into_inner deserializes json
    println!("Inserted agent {}", id);
    HttpResponse::Created().json(
        CreateAgentResponse {
            id,
        }
    )
}

