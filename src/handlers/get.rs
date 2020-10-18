use actix_http::http::StatusCode;
use actix_web::{HttpResponse, web};
use crate::schemas;
use crate::config;

pub async fn get(state: web::Data<config::Configuration>, web::Path(id): web::Path<String>) -> HttpResponse {
    //let base_url = "https://api.mocky.io/api/mock/";
    let url = format!("{}/{}", state.api_server, id);
    let response = super::common::https_client()
        .await
        .get(url)
        .send()
        .await;
    
    match response {
        Err(err) => {
            error!("Send request error: {}", err);
            HttpResponse::InternalServerError().finish()
        },
        Ok(mut row) => {
            match row.status() {
                StatusCode::OK => {
                    let body = row
                    .body()
                    .await;
    
                    match body {
                        Err(err) => {
                            error!("Response body error: {}", err);
                            HttpResponse::InternalServerError().finish()
                        },
                        Ok(data) => {
                            info!("Mock data found!!!");
                            match serde_json::from_slice::<schemas::get::Mock>(&data) {
                                Ok(obj) => HttpResponse::Ok().json(obj),
                                Err(err) => {
                                    error!("Parsing error: {}", err);
                                    HttpResponse::InternalServerError().finish()
                                }
                            }
                        }
                    }    
                },
                StatusCode::NOT_FOUND =>  {
                    error!("Data not found");
                    HttpResponse::NotFound().finish()
                },
                _ => {
                    error!("Unknown request type received {}", row.status());
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
