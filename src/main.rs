use actix_web::{App, HttpServer, http::ContentEncoding, middleware, web};
#[macro_use] extern crate log;

mod handlers;
mod schemas;
mod config;

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
   std::env::set_var("RUST_LOG", "actix_web=debug");
   env_logger::init();
   let server = config::host();

   info!("Server starting {:?}",server);

   HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .configure(handlers::app_config)
            .service(web::resource("/health").route(web::get().to(handlers::get::health)))

   })
   .bind(server)?
   .run()
   .await
}
