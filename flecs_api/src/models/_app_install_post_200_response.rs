/*
 * FLECS Daemon API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0-beta.4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppInstallPost200Response {
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl AppInstallPost200Response {
    pub fn new() -> AppInstallPost200Response {
        AppInstallPost200Response {
            additional_info: None,
            app: None,
            version: None,
        }
    }
}


