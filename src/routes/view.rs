use actix_web::{error, web, Error, HttpResponse};
use tera::{Context, Tera};
use actix_session::Session;


use super::super::comm::*;

pub async fn index_view(
    tmpl: web::Data<Tera>
) -> Result<HttpResponse, Error> {
 
    let mut context = Context::new();
    context.insert("app_name", "gogo mstock");

    let rendered = tmpl
        .render("index.html", &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn get_html(
    tmpl: web::Data<Tera>,
    session: Session,
    html_id: web::Path<(String,)>,    
) -> Result<HttpResponse, Error> {

    if util::check_session(&session) { return Ok(HttpResponse::Ok().body("who?")) };

    let mut context = Context::new();
    context.insert("app_name", "gogo mstock");    

    let view_html = format!("{}", &html_id.0);
    let rendered = tmpl
        .render(&view_html, &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(rendered))
}
