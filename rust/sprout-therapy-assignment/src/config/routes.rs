use crate::controller::{pong, readiness, data_handler};
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .service(
                web::scope("api/")
                    .route("data", web::post().to(data_handler))
            )
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}
