use actix_web::{HttpResponse, web};
use futures::future::join_all;
use crate::schemas;
use crate::config;

pub async fn delete(state: web::Data<config::Configuration>, delete_items: web::Json<Vec<schemas::delete::Mock>>) -> HttpResponse {
    let client = super::common::https_client().await;
    let items = delete_items
    .iter()
    .map(|x| client.delete(format!("{}/{}", state.api_server, &x.id)).send_json(x));
    
    let responses = join_all(items).await;

    super::common::no_content_handler(responses).await
}
