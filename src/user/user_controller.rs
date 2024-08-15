use actix_web::{error, http::header, post, get, web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct AddUserJson {
    email: String
}
#[post("")]
async fn add_user(
    data: web::Json<AddUserJson>
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let _ = super::user_service::add_user(&data.email)
        .map_err(|err|{
            log::error!("{err:?}");
            error::ErrorInternalServerError("Unknown error")
        })?;
    Ok(HttpResponse::Ok().finish())
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
