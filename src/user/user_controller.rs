use actix_web::{cookie::Cookie, error, get, http::header::{self, ContentType}, post, web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

use crate::templates::t::render;


#[derive(Debug, Serialize, Deserialize)]
struct AddUserJson {
    email: String
}
#[post("")]
async fn add_user(
    data: web::Form<AddUserJson>
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let _ = super::user_service::add_user(&data.email)
        .map_err(|err|{
            log::error!("{err:?}");
            error::ErrorInternalServerError("Unknown error")
        })?;

    let body = render("index.html", tera::Context::new())?;
    Ok(
        HttpResponse::Ok()
            .content_type(
                ContentType::html()
            )
            .cookie(Cookie::new("email_added", "true"))
            .body(body)
    )
}

#[get("/test")]
async fn get_users()
-> actix_web::Result<HttpResponse, actix_web::Error> {
    let result = super::user_service::get_all_users()
        .map_err(|err|{
            log::error!("{err:?}");
            error::ErrorInternalServerError("Unknown error")
        })?;
    Ok(
        HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(result)
    )
}


pub fn get_route_config() -> Scope {
    web::scope("user")
    .service(get_users)
    .service(add_user)
}
