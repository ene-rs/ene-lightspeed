// for each entity
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use crate::error::Error;
use crate::models::PaginatedResult;
use serde::Serializer;
use crate::controllers::entity_name_controller::{AddEntityNamePayload, UpdateEntityNamePayload};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityName {
    
    // for each attribute
    pub attribute_name: AttributeType,
    pub attribute2_name: Attribute2Type,
    // end for each attribute
}

impl EntityName {
    pub fn new(payload: AddEntityNamePayload) -> Result<Self, Error> {
        Ok(Self {
            primary_key: payload.primary_key,
            // for each attribute
            attribute_name: payload.attribute_name,
            attribute2_name: payload.attribute2_name,
            // end for each attribute
        })
    }

    pub fn update(payload: UpdateEntityNamePayload) -> Result<Self, Error> {
        Ok(Self {
            primary_key: payload.primary_key,
            // for each attribute
            attribute_name: payload.attribute_name,
            attribute2_name: payload.attribute2_name,
            // end for each attribute
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityNameFilterAttributes {

    // for each filter
    //    filter
    //       for each filter attribute
    filter_attribute_name: Option<FilterAttributeType>,
    filter2_attribute_name: Option<Filter2AttributeType>,
    //       end for each filter attribute
    //     end filter
    //     filter
    //        for each filter attribute
    filter_attribute_name: Option<FilterAttributeType>,
    filter2_attribute_name: Option<Filter2AttributeType>,
    //         end for each filter attribute
    //      end filter
    // end for each filter
}

impl EntityNameFilterAttributes {
    // for each filter
    //    filter
    fn is_most_specific_attribute_name_filter(&self) -> bool {
        //    for each filter attribute
        self.filter_attribute_name.is_some() && self.filter2_attribute_name.is_some()
        //     end for each filter attribute
    }
    //     end filter
    //     filter
    fn is_most_specific_attribute_name_filter(&self) -> bool {
        //    for each filter attribute
        self.filter_attribute_name.is_some() && self.filter2_attribute_name.is_some()
        //    end for each filter attribute
    }
    //     end filter
    // end for each filter

}
// end for each entity