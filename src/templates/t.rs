use lazy_static::lazy_static;
use tera::Tera;
use actix_web::Error;

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("./assets/templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

pub fn render(template_name: &str, context: tera::Context) -> Result<String, Error> {
    match TEMPLATES.render(template_name, &context) {
        Ok(x) => Ok(x),
        Err(err) => {
            return Err(actix_web::error::ErrorInternalServerError(err))
        },
    }
}
