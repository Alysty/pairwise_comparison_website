use actix_web::{get, HttpResponse, Error, http::header::ContentType};

use super::super::templates::t::render;

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    let body = render("index.html", tera::Context::new())?;
    Ok(
        HttpResponse::Ok()
            .content_type(
                ContentType::html()
            )
            .body(body)
    )
}
