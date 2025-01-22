use std::future::IntoFuture;

use alloy::primitives::{Address, Bytes, TxHash, B256, U256};
use axum::{
    extract::{rejection::JsonRejection, FromRequest, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use blueprint_sdk::logging;
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};

use crate::ServiceContext;

/// The routes for the HTTP server.
pub fn routes() -> axum::Router<crate::ServiceContext> {
    axum::Router::new().route("/relay", axum::routing::post(relay_tx))
}

#[derive(Clone, Debug, Deserialize)]
pub struct RelayTransactionRequest {
    from: Address,
    to: Address,
    #[serde(default)]
    value: U256,
    data: Bytes,
    gaslimit: u64,
    deadline: U256,
    v: u8,
    r: B256,
    s: B256,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub enum RelayTransactionResponse {
    Success {
        tx_hash: TxHash,
        simulated_outcome: Bytes,
    },
    Failure {
        error: String,
    },
}

pub async fn relay_tx(
    State(ctx): State<ServiceContext>,
    AppJson(tx): AppJson<RelayTransactionRequest>,
) -> Result<(StatusCode, AppJson<RelayTransactionResponse>), AppError> {
    let RelayTransactionRequest {
        from,
        to,
        value,
        data,
        gaslimit,
        deadline,
        v,
        r,
        s,
    } = tx;
    let dispatch_builder = ctx
        .call_permit_instance
        .dispatch(from, to, value, data, gaslimit, deadline, v, r, s);

    let outcome = dispatch_builder
        .call()
        .into_future()
        .map_err(crate::Error::from)
        .await?;

    let pending_tx = dispatch_builder.send().map_err(crate::Error::from).await?;

    Ok((
        StatusCode::OK,
        AppJson(RelayTransactionResponse::Success {
            tx_hash: *pending_tx.tx_hash(),
            simulated_outcome: outcome.output,
        }),
    ))
}

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

// The kinds of errors we can hit in our application.
pub enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    // Error from a third party library we're using
    Blueprint(crate::Error),
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize, Default)]
        struct ErrorResponse {
            status: &'static str,
            error: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            details: Option<String>,
        }

        let (status, err) = match self {
            AppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (
                    rejection.status(),
                    ErrorResponse {
                        status: "failure",
                        error: "Invalid JSON".to_owned(),
                        details: Some(rejection.body_text()),
                    },
                )
            }
            AppError::Blueprint(err) => {
                // Because `TraceLayer` wraps each request in a span that contains the request
                // method, uri, etc we don't need to include those details here
                logging::error!(%err, "something went wrong");

                match err {
                    crate::Error::Contract(alloy::contract::Error::TransportError(
                        alloy::transports::TransportError::ErrorResp(e),
                    )) => (
                        StatusCode::BAD_REQUEST,
                        ErrorResponse {
                            status: "failure",
                            error: e.message.to_string(),
                            details: e.data.as_ref().map(|data| data.get().replace("\"", "")),
                        },
                    ),
                    _ =>
                    // Don't expose any details about the error to the client
                    {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ErrorResponse {
                                status: "failure",
                                error: "Internal Server Error".to_owned(),
                                details: None,
                            },
                        )
                    }
                }
            }
        };

        (status, AppJson(err)).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<crate::Error> for AppError {
    fn from(error: crate::Error) -> Self {
        Self::Blueprint(error)
    }
}
