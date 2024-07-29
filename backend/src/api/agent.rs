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
use thiserror::Error;
use actix_web::http::header::ContentType;

use std::sync::{Arc, Mutex};
use actix_web::{};
use serde::{Serialize, Deserialize};

use crate::repository::db::DbHandle;
use crate::repository::db::EmptyTableError;
use crate::repository::db::QuerryError;
use crate::model::agent::Agent;


#[derive(Debug, Error, derive_more::Display)]
pub enum DbAgentError {
    AgentNotFound(QuerryError),
    PushFailed(QuerryError),
}

impl ResponseError for DbAgentError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }
}


#[derive(Serialize)]
struct CreateAgentResponse {
    id: String,
}

#[derive(Serialize)]
struct GetAgentsResponse { 
    agents_str: Vec<String>,
}

#[actix_web::get("/get_agents")]
async fn get_agents(db: web::Data<Arc<Mutex<DbHandle>>>) -> Result<Json<GetAgentsResponse>, DbAgentError>{ 
    let db = db.lock().unwrap();
    match db.get_agents() {
        Ok(a) => { 
            let agents_str = a.iter().map(|agent| {
                agent.to_string()
            }).collect();
            Ok(Json(
                GetAgentsResponse {
                    agents_str,
                }
            ))
        },
        Err(e) => Err(DbAgentError::AgentNotFound(e)),
        //Err(QuerryError::RusqliteError(e)) => panic!("Failed with {}", e),
    }
}

#[actix_web::post("/create_agent")]
async fn create_agent(db: web::Data<Arc<Mutex<DbHandle>>>)-> Result<HttpResponse, DbAgentError> {
    let mut db = db.lock().unwrap();
    let agent = Agent::new();
    let id = agent.id.clone();
    match db.push_agent(agent.id) {
        Ok(id) => {
            println!("Inserted agent {}", id);
            Ok(HttpResponse::Created().json(
                CreateAgentResponse {
                    id,
                }
            ))
        },
        Err(e) => Err(DbAgentError::PushFailed(QuerryError::RusqliteError(e))),
    }
}

