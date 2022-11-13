use actix_web::{get, web, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    hello: String
}

#[get("/")]
async fn greet() -> impl Responder {
    web::Json(MyObj {
        hello: "world".to_lowercase()
    })
}
