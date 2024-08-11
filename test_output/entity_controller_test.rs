
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::Error;
use crate::error::Result;
use crate::services::ServicesState;

use crate::models::car::*;

pub async fn create_car(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<AddCarPayload>,
) -> Result<impl IntoResponse> {
    services
        .cars_service
        .create_car(payload)
        .await
        .map(|car| (StatusCode::CREATED, Json(car)))
}

pub async fn get_car(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    return services
        .cars_service
        .get_car(&id)
        .await
        .map(|car| (StatusCode::OK, Json(CarResponse::Car(car))));
}

pub async fn get_cars(
    Query(filter_params): Query<CarFilterParams>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    if filter_params.is_id_filter() {
        services.
                cars_service
                .get_car_by_id(
                    &filter_params.id,
                )
                .await
                .map(|car| (StatusCode::OK, Json(cars)))
    }
    else
    if filter_params.is_model_filter() {
        services.
                cars_service
                .get_car_by_model(
                    &filter_params.name,
                    &filter_params.model,
                )
                .await
                .map(|car| (StatusCode::OK, Json(cars)))
    }
    else
    if filter_params.is_manufacturer_filter() {
        services.
                cars_service
                .get_car_by_manufacturer(
                    &filter_params.manufacturer,
                )
                .await
                .map(|car| (StatusCode::OK, Json(cars)))
    }
    else
    if filter_params.is_year_filter() {
        services.
                cars_service
                .get_car_by_year(
                    &filter_params.year,
                )
                .await
                .map(|car| (StatusCode::OK, Json(cars)))
    }
    else
    if filter_params.is_owner_filter() {
        services.
                cars_service
                .get_car_by_owner(
                    &filter_params.owner,
                )
                .await
                .map(|car| (StatusCode::OK, Json(cars)))
    }
    else {
        Err(Error::InvalidCarFilter)
    }
    
    
    
}

pub async fn update_car(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<UpdateCarPayload>,
) -> Result<impl IntoResponse> {
    services
        .cars_service
        .update_car(&id, payload)
        .await
        .map(|car| (StatusCode::OK, Json(car)))
}

pub async fn delete_car(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    services.cars_service.delete_car(&id).await
}

#[derive(Deserialize)]
pub struct AddCarPayload {
    pub id: Uuid,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub owner: Uuid,
    pub year: I32,
}

#[derive(Deserialize)]
pub struct UpdateCarPayload {
    pub id: Option<Uuid>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub name: Option<String>,
    pub owner: Option<Uuid>,
    pub year: Option<I32>,
}