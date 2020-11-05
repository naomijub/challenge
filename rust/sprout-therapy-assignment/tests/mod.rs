use sprout_therapy_assignment::config::routes::app_routes;

use actix_service::Service;
use actix_web::{http::StatusCode, test, App};
use bytes::Bytes;
use env_logger;

mod data;

#[actix_rt::test]
async fn test_ping_pong() {
    env_logger::init();
    let mut app = test::init_service(App::new().configure(app_routes)).await;

    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::read_response(&mut app, req).await;

    assert_eq!(resp, Bytes::from_static(b"pong"));
}

#[actix_rt::test]
async fn test_readiness_ok() {
    let mut app = test::init_service(App::new().configure(app_routes)).await;

    let req = test::TestRequest::with_uri("/~/ready").to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::ACCEPTED);
}

#[actix_rt::test]
async fn not_found_route() {
    let mut app = test::init_service(App::new().configure(app_routes)).await;

    let req = test::TestRequest::get().uri("/crazy-route").to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
