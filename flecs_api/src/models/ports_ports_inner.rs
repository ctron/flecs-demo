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
pub struct PortsPortsInner {
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl PortsPortsInner {
    pub fn new() -> PortsPortsInner {
        PortsPortsInner {
            container: None,
            host: None,
        }
    }
}


