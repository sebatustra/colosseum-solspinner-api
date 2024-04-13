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
    BirdeyeFetchFail,
    BirdeyeDeserializationFail,

    // selected token errors
    SelectedTokenCreateFail,
    SelectedTokenGetFail,
    SelectedTokenUpdateFail,
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
            ApiError::JupiterFetchFail => "Error fetching Jupiter price data",
            ApiError::JupiterDeserializationFail => "Error deserializing Jupiter price data",

            // birdeye
            ApiError::BirdeyeFetchFail => "Error fetching data from Birdeye",
            ApiError::BirdeyeDeserializationFail => "Error deserializing Birdeye data",

            // selected token
            ApiError::SelectedTokenCreateFail => "Error creating the selected token",
            ApiError::SelectedTokenGetFail => "Error fetching the selected token",
            ApiError::SelectedTokenUpdateFail => "Error updating selected coins",

        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}