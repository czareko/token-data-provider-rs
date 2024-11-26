use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use crate::domain::services::data_storage_service::{DataStorageService, DataStorageTrait};

pub fn create_token_rest_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let token_data = warp::path!("token"/"data")
        .and_then( move || async move{
            let data_service = Arc::new(DataStorageService);

            let tokens = data_service.get_tokens();

            if !tokens.is_empty() {
                let reply = warp::reply::json(&tokens);
                Ok::<_, warp::Rejection>(warp::reply::with_status(reply, warp::http::StatusCode::OK))
            } else {
                let error_reply = warp::reply::json(&json!({ "error": "No tokens found" }));
                Ok::<_, warp::Rejection>(warp::reply::with_status(
                    error_reply,
                    warp::http::StatusCode::NOT_FOUND,
                ))
            }

        });

    token_data
}