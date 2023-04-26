#![allow(unused)]

use axum::response::Html;
use axum::response::IntoResponse;
use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
  let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );  


// region: --- Start Server
  let addr = SocketAddr::from(([127,0,0,1], 8080));
  println!("->> LISTENING on {addr}\n");
  axum::Server::bind(&addr)
      .serve(routes_hello.into_make_service())
      .await
      .unwrap();

  // endregion: --- Start Server
}

//  region: --- Handler Hello
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");

// endregion: --- Handler Hello
}



