use actix_cors::Cors;
use actix_web::{
    client::Client, http::header, middleware::Logger, web, web::Data, web::Json, App, Error,
    FromRequest, HttpResponse, HttpServer,
};
use futures::future::Future;
use log::error;
use serde::de::DeserializeOwned;
use std::env;

struct Payload {
    webhook: String,
}

pub fn start<T>()
where
    T: DeserializeOwned + super::FromForm + 'static,
{
    let addr = format!(
        "{}:{}",
        env::var("SERVER_BIND_ADDR").unwrap_or("127.0.0.1".into()),
        env::var("SERVER_LISTEN_PORT").unwrap_or("3000".into())
    );

    let origin = env::var("SERVER_CORS_ORIGIN").unwrap_or("*".into());

    let webhook = env::var("DISCORD_WEBHOOK_URL").unwrap();

    HttpServer::new(move || {
        App::new()
            .data(Payload {
                webhook: webhook.clone(),
            })
            .service(
                web::resource("/")
                    .data(Json::<T>::configure(|config| {
                        config.error_handler(|error, _req| {
                            error!("{}", error.to_string());
                            HttpResponse::BadRequest().finish().into()
                        })
                    }))
                    .route(web::post().to_async(handle::<T>)),
            )
            .wrap(
                Cors::new()
                    .allowed_origin(&origin)
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .wrap(Logger::default())
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}

fn handle<T>(data: Data<Payload>, form: Json<T>) -> impl Future<Item = HttpResponse, Error = Error>
where
    T: DeserializeOwned + super::FromForm + 'static,
{
    Client::default()
        .post(&data.webhook)
        .send_json(&T::into_body(&form, &super::id::next(&T::prefix())))
        .then(|response| match response {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(error) => {
                error!("{}", error.to_string());
                HttpResponse::InternalServerError().finish()
            }
        })
}
