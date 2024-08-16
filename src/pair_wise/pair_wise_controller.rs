use actix_web::{cookie::Cookie, error, get, http::header::{self, ContentType}, post, web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

use crate::templates::t::render;


#[derive(Debug, Serialize, Deserialize)]
struct VoteRequest {
    item_id: i64
}

#[post("")]
async fn add_vote(
    data: web::Form<VoteRequest>
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let _ = super::pair_wise_service::add_vote(&data.email)
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


