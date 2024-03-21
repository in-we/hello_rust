use actix_web::{HttpResponse, web};
use std::io::Cursor;
use log::info;
use crate::mandelbrot::control::mandelbrot::generate_mandelbrot;
use crate::mandelbrot::entity::fractal::Fractal;

pub async fn fractal_get_handler(query: web::Query<Fractal>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(process(query.into_inner()))
}

pub async fn fractal_post_handler(fractal: web::Json<Fractal>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(process(fractal.into_inner()))
}

fn process(fractal: Fractal) -> Vec<u8> {
    let img = generate_mandelbrot(fractal);

    let start = std::time::Instant::now();
    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png).expect("Could not write image to buffer.");
    info!("Mandelbrot PNG image generated in {} ms", start.elapsed().as_millis());
    buffer
}
