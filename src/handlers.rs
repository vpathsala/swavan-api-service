use actix_web::HttpResponse;
use actix_web::client::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};
use crate::schema;

async fn https_client() -> Client {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();
    return client
}

pub async fn get_raw() -> HttpResponse {
    let response = https_client()
        .await
        .get("https://api.mocky.io/api/mock/24396484-da5c-4cc1-8b01-40963743d27a")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();
    
    let obj = serde_json::from_slice::<schema::MockPayloadModal>(&response).unwrap();
    HttpResponse::Ok().json(obj)
    // HttpResponse::Ok().body(response)
}


// pub async fn remove() {
//     let response = https_client()
//         .await
//         .delete("https://api.swavan.io/mock/v1/f4ed5fdd-5fbb-411e-996f-c6c8ea784626")
//         .send()
//         .await
//         .unwrap()
//         .body()
//         .await
//         .unwrap();
//     println!("Response: {:?}", response);
// }