use actix_web::{web, test, App};
use hello_rust::mandelbrot::boundary::request_handler::fractal_get_handler;
use hello_rust::mandelbrot::entity::fractal::Fractal;
use serde_urlencoded::to_string;

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_web::test]
    async fn test_fractal_get() {

        let app = test::init_service(App::new().service(web::resource("/fractal")
            .route(web::get().to(fractal_get_handler)))).await;

        let fractal: Fractal = Fractal {
            c0: -1.5,
            c0i: -1.0,
            c1: 0.5,
            c1i: 1.0,
            width: 100,
            height: 100,
            max_iterations: 50
        };
        let query_string = to_string(&fractal).unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/fractal?{}", query_string))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

}
