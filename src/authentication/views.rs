use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;


#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all="camelCase")]
pub struct  LoginSchema {
    #[schema(example="user@example.com")]
    pub email:String,
    pub password:String
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all="camelCase")]
pub struct DeserializeToken {
    pub user_id:i32,
    pub access_token:String,
    pub refresh_token:String
}