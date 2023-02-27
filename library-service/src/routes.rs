use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/probe", web::get().to(get_probe));
}

pub fn book_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/library")
            .route("", web::get().to(get_all_books))
            .route("/", web::post().to(new_book))
            .route("/{id}", web::get().to(get_book_by_id))
            .route("/{id}", web::delete().to(delete_book_by_id))
            .route("/{id}", web::put().to(update_book_by_id))
            .route("/bulk", web::post().to(bulk)),
    );
}