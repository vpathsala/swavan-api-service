use actix_http::http::StatusCode;
use actix_web::{HttpResponse, web};
use futures::future::join_all;
use crate::schemas;
use crate::config;

pub async fn post(
    state: web::Data<config::Configuration>,
    mocks: web::Json<Vec<schemas::post::Mock<Vec<schemas::common::Header>>>>) -> HttpResponse {
    let client = super::common::https_client().await;
    //let url = "https://api.mocky.io/api/mock";
    let items = mocks
    .iter()
    .map(|x| {
        client
        .post(&state.api_server)
        .send_json(&schemas::post::mock_to_post(x))
    });
    
    let responses = join_all(items).await;
    let mut post_responses: Vec<schemas::post::Response> = Vec::new();
    for res in responses {
        let row = match res {
            Err(err) => {
                error!("{:?}", err);
                default_response()
            },
            Ok(mut data) => match data.status() {
                StatusCode::OK | StatusCode:: CREATED => {
                    let data = data
                        .body()
                        .await;
                    
                    match data {
                            Err(err) => {
                                error!("Response body error: {}", err);
                                default_response()
                            },
                            Ok(data) => {
                                info!("Mock data found!!!");
                                match serde_json::from_slice::<schemas::post::Response>(&data) {
                                    Ok(obj) => obj,
                                    Err(err) => {
                                        error!("Parsing error: {}", err);
                                        default_response()
                                    }
                                }
                            }
                    }
                },
                _ => {
                    default_response()
                }
            }
        };
        post_responses.push(row);
    }
    HttpResponse::Created().json(post_responses)
}

fn default_response() -> schemas::post::Response {
    schemas::post::Response{ id: None, key: None, link: None }
}
