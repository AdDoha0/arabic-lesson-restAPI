use axum::{extract::State, http::StatusCode, Json};
use axum::response::{IntoResponse, Json as AxumJson};

use crate::serializers::Item;
use crate::{serializers::RequestItem, state::AppState};


#[derive(serde::Serialize)]
struct  DeletedItemsResponse {
    deleted_count: u64,
}


pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<RequestItem>
) -> (StatusCode, AxumJson<Item>) {
    let item = state.create_item(&payload.name, &payload.description)
        .await
        .expect("Error in create_item");


    (StatusCode::CREATED, AxumJson(item))
}



pub async fn get_items(
    State(state): State<AppState>
) -> impl IntoResponse {

    let items = state.get_items()
        .await
        .expect("Error in get_items");

    AxumJson(items)
}


pub async fn get_item(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>
) -> impl IntoResponse {

    match state.get_item(id).await {
        Ok(item) => (StatusCode::OK, AxumJson(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}



pub async fn update_item (
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<RequestItem>
) -> impl IntoResponse {

    match state.update_item(id, &payload.name, &payload.description).await {
        Ok(item) => (StatusCode::OK, AxumJson(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}


pub async fn delete_item (
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>
) -> impl IntoResponse {

    match  state.delete_item(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}


pub async fn delete_all_items (State(state): State<AppState>) -> impl IntoResponse {

    match state.delete_all_items().await {
        Ok(deleted_count) => (StatusCode::OK, AxumJson(DeletedItemsResponse { deleted_count })).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}


