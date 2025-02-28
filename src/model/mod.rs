pub mod mesh;
pub mod transform;
pub use mesh::*;

use instant_xml::*;
use serde::{Deserialize, Serialize};
use transform::Transform;

#[derive(Serialize, Deserialize, FromXml, ToXml)]
#[serde(rename_all = "lowercase")]
#[xml(rename_all = "lowercase")]
pub struct Model {
    #[serde(rename = "@xmlns", default)]
    #[xml(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Metadata>,
    pub resources: Resources,
    pub build: Build,
    // #[serde(rename = "@unit", default)]
    #[xml(rename = "@unit")]
    pub unit: Unit,
}

/// Model measurement unit, default is millimeter
#[derive(Serialize, Deserialize, FromXml, ToXml, Default)]
#[xml(scalar, rename_all = "lowercase")]
pub enum Unit {
    Micron = 0,
    #[default]
    Millimeter = 1,
    Centimeter = 2,
    Inch = 3,
    Foot = 4,
    Meter = 5,
}

#[derive(Serialize, Deserialize, FromXml, ToXml)]
pub struct Metadata {
    #[serde(rename = "@name")]
    #[xml(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    #[xml(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml, Default)]
pub struct Resources {
    #[serde(default)]
    pub object: Vec<Object>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // pub basematerials: Option<()>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml)]
#[serde(rename_all = "lowercase")]
#[xml(rename_all = "lowercase")]
pub struct Object {
    #[serde(rename = "@id")]
    #[xml(rename = "@id")]
    pub id: usize,
    #[serde(rename = "@partnumber", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "@partnumber")]
    pub partnumber: Option<String>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@pid", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "@pid")]
    pub pid: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Mesh>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml)]
pub struct Components {
    pub component: Vec<Component>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml)]
pub struct Component {
    #[serde(rename = "@objectid")]
    #[xml(rename = "@objectid")]
    pub objectid: usize,
    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "@transform")]
    pub transform: Option<Transform>,
}
#[derive(Serialize, Deserialize, Default, FromXml, ToXml)]
pub struct Build {
    #[serde(default)]
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml)]
pub struct Item {
    #[serde(rename = "@objectid")]
    #[xml(rename = "@objectid")]
    pub objectid: usize,
    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "@transform")]
    pub transform: Option<Transform>,
    #[serde(rename = "@partnumber", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "@partnumber")]
    pub partnumber: Option<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            xmlns: "http://schemas.microsoft.com/3dmanufacturing/core/2015/02".to_owned(),
            metadata: Vec::new(),
            resources: Resources::default(),
            build: Build::default(),
            unit: Unit::default(),
        }
    }
}

impl From<Mesh> for Model {
    fn from(mesh: Mesh) -> Self {
        let object = Object {
            id: 1,
            partnumber: None,
            name: None,
            pid: None,
            mesh: Some(mesh),
            components: None,
        };
        let resources = Resources {
            object: vec![object],
            // basematerials: None,
        };
        let build = Build {
            item: vec![Item {
                objectid: 1,
                transform: None,
                partnumber: None,
            }],
        };
        Model {
            resources,
            build,
            ..Default::default()
        }
    }
}
