#[macro_use]
extern crate rocket;

mod logging;
mod cors;

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    logging::setup_logging();

    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![index])
        .launch()
        .await;
}

#[get("/")]
async fn index() -> &'static str {
    "hello world"
}




