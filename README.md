# ene-lightspeed
Generate your next event-driven microservice in any language!


- The general CRUD stuff. eg 
        {
            "User": {
                "id": "Uuid",
                "name": "String",
                "organisation_id": "Uuid",
                "unit": "String",
                "primary_key": "id",
                "filter_by": [
                    "name"
                ]
            }
        },

Will generate for you:
create_car
get_car/get_cars/filter_cars_by_name
update_car
delete_car

we have a set of keywords to construct the template:
- attribute name
- attribute type. Must always be present on the same line as the attribute type
- attribute name2, attribute type2, to indicate how attributes will be generated (eg commas inbetween, trailing commas, ..)
- entity name
- entity name plural
- primary key. Always a UUID for now
- most specific filter attribute name
- filter attribute name
- filter attribute type
- filter attribute name2, filter attribute type2, to indicate how filter attributes will be generated (eg && or AND inbetween)

Which naming convention is used will be derived from how the word is spelled in the template. Example:

struct template (Rust):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityName {
    attribute_name: attribute_type,
}
```