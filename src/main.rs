use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::web::{get, resource, scope};
use std::error::Error;
use std::net::Ipv4Addr;

mod api;
mod openapi;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .document(openapi::get_documentation())
            .service(resource("/ping").route(get().to(api::ping)))
            .service(scope("/places").service(resource("").route(get().to(api::get_places))))
            .build_with(
                &openapi::get_json_path(),
                openapi::get_swagger_configuration(),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
