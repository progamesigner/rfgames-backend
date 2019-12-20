use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use http::Request;
use now_lambda::{error::NowError, Body, Handler, IntoResponse};
use std::{cell::RefCell, env};

pub fn serve<R, B, E>(handler: impl Handler<R, B, E> + Clone + Send + 'static)
where
    B: From<Body>,
    E: Into<NowError>,
    R: IntoResponse,
{
    let addr = format!(
        "{}:{}",
        env::var("SERVER_BIND_ADDR").unwrap_or("127.0.0.1".into()),
        env::var("SERVER_LISTEN_PORT").unwrap_or("3000".into())
    );

    HttpServer::new(move || {
        let f = RefCell::new(handler.clone());

        let process = move |payload: String| {
            let request = Request::builder()
                .method("POST")
                .body(Body::from(payload))
                .unwrap()
                .map(|b| b.into());

            match f.borrow_mut().run(request) {
                Ok(_) => HttpResponse::NoContent().finish(),
                Err(error) => {
                    println!("Error: {}", error.into());
                    HttpResponse::BadRequest().finish()
                },
            }
        };

        App::new()
            .route("*", web::to(process))
            .wrap(Logger::default())
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}
