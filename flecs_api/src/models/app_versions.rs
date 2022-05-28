/*
 * FLECS Daemon API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0-beta.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AppVersions : App versions



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppVersions {
    #[serde(rename = "desired", skip_serializing_if = "Option::is_none")]
    pub desired: Option<String>,
    #[serde(rename = "installedSize", skip_serializing_if = "Option::is_none")]
    pub installed_size: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl AppVersions {
    /// App versions
    pub fn new() -> AppVersions {
        AppVersions {
            desired: None,
            installed_size: None,
            status: None,
            version: None,
        }
    }
}

