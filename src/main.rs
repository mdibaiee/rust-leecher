#[macro_use]
extern crate hyper;
extern crate leecher;

use hyper::Server;
use hyper::server::{Request, Response};

use leecher::url_type::{url_type, UrlType};
use leecher::download;
use hyper::status::StatusCode;

fn handler(request: Request, mut response: Response) {
  let raw = request.uri.to_string();
  let base = "http://0.0.0.0:8080".to_string();
  let full = base + &raw;

  let query = leecher::query::parse(&full);

  if query.url.is_empty() {
    response.send(b"Invalid url parameter").unwrap();

    return;
  }

  let utype = url_type(&query.url);

  println!("[utype] request type: {}, url: {}", utype, query.url);

  let result = match utype {
    UrlType::Direct => download::direct(&query),
    UrlType::Youtube => download::youtube(&query),
    _ => Err("Unknown url type".to_string())
  };

  match result {
    Ok(path) => {
      let p = &path.to_str().unwrap().as_bytes();

      println!("[result] wrote to: {}", path.to_str().unwrap());

      response.send(p).unwrap();
    },
    Err(e) => {
      let message: String = "Error: ".to_string() + &e;
      println!("[result] error: {}", message);

      {
        let mut status = response.status_mut();
        *status = StatusCode::InternalServerError;
      }
      response.send(&message.into_bytes()).unwrap();
    }
  };
}

fn main() {
  Server::http("0.0.0.0:8080").unwrap().handle(handler).unwrap();
}
