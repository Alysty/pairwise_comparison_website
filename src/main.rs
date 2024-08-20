mod general_error;
mod templates;
mod user;
mod database;
mod index;
mod pair_wise;
mod items;
use actix_web::{
    get, middleware::Logger, App, HttpResponse, HttpServer, Responder
};
use actix_files as afs;
use env_logger::Env;

#[get("/ping")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    _ = items::add_items_to_db().unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                afs::Files::new("/public", "./assets/public")
                .show_files_listing()
            )
            .service(hello)
            .service(index::index_controller::index)
            .service(user::user_controller::get_route_config())
            .service(pair_wise::pair_wise_controller::get_route_config())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
