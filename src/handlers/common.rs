use std::pin::Pin;
use actix_http::{client::SendRequestError, encoding::Decoder};
use actix_http::Payload;
use actix_web::{
    HttpResponse,
    http::StatusCode,
    web::Bytes
};
use actix_web::client::{
    Client,
    Connector,
    ClientResponse,
    PayloadError,
};

use futures::Stream;
use openssl::ssl::{SslConnector, SslMethod};


pub async fn https_client() -> Client {
    let builder = SslConnector::builder(SslMethod::tls());
    match builder {
        Ok(context) => {
            Client::builder()
            .connector(Connector::new().ssl(context.build()).finish())
            .finish()
        },
        Err(err) => {
            println!("SSL issue: {:?}", err);
            error!("SSL issue: {:?}", err);
            panic!();
        }
    }
}

pub async fn no_content_handler(responses: Vec<Result<ClientResponse<Decoder<Payload<Pin<Box<dyn Stream<Item = Result<Bytes, PayloadError>>>>>>>, SendRequestError>>) -> HttpResponse {
    let status_codes = responses
    .iter()
    .map(|x| 
        match x {
            Err(err) => {
                error!("error on no_content_handler: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Ok(row) => row.status()
        }
    );

    for status in status_codes.into_iter() {
        if status == StatusCode::BAD_REQUEST {
            return  HttpResponse::BadRequest().finish();
        } 
        if status == StatusCode::UNPROCESSABLE_ENTITY {
            return  HttpResponse::UnprocessableEntity().finish();
        }
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            return  HttpResponse::InternalServerError().finish();
        }
    }
    
    HttpResponse::NoContent().finish()
}
