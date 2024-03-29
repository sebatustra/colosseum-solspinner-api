use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = core::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    // token errors
    TokenCreateFail,
    TokenGetFail,
    TokenUpdateFail,

    // user errors
    UserCreateFail,
    UserGetFail,

    // position errors
    PositionCreateFail,
    PositionGetFail,

    // client errors
    JupiterFetchFail,
    JupiterDeserializationFail,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES_ERR");

        let body = match self {
            // positions
            ApiError::PositionCreateFail => "Error creating the position",
            ApiError::PositionGetFail => "Error fetching positions",

            // tokens
            ApiError::TokenCreateFail => "Error creating the token",
            ApiError::TokenGetFail => "Error fetching tokens",
            ApiError::TokenUpdateFail => "Error updating the token",

            // users
            ApiError::UserCreateFail => "Error creating the user",
            ApiError::UserGetFail => "Error fetching users",

            // jupiter
            ApiError::JupiterFetchFail => "Error fetching jupiter price data",
            ApiError::JupiterDeserializationFail => "Error deserializing jupiter price data"
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}