use actix_web::web::{Json, Query};
use actix_web::Responder;
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, JsonSchema, ApiComponent)]
struct Place {
    pub id: Uuid,
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SearchParams {
    name: String,
}
impl SearchParams {
    pub fn get_name(self) -> String {
        self.name
    }
}

#[api_operation(summary = "Returns a list of places that match a given string")]
pub async fn get_places(search_params: Query<SearchParams>) -> impl Responder {
    let name = search_params.into_inner().get_name();
    Json(Place {
        id: Uuid::new_v4(),
        name,
        lat: Default::default(),
        lon: Default::default(),
    })
}
