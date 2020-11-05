use crate::controller::{pong, readiness};
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}
