#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;

extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;

mod schema;
mod talent;

use iron::Iron;
use iron::prelude::Chain;
use router::Router;
use persistent::Read;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    let mut router = Router::new();
    router.post("/talent", talent::handlers::post_talent, "post_talent");

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}
