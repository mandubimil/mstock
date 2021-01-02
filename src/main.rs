use actix_web::{web, App, HttpServer, middleware};
// use actix_web::{web, App, HttpServer};
use r2d2_oracle::OracleConnectionManager;

use actix_files as fs;
use tera::Tera;

use actix_redis::RedisSession;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use json::JsonValue;

mod routes;
mod comm;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let file = File::open("../mstock.conf").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let result = json::parse(&contents);
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string()},
    };

    let db_ip = injson["db_ip"].as_str().unwrap();
    let db_id = injson["db_id"].as_str().unwrap();
    let db_passwd = injson["db_passwd"].as_str().unwrap();
    let service_ip_port = injson["service_ip_port"].as_str().unwrap();

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = OracleConnectionManager::new(&format!("{}",db_id), &format!("{}",db_passwd), &format!("//{}/orcl",db_ip));
    let pool = r2d2::Pool::builder().max_size(5).build(manager).unwrap();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/view/**/*")).unwrap();

        App::new()
            // enable logger
            .data(pool.clone())
            .data(tera)
            .wrap(middleware::Logger::default())
            .wrap(RedisSession::new("192.168.0.111:6379", &[0; 32]))

            .service(web::resource("/view/{html_id}").route(web::get().to(routes::view::get_html)))
            .service(fs::Files::new("/static", "static/"))

            .service(web::resource("/").route(web::get().to(routes::view::index_view)))
            .service(web::resource("/gogo/{id}").route(web::post().to(routes::login::post_job)))

            .service(web::resource("/common_url_get/{id}").route(web::get().to(routes::common_url::get_job)))
            .service(web::resource("/common_url_post/{id}").route(web::post().to(routes::common_url::post_job)))

            .service(web::resource("/bc1010/{id}").route(web::post().to(routes::bc1010::post_job)))
            .service(web::resource("/bc1020/{id}").route(web::post().to(routes::bc1020::post_job)))

            .service(web::resource("/bt1010/{id}").route(web::post().to(routes::bt1010::post_job)))

            .service(web::resource("/ju1010/{id}").route(web::post().to(routes::ju1010::post_job)))

            .service(web::resource("/rt1010/{id}").route(web::post().to(routes::rt1010::post_job)))
            .service(web::resource("/rt1020/{id}").route(web::post().to(routes::rt1020::post_job)))
            .service(web::resource("/rt1040/{id}").route(web::post().to(routes::rt1040::post_job)))

            .service(web::resource("/pu1020/{id}").route(web::post().to(routes::pu1020::post_job)))
            .service(web::resource("/pu1030/{id}").route(web::post().to(routes::pu1030::post_job)))
            .service(web::resource("/pu1040/{id}").route(web::post().to(routes::pu1040::post_job)))
            .service(web::resource("/pu1050/{id}").route(web::post().to(routes::pu1050::post_job)))
            .service(web::resource("/pu1070/{id}").route(web::post().to(routes::pu1070::post_job)))
            .service(web::resource("/pu1080/{id}").route(web::post().to(routes::pu1080::post_job)))

            .service(web::resource("/python_exe").route(web::post().to(routes::common_url::python_exe)))


    })
    .bind_openssl(&format!("{}",service_ip_port), builder)?
    .run()
    .await
}
