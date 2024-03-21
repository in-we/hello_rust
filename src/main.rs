#![crate_name = "hello_rust"]
use actix_web::{App, HttpServer, web};
use hello_rust::mandelbrot::boundary::request_handler::{fractal_get_handler, fractal_post_handler};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new().service(web::resource("/fractal")
                .route(web::post().to(fractal_post_handler))
                .route(web::get().to(fractal_get_handler))
        )})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
