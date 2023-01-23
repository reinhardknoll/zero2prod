use actix_web::{web::Form, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<FormData>) -> impl Responder {
    print!("Subscribed with {} and {}", form.email, form.name);
    HttpResponse::Ok()
}
