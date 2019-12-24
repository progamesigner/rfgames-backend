use {
    super::{id, Form},
    actix_web::{
        client::Client, http::Uri, middleware::Logger, web, App, Error, HttpResponse, HttpServer,
    },
    futures::future::Future,
    serde::de::DeserializeOwned,
    std::{env, io},
};

fn handle<T>(body: web::Json<T>) -> impl Future<Item = HttpResponse, Error = Error>
where
    T: DeserializeOwned + Form + 'static,
{
    let webhook = T::webhook().parse::<Uri>().expect("Invalid webhook URL.");

    Client::default()
        .post(webhook)
        .send_json(&body.into_payload(&id::next(&T::prefix())))
        .then(|response| match response {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(error) => {
                println!("{}", error.to_string());
                HttpResponse::InternalServerError().finish()
            }
        })
}

pub fn serve<T>() -> io::Result<()>
where
    T: DeserializeOwned + Form + 'static,
{
    let addr = env::var("SERVER_LISTEN_ADDR").unwrap_or("127.0.0.1".into());
    let port = env::var("SERVER_LISTEN_PORT").unwrap_or("3000".into());

    HttpServer::new(move || {
        App::new()
            .route("*", web::to_async(handle::<T>))
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
}
