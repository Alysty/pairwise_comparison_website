use actix_web::{get, http::header::ContentType, Error, HttpRequest, HttpResponse};

use super::super::templates::t::render;

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    let body: String;
    match req.cookie("email_added") {
        Some(_) => {
            body = render("index.html", tera::Context::new())?;
        },
        None => {
            body = render("register_page.html", tera::Context::new())?;
        },
    }
    Ok(
        HttpResponse::Ok()
            .content_type(
                ContentType::html()
            )
            .body(body)
    )
}
