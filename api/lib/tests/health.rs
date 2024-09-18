use actix_web::{test, App, http::StatusCode};
use api_lib::health;

#[actix_rt::test]
async fn health_check_works() {
    let app = App::new().configure(health::service);

    let mut app = actix_web::test::init_service(app).await;
    let req = actix_web::test::TestRequest::get().uri("/health").to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}