use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    //http::{header::ContextType, StatusCode}
};

use serde::{Serialize, Deserialize};
use derive_more::{Display};

#[derive(Deserialize, Serialize)]
pub struct ItemId { 
    item_global_id: u64,
}

#[get("/item/{item_global_id}")]
pub async fn get_item() -> Json<String> {
    return Json("hello world".to_string());
}
