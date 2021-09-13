use actix_web::http::header;
use actix_web::web::Form;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use image::io::Reader as ImageReader;
use log::info;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Cursor;
use tempfile::tempdir;
use tenki_slack_post::{slack, tenki};

const TEMP_TENKI_IMAGE: &str = "tenki.jpg";

#[derive(Deserialize)]
struct TenkiSlackPostRequest {
    slack_channel: String,
    slack_oauth_token: String,
    redirect_page: String,
}

#[post("/post")]
async fn post(req: Form<TenkiSlackPostRequest>) -> impl Responder {
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
    slack::post_image_to_slack(
        req.slack_channel.as_str(),
        req.slack_oauth_token.as_str(),
        file_path.as_path(),
    )
    .unwrap();
    drop(file);
    dir.close().unwrap();

    HttpResponse::MovedPermanently()
        .header(header::LOCATION, req.redirect_page.clone())
        .finish()
}

#[get("/form")]
async fn form() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset='UTF-8' />
    <style type='text/css'>
label {
    display: inline-block;
    text-align: right;
    width: 200px;
}
    </style>
  </head>
  <body>
    <form action='/post' method='POST' >
      <div>
        <label for='slack_channel'>Slack Channel: </label>
        <input type='text' name='slack_channel' id='slack_channel' />
      </div>
      <div>
        <label for='slack_oauth_token'>Slack OAuth Token: </label>
        <input type='text' name='slack_oauth_token' id='slack_oauth_token' />
      </div>
      <div>
        <label for='redirect_page'>Redirect Page: </label>
        <input type='text' name='redirect_page' id='redirect_page' />
      </div>
      <input type='submit' name='submit' />
    </form>
  </body>
</html>
"#,
    )
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::MovedPermanently()
        .header(header::LOCATION, "/form")
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    info!("Bootstrapping the server...");
    HttpServer::new(move || App::new().service(form).service(post).service(index))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
