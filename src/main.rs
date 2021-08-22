use actix_web::http::header;
use actix_web::web::get;
use actix_web::{App, HttpResponse, HttpServer};
use chrono::Local;
use image::io::Reader as ImageReader;
use std::env;
use std::fs::File;
use std::io::Cursor;
use tempfile::tempdir;
use tenki_slack_post::{slack, tenki};

const TEMP_TENKI_IMAGE: &str = "tenki.jpg";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    HttpServer::new(|| {
        App::new().route(
            "/",
            get().to(|| {
                let slack_oauth_token =
                    env::var("SLACK_OAUTH_TOKEN").expect("SLACK_OAUTH_TOKEN is not set");
                let slack_channel = env::var("SLACK_CHANNEL").expect("SLACK_CHANNEL is not set");
                let redirect_page = env::var("REDIRECT_PAGE").expect("REDIRECT_PAGE is not set");

                // Get image from tenki.jp
                let now = Local::now();
                let img = tenki::get_tenki_image(now).unwrap();

                // WebP => JPEG tempfile
                let img = ImageReader::new(Cursor::new(img))
                    .with_guessed_format()
                    .unwrap()
                    .decode()
                    .unwrap();
                let dir = tempdir().unwrap();
                let file_path = dir.path().join(TEMP_TENKI_IMAGE);
                let file = File::create(file_path.clone()).unwrap();
                img.save_with_format(file_path.as_path(), image::ImageFormat::Jpeg)
                    .unwrap();

                // Post image to Slack
                slack::post_image_to_slack(&slack_channel, &slack_oauth_token, file_path.as_path())
                    .unwrap();
                drop(file);
                dir.close().unwrap();

                HttpResponse::MovedPermanently()
                    .header(header::LOCATION, redirect_page)
                    .finish()
            }),
        )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
