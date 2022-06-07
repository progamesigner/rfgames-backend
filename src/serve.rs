use {
    super::{id, Form},
    actix_rt::System,
    actix_web::{http::Uri, middleware::Logger, web, App, Error, HttpResponse, HttpServer},
    awc::Client,
    serde::de::DeserializeOwned,
    std::{env, io},
};

async fn handle<T>(body: web::Json<T>) -> Result<HttpResponse, Error>
where
    T: DeserializeOwned + Form + 'static,
{
    let webhook = T::webhook().parse::<Uri>().expect("Invalid webhook URL.");

    let client = Client::default();

    let response = client
        .post(webhook)
        .send_json(&body.into_payload(&id::next(&T::prefix())))
        .await;

    match response {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(error) => {
            println!("{}", error.to_string());
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

async fn healthz() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

async fn statusz() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn serve<T>() -> io::Result<()>
where
    T: DeserializeOwned + Form + 'static,
{
    let addr = env::var("SERVER_LISTEN_ADDR").unwrap_or("127.0.0.1".into());
    let port = env::var("SERVER_LISTEN_PORT").unwrap_or("3000".into());

    System::new().block_on(async move {
        HttpServer::new(move || {
            App::new()
                .route("/healthz", web::to(healthz))
                .route("/statusz", web::to(statusz))
                .route("*", web::to(handle::<T>))
                .wrap(Logger::default())
        })
        .bind(format!("{}:{}", addr, port))?
        .run()
        .await
    })
}
