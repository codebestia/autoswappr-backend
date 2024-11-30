use axum::{
    routing::{get, post},
    Router,
};
mod health_check;
mod transaction_logs;
mod types;

use crate::AppState;

// Application router.
// All routes should be merged here.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::health_check))
        .route(
            "/log_transaction",
            post(transaction_logs::log_transaction_to_db),
        )
}
