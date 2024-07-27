//use actix_web::{
//    get,
//    post,
//    put,
//    error::ResponseError,
//    web::Path,
//    web::Json,
//    web::Data,
//    HttpResponse,
//};
    //http::{header::ContextType, StatusCode}
use std::sync::{Arc, Mutex};
use actix_web::http::header::ContentType;
use actix_web::get;
use actix_web::web;
use actix_web::post;
use actix_web::web::Json;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use derive_more::{Display};

use crate::repository::db::QuerryError;
use crate::repository::db::DbHandle;
use crate::api::item::DbItemError::ItemNotFound;
use crate::model::item::Item;

#[derive(Debug, Error, derive_more::Display)]
pub enum DbItemError {
    ItemNotFound(QuerryError),
    PushFailed(QuerryError),
}

impl ResponseError for DbItemError {
    fn error_response(&self) -> HttpResponse { 
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ItemId { 
    item_global_id: u64,
}

#[derive(Deserialize)]
pub struct SubmitItem {
    name: String,
    price: f64,
}

#[derive(Deserialize)]
pub struct GetItem{
    id: String
}


#[get("/item")]
pub async fn get_item(db: web::Data<Arc<Mutex<DbHandle>>>, request: Json<GetItem>) -> Result<Json<Item>, DbItemError> {
    println!("item: {}", request.id.clone());
    match db.lock().unwrap().get_item(request.id.clone()) { 
        Ok(i) => {
            Ok(Json(
                Item {
                    id: i.id,
                    name: i.name,
                    price: i.price,
                }
            ))
        },
        Err(e) => Err(DbItemError::ItemNotFound(e)),
        //Err(ItemNotFound(QuerryError::RusqliteError(e))) => Err("Failed with Sqlite error: {}", e),  
    }
}

#[post("/add_item")]
pub async fn post_item(db: web::Data<Arc<Mutex<DbHandle>>>, 
        request: Json<SubmitItem>,
        //name: web::Path<String>, 
        ) -> Result<Json<String>, DbItemError> {
    let item = Item::new(request.name.clone(), request.price);
    match db.lock().unwrap().push_item(item.id, item.name, item.price) {
        Ok(id) => Ok(Json(id)),
        Err(e) => Err(DbItemError::PushFailed(QuerryError::RusqliteError(e))),
    }
}

    

