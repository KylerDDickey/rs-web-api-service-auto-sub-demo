use actix_web::{App, HttpServer};

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    build_and_start("localhost", 8080).await
}

macro_rules! config_scope {
    ($s: ident) => {
        |cfg: &mut actix_web::web::ServiceConfig| {
            cfg.service(actix_web::web::scope(&format!("/{}", stringify!($s)))
                .configure(services::scopes::$s::config));
        }
    };
}

async fn build_and_start(ip_addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config_scope!(hello))
    })
        .bind(format!("{}:{}", ip_addr, port))?
        .run()
        .await
}
