use actix_web::HttpResponse;

use crate::api::models::ApiError;

pub async fn api_route_not_found() -> HttpResponse {
    let api_error = ApiError {
        message: "API route not found".into(),
    };
    HttpResponse::NotFound().json(&api_error)
}

pub fn internal_server_error<S: Into<String>>(message: S) -> HttpResponse {
    let api_error = ApiError {
        message: message.into(),
    };
    HttpResponse::InternalServerError().json(&api_error)
}
