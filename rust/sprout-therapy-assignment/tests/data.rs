use sprout_therapy_assignment::config::routes::app_routes;
use sprout_therapy_assignment::schema::{Response, Request, LogicalPath};

use actix_web::{http::StatusCode, test, App};
use actix_service::Service;
use bytes::Bytes;

#[actix_rt::test]
async fn h_is_m() {
    let mut app =
        test::init_service(App::new().configure(app_routes)).await;

    let post_req = test::TestRequest::post()
        .uri("/api/data")
        .header("Content-Type", "application/json")
        .set_payload(payload_m().as_bytes().to_owned())
        .to_request();

    let resp = test::read_response(&mut app, post_req).await;

    let response: Response =
        serde_json::from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
    assert_eq!(response.H, LogicalPath::M);
    assert_eq!(response.K, 7f64);
}

#[actix_rt::test]
async fn h_is_p() {
    let mut app =
        test::init_service(App::new().configure(app_routes)).await;

    let post_req = test::TestRequest::post()
        .uri("/api/data")
        .header("Content-Type", "application/json")
        .set_payload(payload_p().as_bytes().to_owned())
        .to_request();

    let resp = test::read_response(&mut app, post_req).await;

    let response: Response =
        serde_json::from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
    assert_eq!(response.H, LogicalPath::P);
    assert_eq!(response.K, 6.617647058823529f64);
}

#[actix_rt::test]
async fn h_is_t() {
    let mut app =
        test::init_service(App::new().configure(app_routes)).await;

    let post_req = test::TestRequest::post()
        .uri("/api/data")
        .header("Content-Type", "application/json")
        .set_payload(payload_t().as_bytes().to_owned())
        .to_request();

    let resp = test::read_response(&mut app, post_req).await;

    let response: Response =
        serde_json::from_str(&String::from_utf8(resp.to_vec()).unwrap()).unwrap();
    assert_eq!(response.H, LogicalPath::T);
    assert_eq!(response.K, 4.95f64);
}

#[actix_rt::test]
async fn error_case_err_body() {
    let mut app =
        test::init_service(App::new().configure(app_routes)).await;

    let post_req = test::TestRequest::post()
        .uri("/api/data")
        .header("Content-Type", "application/json")
        .set_payload(payload_err().as_bytes().to_owned())
        .to_request();

    let resp = test::read_response(&mut app, post_req).await;

    assert_eq!(resp, Bytes::from_static(b"No such logical combination for A, B, C"));
}

#[actix_rt::test]
async fn error_case_err_status() {
    let mut app =
        test::init_service(App::new().configure(app_routes)).await;

    let post_req = test::TestRequest::post()
        .uri("/api/data")
        .header("Content-Type", "application/json")
        .set_payload(payload_err().as_bytes().to_owned())
        .to_request();

        let resp = app.call(post_req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

fn payload_m() -> String {
    serde_json::to_string(&Request{
        A: true,
        B: true,
        C: false,
        D: 7f64,
        E: 0,
        F: 34
    }).unwrap()
}

fn payload_p() -> String {
    serde_json::to_string(&Request{
        A: true,
        B: true,
        C: true,
        D: 7.5f64,
        E: 0,
        F: 3
    }).unwrap()
}

fn payload_t() -> String {
    serde_json::to_string(&Request{
        A: false,
        B: true,
        C: true,
        D: 5.5f64,
        E: -4,
        F: 3
    }).unwrap()
}

fn payload_err() -> String {
    serde_json::to_string(&Request{
        A: true,
        B: false,
        C: true,
        D: 5.5f64,
        E: -4,
        F: 3
    }).unwrap()
}