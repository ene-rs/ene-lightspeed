# ene-lightspeed
Generate your next CRUD program/server/microservice/whatever in any language/framework!
This is a continuation of https://github.com/Abdullahsab3/lightspeed

                                                                                                                   
                                                                                                                   
                                                                                                                   
                                                                                                                   
                                                                                                                   
          ┌──────────────────────────────┐                                                                        
          │                              │                                                                        
          │                              │                                                                        
          │                              │                                                                        
          │                              │                                                                        
          │  Template of CRUD programs   ├─────────────────────┐                                                  
          │                              │                     │                                                  
          │                              │                     ▼                                                  
          │                              │             ┌──────────────────┐                                        
          │                              │             │                  │      ┌────────────────────┐           
          └──────────────────────────────┘             │                  │      │                    │           
                                                       │  ene-lightspeed  ├─────►│ Your CRUD program! │           
                                                       │                  │      │                    │           
                                                       │                  │      └────────────────────┘           
           ┌────────────────────────────┐              └──────────────────┘                                        
           │                            │                       ▲                                                 
           │                            │                       │                                                 
           │                            │                       │                                                 
           │  Your entity definitions   ├───────────────────────┘                                                 
           │                            │                                                                         
           │                            │                                                                         
           │                            │                                                                         
           └────────────────────────────┘                                                                         
                                                                                                                   
                                                                                                                   
                                                                                                                   
                                                                                                                   
                                                                                                                   

This is still in very early development. No documentation is available, and no working functionalities are available

- [x] Templating language and parser
- [x] Template AST
- [x] Code generation
- [x] Entity definitions
- [ ] Mapping of types to make sure templates are flexible (eg if you have SQL in the project next to, for example, Rust or Scala)
- [ ] Optional types
- [ ] Actual project generation


## Features
The general CRUD stuff. eg
```json
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
```

Will generate for you:
create_car
get_car/get_cars/filter_cars_by_name
update_car
delete_car

we have a set of keywords to construct the template:
- attribute name
- attribute type
- attribute name2, attribute type2, to indicate how attributes will be generated (eg commas inbetween, trailing commas, ..)
- entity name
- entity name plural
- primary keys
- most specific filter attribute name
- filter attribute name
- filter attribute type
- filter attribute name2, filter attribute type2, to indicate how filter attributes will be generated (eg && or AND inbetween)
- Composite keys
- and more

Which naming convention is used will be derived from how the word is spelled in the template. Example:

struct template (Rust):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityName {
    attribute_name: AttributeType,
}
```

Will generate:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub unit: String,
    pub name: String,
}
```

`Car` is upper camel case since `EntityName` is also upper camel case. `organisation_id` is snake case since `attribute_name` is also snake case
