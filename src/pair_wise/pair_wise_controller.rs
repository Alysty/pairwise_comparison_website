use actix_web::{
    cookie::Cookie,
    error, get,
    http::header::{self, ContentType},
    post, web, HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};

use super::pair_wise_structs::Vote;
use crate::templates::t::render;

#[derive(Debug, Serialize, Deserialize)]
struct VoteRequest {
    win_item_id: i64,
    lose_item_id: i64,
}
// TODO: remove this test function
#[get("")]
async fn get_all_items(
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let result = super::pair_wise_service::get_all_items()
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
#[post("")]
async fn add_vote(
    data: web::Form<VoteRequest>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let _ = super::pair_wise_service::add_vote(Vote {
        user_id: 1,
        win_item_id: data.win_item_id,
        lose_item_id: data.lose_item_id,
    })
    .map_err(|err| {
        log::error!("{err:?}");
        error::ErrorInternalServerError("Unknown error")
    })?;

    let body = render("index.html", tera::Context::new())?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(Cookie::new("email_added", "true"))
        .body(body))
}

pub fn get_route_config() -> Scope {
    web::scope("pair_wise")
        .service(add_vote)
        .service(get_all_items)
}
