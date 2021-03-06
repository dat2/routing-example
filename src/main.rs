#![feature(attr_literals)]

#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate regex;
extern crate routing;
#[macro_use]
extern crate routing_derive;

use futures::future::{self, Future};
use hyper::StatusCode;
use hyper::server::{Http, Request, Response, Service};
use routing::{NewRoutingTable, RoutingTable};

mod errors {
  error_chain! {
    foreign_links {
      AddrParse(::std::net::AddrParseError);
      Hyper(::hyper::Error);
    }
  }
}

// this is just a simple enum so we can match on
#[derive(Debug, RoutingTable)]
enum Routes {
  #[get("/")]
  Index,

  #[post("/echo")]
  Echo,

  #[post("/users/:id")]
  CreateUser {
    id: usize
  },

  #[get("/users/:user_id/friends/:friend_id")]
  GetUserFriend {
    user_id: usize,
    friend_id: usize
  },
}

struct Example;

impl Service for Example {
  // boilerplate hooking up hyper's server types
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  // The future representing the eventual Response your call will
  // resolve to. This can change to whatever Future you need.
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {

    let route = Routes::routing_table().route(&req);

    println!("{:?}", route);

    let response = match route {
      Some(Routes::Index) => Response::new().with_body("Hello World!"),
      Some(Routes::Echo) => Response::new().with_body(req.body()),
      Some(Routes::CreateUser { id }) => Response::new().with_body(format!("Created user with id {}", id)),
      Some(Routes::GetUserFriend { user_id, friend_id }) => Response::new().with_body(format!("User {} is friends with {}", user_id, friend_id)),
      None => Response::new().with_status(StatusCode::NotFound)
    };

    Box::new(future::ok(response))
  }
}


fn run() -> errors::Result<()> {
  let addr = "127.0.0.1:3000".parse()?;
  let server = Http::new().bind(&addr, || Ok(Example))?;
  server.run()?;
  Ok(())
}

quick_main!(run);
