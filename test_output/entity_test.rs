
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use crate::error::Error;
use crate::models::PaginatedResult;
use serde::Serializer;
use crate::controllers::car_controller::{AddCarPayload, UpdateCarPayload};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub id: Uuid,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub owner: Uuid,
    pub year: I32,
}

impl Car {
    pub fn new(payload: AddCarPayload) -> Result<Self, Error> {
        Ok(Self {
            id: payload.id,
            id: payload.id,
            manufacturer: payload.manufacturer,
            model: payload.model,
            name: payload.name,
            owner: payload.owner,
            year: payload.year,
        })
    }

    pub fn update(payload: UpdateCarPayload) -> Result<Self, Error> {
        Ok(Self {
            id: payload.id,
            id: payload.id,
            manufacturer: payload.manufacturer,
            model: payload.model,
            name: payload.name,
            owner: payload.owner,
            year: payload.year,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarFilterAttributes {
    id: Option<Uuid>,
    name: Option<String>,
    model: Option<String>,
    manufacturer: Option<String>,
    year: Option<I32>,
    owner: Option<Uuid>,
}

impl CarFilterAttributes {
    fn is_id_filter(&self) -> bool {
        self.id.is_some()
    }
    fn is_model_filter(&self) -> bool {
        self.name.is_some() && self.model.is_some()
    }
    fn is_manufacturer_filter(&self) -> bool {
        self.manufacturer.is_some()
    }
    fn is_year_filter(&self) -> bool {
        self.year.is_some()
    }
    fn is_owner_filter(&self) -> bool {
        self.owner.is_some()
    }

}