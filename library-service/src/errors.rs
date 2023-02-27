use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum BookError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}


impl BookError {
    fn error_response(&self) -> String {
        match self {
            BookError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            BookError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            BookError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for BookError {
    fn status_code(&self) -> StatusCode {
        match self {
            BookError::DBError(_msg) | BookError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            BookError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for BookError {
    fn from(err: actix_web::error::Error) -> Self {
        BookError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for BookError {
    fn from(err: SQLxError) -> Self {
        BookError::DBError(err.to_string())
    }
}