use actix_web::HttpResponse;
use http::StatusCode;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;

use crate::models::error::ApiGatewayError;

/// HTTP response of the API.
#[derive(Deserialize, Serialize)]
pub struct ServiceResponse<T> {
    data: Option<T>,
    error: Option<String>,
}

impl<T> ServiceResponse<T> {
    /// Creates a response containing normal data.
    fn ok(data: Option<T>) -> Self {
        ServiceResponse { data, error: None }
    }

    /// Creates a response containing error.
    fn err(error: Option<String>) -> Self {
        ServiceResponse { data: None, error }
    }
}

/// Returns HttpResponse by status code.
///
/// # Arguments
///
/// * `status_code` - HTTP status code.
/// * `service_response` - A response received from back-end service. It will be wrapped by HttpResponse.
fn get_response_by_status_code<T: DeserializeOwned + Serialize>(
    status_code: StatusCode,
    service_response: ServiceResponse<T>,
) -> HttpResponse {
    let ServiceResponse { data, error } = service_response;

    match status_code {
        StatusCode::OK => HttpResponse::Ok().json(ServiceResponse::<T>::ok(data)),
        StatusCode::NOT_FOUND => HttpResponse::NotFound().json(ServiceResponse::<T>::err(error)),
        StatusCode::BAD_REQUEST => {
            HttpResponse::BadRequest().json(ServiceResponse::<T>::err(error))
        }
        StatusCode::CONFLICT => HttpResponse::Conflict().json(ServiceResponse::<T>::err(error)),
        StatusCode::UNAUTHORIZED => {
            HttpResponse::Unauthorized().json(ServiceResponse::<T>::err(error))
        }
        _ => HttpResponse::InternalServerError().json(ServiceResponse::<T>::err(error)),
    }
}

/// Parses JSON body in service response.
///
/// # Arguments
///
/// * `response` - A general HTTP response.
pub async fn parse_data_from_service_response<T: DeserializeOwned + Serialize>(
    response: Response,
) -> Result<Option<T>, ApiGatewayError> {
    match response.json::<ServiceResponse<T>>().await {
        Ok(service_response) => Ok(service_response.data),
        Err(_) => Err(ApiGatewayError::ServiceResponseParsingFailure),
    }
}

/// Converts http response from back-end service to .
///
/// # Arguments
///
/// * `response` - HTTP response received from back-end service.
pub async fn pass_response<T: DeserializeOwned + Serialize>(
    response: reqwest::Result<Response>,
) -> HttpResponse {
    match response {
        Ok(response) => {
            let status_code = response.status();
            match response.json::<ServiceResponse<T>>().await {
                Ok(service_response) => {
                    get_response_by_status_code::<T>(status_code, service_response)
                }
                Err(_) => {
                    HttpResponse::InternalServerError().json(ServiceResponse::<T>::err(Some(
                        format!("{}", ApiGatewayError::ServiceResponseParsingFailure),
                    )))
                }
            }
        }
        Err(error) => get_response_by_status_code::<T>(
            error.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            ServiceResponse {
                data: None,
                error: None,
            },
        ),
    }
}

/// Returns 200 OK HTTP response that contains `data`.
///
/// # Arguments
///
/// * `data` - The data to be contained in response.
pub fn get_ok_response<T: DeserializeOwned + Serialize>(data: T) -> HttpResponse {
    get_response_by_status_code::<T>(StatusCode::OK, ServiceResponse::<T>::ok(Some(data)))
}

/// Returns HTTP error response.
///
/// # Arguments
///
/// * `status_code` - HTTP status code.
/// * `error` - An error to be contained in response.
pub fn get_err_response<T: DeserializeOwned + Serialize>(
    status_code: StatusCode,
    error: &str,
) -> HttpResponse {
    get_response_by_status_code::<T>(status_code, ServiceResponse::err(Some(error.to_string())))
}

/// Returns back-end service url.
///
/// # Arguments
///
/// * `resource` - A resource of the service.
pub fn get_url(resource: &str) -> String {
    let base_url = env::var("BACK_END_SERVICE_ADDRESS").unwrap();
    format!("{}{}", base_url, resource)
}
