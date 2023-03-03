use actix_web::{HttpResponse, web};
use crate::state::AppState;

pub async fn get_probe(app_state: web::Data<AppState>) -> HttpResponse {
    let probe_response = &app_state.probe;
    HttpResponse::Ok().json(&probe_response)
}