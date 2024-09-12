// for each entity
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

use crate::models::entity_name::*;

pub async fn create_entity_name(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<AddEntityNamePayload>,
) -> Result<impl IntoResponse> {
    services
        .entity_plural_service
        .create_entity_name(payload)
        .await
        .map(|entity_name| (StatusCode::CREATED, Json(entity_name)))
}

pub async fn get_entity_name(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    return services
        .entity_plural_service
        .get_entity_name(&id)
        .await
        .map(|entity_name| (StatusCode::OK, Json(EntityNameResponse::EntityName(entity_name))));
}

pub async fn get_entity_plural(
    Query(filter_params): Query<EntityNameFilterParams>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    // for each filter
    // filter
    if filter_params.is_most_specific_attribute_name_filter() {
        services.
                entity_plural_service
                .get_entity_name_by_most_specific_attribute_name(
                    // for each filter attribute
                    &filter_params.filter_attribute_name,
                    &filter_params.filter2_attribute_name,
                    // end for each filter attribute
                )
                .await
                .map(|entity_name| (StatusCode::OK, Json(entity_plural)))
    }
    // end filter
    else
    // filter
    if filter_params.is_most_specific_attribute_name_filter() {
        services.
                entity_plural_service
                .is_most_specific_attribute_name_filter(
                    // for each filter attribute
                    &filter_params.filter_attribute_name,
                    &filter_params.filter2_attribute_name,
                    // end for each filter attribute
                )
                .await
                .map(|entity_name| (StatusCode::OK, Json(entity_plural)))
    }
    // end filter
    // end for each filter
    else {
        Err(Error::InvalidEntityNameFilter)
    }
        
}

pub async fn update_entity_name(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<UpdateEntityNamePayload>,
) -> Result<impl IntoResponse> {
    services
        .entity_plural_service
        .update_entity_name(&id, payload)
        .await
        .map(|entity_name| (StatusCode::OK, Json(entity_name)))
}

pub async fn delete_entity_name(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    services.entity_plural_service.delete_entity_name(&id).await
}

#[derive(Deserialize)]
pub struct AddEntityNamePayload {
    // for each attribute
    pub attribute_name: AttributeType,
    pub attribute2_name: Attribute2Type,
    // end for each attribute
}

#[derive(Deserialize)]
pub struct UpdateEntityNamePayload {
    // for each attribute
    pub attribute_name: Option<AttributeType>,
    pub attribute2_name: Option<Attribute2Type>,
    // end for each attribute
}
// end for each entity