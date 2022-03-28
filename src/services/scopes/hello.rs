#[auto_service::service]
pub mod controller {
    use actix_web::{route, HttpResponse, web};

    #[route("", method="GET")]
    pub async fn hello_world() -> HttpResponse {
        println!("Hello, world!");
        HttpResponse::Ok().finish()
    }

    #[route("/{name}", method="GET")]
    pub async fn hello_name(path: web::Path<String>) -> HttpResponse {
        println!("Hello, {}!", path);
        HttpResponse::Ok().finish()
    }
}
