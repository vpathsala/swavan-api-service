use actix_web::{ middleware, App, HttpServer,  http::ContentEncoding};
#[macro_use] extern crate log;

mod handlers;
mod schemas;
mod config;

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
   std::env::set_var("RUST_LOG", "actix_web=debug");
   env_logger::init();

   HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .configure(handlers::app_config)
   })
   .bind("0.0.0.0:8080")?
   .run()
   .await
}
