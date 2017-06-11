#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;

use iron::prelude::*;
use iron::status;
use router::Router;
use persistent::Read;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TalentForm {
    name: String,
    username: String,
    password: String,
    password_confirmation: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Talent {
    name: String,
    username: String,
}

fn post_talent(request: &mut Request) -> IronResult<Response> {
    match request.get::<bodyparser::Struct<TalentForm>>() {
        Ok(Some(talent)) => Ok(Response::with((status::Ok, create_talent(talent)))),
        Ok(None)         => Ok(Response::with(status::BadRequest)),
        Err(_)           => Ok(Response::with(status::InternalServerError)),
    }
}

fn create_talent(new_talent: TalentForm) -> String {
    let talent = Talent { name: new_talent.name, username: new_talent.username };
    serde_json::to_string(&talent).unwrap()
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    let mut router = Router::new();
    router.post("/talent", post_talent, "post_talent");

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}
