use actix_web::web;
use crate::handlers::book::*;
use crate::handlers::default::*;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/probe", web::get().to(get_probe));
}

pub fn book_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/library")
            .route("/", web::post().to(post_add_book))
            .route("/bulk", web::post().to(post_bulk_insert))
            .route("", web::get().to(get_books))
            .route("/{id}", web::get().to(get_book_by_id))
            .route("/{id}", web::put().to(put_book_by_id))
            .route("/{id}", web::delete().to(delete_book_by_id)),
    );
}