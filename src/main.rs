#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
extern crate rocket_contrib;
extern crate uuid;

use std::collections::HashMap;
use rocket_contrib::uuid::Uuid;

lazy_static! {
    // A small people lookup table for the sake of this example. In a real
    // application this could be a database lookup. Notice that we use the
    // uuid::Uuid type here and not the rocket_contrib::uuid::Uuid type.
    static ref PEOPLE: HashMap<std::string::String, &'static str> = {
        let mut m = HashMap::new();
        let lacy_id = String::from("one");
        let bob_id = String::from("two");
        let george_id = String::from("three");
        let enzo_id = String::from("four");
        m.insert(lacy_id, "Lacy");
        m.insert(bob_id, "Bob");
        m.insert(george_id, "George");
        m.insert(enzo_id, "Enzo");
        m
    };
}

#[get("/records/<id>")]
fn records(id: String) -> Result<String, String> {
    // Because Uuid implements the Deref trait, we use Deref coercion to convert
    // rocket_contrib::uuid::Uuid to uuid::Uuid.
    Ok(PEOPLE.get(&id)
        .map(|person| format!("We found: {}", person))
        .ok_or_else(|| format!("Person not found for UUID: {}", id))?)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![records])
}

fn main() {
    rocket().launch();
}