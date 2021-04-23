//! Default Compute@Edge template program.

use fastly::http::{Method, StatusCode};
use fastly::{Body, Error, Request, Response, ResponseExt};

#[fastly::main]
fn main(req: Request<Body>) -> Result<impl ResponseExt, Error> {
    // Pattern match on the request method and path.
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            println!(
                "DOG service {}",
                std::env::var("FASTLY_SERVICE_VERSION").unwrap()
            );
            use std::ops::Deref;

            let dog_jpg = include_bytes!("not_a_dog.jpg");
            println!("dog jpg bytes");
            let dog_jpg_cursor: std::io::Cursor<&[u8]> = std::io::Cursor::new(dog_jpg);
            println!("dog jpg cursor");
            let mut jpeg = jpeg_decoder::Decoder::new(dog_jpg_cursor);
            let dog_pixels = jpeg.decode().expect("decode jpeg");
            println!("decoded");
            let dog_info = jpeg.info().expect("get jpeg info");
            assert_eq!(
                dog_info.pixel_format,
                jpeg_decoder::PixelFormat::RGB24,
                "only rgb24 jpegs supported"
            );

            let encoded_webp = webp::Encoder::new(
                &dog_pixels,
                webp::PixelLayout::Rgb,
                dog_info.width as u32,
                dog_info.height as u32,
            )
            .encode_lossless();
            println!("webp encoded.");

            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/webp")
                .body(Body::from(encoded_webp.deref()))
                .expect("encode body");

            println!("body constructed.");
            Ok(resp)
        }
        // Catch all other requests and return a 404.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("The page you requested could not be found"))?),
    }
}
