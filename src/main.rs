use actix_web::{ middleware, web, App, HttpServer,  http::ContentEncoding};

mod handlers;
mod schema;


#[actix_rt::main]
async fn main()-> std::io::Result<()> {
   std::env::set_var("RUST_LOG", "actix_web=info");
   env_logger::init();

   HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .service(web::resource("/").route(web::get().to(handlers::get_raw)))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await

}

