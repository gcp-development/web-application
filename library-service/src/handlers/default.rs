use actix_web::{HttpResponse, web};
use crate::state::AppState;

pub async fn get_probe(app_state: web::Data<AppState>) -> HttpResponse {
    let probe_response = &app_state.probe;
    HttpResponse::Ok().json(&probe_response)
}

#[cfg(test)]
mod tests {
    use std::env;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use crate::handlers::default::get_probe;
    use crate::state::AppState;

    #[actix_rt::test]
    async fn test_add_book() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(10))
            .connect(&database_url)
            .await
            .unwrap();
        let shared_data = web::Data::new(AppState {
            probe: "Probe test ok....".to_string(),
            db: db_pool,
        });

        let http_response = get_probe(shared_data).await;
        assert_eq!(http_response.status(), StatusCode::OK);
    }
}