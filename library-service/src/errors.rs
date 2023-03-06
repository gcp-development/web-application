use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ServiceError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

impl ServiceError {
    fn error_response(&self) -> String {
        match self {
            ServiceError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            ServiceError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            ServiceError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for ServiceError {
    fn from(err: actix_web::error::Error) -> Self {
        ServiceError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for ServiceError {
    fn from(err: SQLxError) -> Self {
        ServiceError::DBError(err.to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::DBError(_msg) | ServiceError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ServiceError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}