extern crate serde;
extern crate serde_json;

extern crate iron;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;

pub fn post_talent(request: &mut Request) -> IronResult<Response> {
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

