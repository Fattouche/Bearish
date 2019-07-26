#![feature(async_await)]

extern crate futures;
extern crate hyper;
extern crate url;
extern crate tokio;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{service_fn, make_service_fn};

use std::collections::HashMap;
use url::form_urlencoded;
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| {
        async {
            Ok::<_, hyper::Error>(service_fn(price))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

fn response(code: u16) -> Response{
    let mut response = Response::default();
    let status = StatusCode::from_u16(code).unwrap()
    *response.status_mut() = status;
    response
}

async fn price(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("You need to GET a specific stock symbol ie /ACB")))
        }

        (&Method::GET, "/") => {
            let b = req.into_body().try_concat().await?;
            let params = form_urlencoded::parse(b.as_ref()).into_owned().collect::<HashMap<String, String>>();

            println!("PARAMS: {:?}",params);
            Ok(Response::new(Body::from("$8")))
        }

        // Return the 404 Not Found for other routes.
        _ => response(404)
    }
}