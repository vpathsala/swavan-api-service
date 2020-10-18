
use actix_web::{HttpResponse, web};
use futures::future::join_all;
use crate::schemas;
use crate::config;

pub async fn put(state: web::Data<config::Configuration>, mocks: web::Json<Vec<schemas::post::Mock<Vec<schemas::common::Header>>>>) -> HttpResponse {
    let client = super::common::https_client().await;
    let items = mocks
    .iter()
    .filter(|x| x.id != None)
    .map(|x| {
        client
        .put(format!("{}/{}", state.api_server, x.id.as_ref().unwrap()))
        .send_json(&schemas::post::mock_to_post(x))
    });
    
    let responses = join_all(items).await;
    super::common::no_content_handler(responses).await
}
