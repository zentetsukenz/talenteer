#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;

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
