use actix_web::{web, HttpResponse, Responder};
use log::error;
use crate::schema::{Request, Response};
use crate::logic::{base, base_k};

pub async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

pub async fn readiness() -> impl Responder {
    let process = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output();

    match process {
        Ok(_) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

pub async fn data_handler(payload: web::Json<Request>) -> impl Responder {
    let data: Request = payload.into_inner();
    if let Ok(base_logical_path) = base(&data) {
        let base_k_value = base_k(&base_logical_path, &data);

        let response = Response::new(base_logical_path, base_k_value);
        let body = serde_json::to_string(&response);

        match body {
            Ok(json) => HttpResponse::Ok().body(json),
            _ => HttpResponse::InternalServerError().finish()
        }
    } else {
        error!("No logical value for the given input");
        HttpResponse::BadRequest().body("No such logical combination for A, B, C")
    }
}