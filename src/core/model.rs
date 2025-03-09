use std::vec;

use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        build::{Build, Item},
        metadata::Metadata,
        object::Object,
        resources::Resources,
        Mesh,
    },
    threemf_namespaces::CORE_NS,
};

#[derive(Serialize, Deserialize, FromXml, ToXml, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[xml(ns(CORE_NS), rename = "model")]
pub struct Model {
    #[serde(rename = "@xmlns", default)]
    #[xml(attribute)]
    pub xmlns: Option<String>,

    #[serde(rename = "@unit", default)]
    #[xml(attribute)]
    pub unit: Unit,

    #[serde(rename = "@requiredextensions", default)]
    #[xml(attribute)]
    pub requiredextensions: Option<String>,

    #[serde(rename = "@recommendedextensions", default)]
    #[xml(attribute)]
    pub recommendedextensions: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Metadata>,

    pub resources: Resources,

    pub build: Build,
}

/// Model measurement unit, default is millimeter
#[derive(Serialize, Deserialize, FromXml, ToXml, Default, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[xml(scalar, rename_all = "lowercase")]
pub enum Unit {
    Micron,
    #[default]
    Millimeter,
    Centimeter,
    Inch,
    Foot,
    Meter,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            xmlns: Some(CORE_NS.to_owned()),
            requiredextensions: None,
            recommendedextensions: None,
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
            objecttype: None,
            thumbnail: None,
            partnumber: None,
            name: None,
            pid: None,
            pindex: None,
            mesh: Some(mesh),
            components: None,
        };
        let resources = Resources {
            object: vec![object],
            basematerials: vec![],
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

#[cfg(test)]
pub mod test {
    use instant_xml::{from_str, to_string, FromXml, ToXml};
    use pretty_assertions::assert_eq;

    use crate::{
        core::{
            build::{Build, Item},
            metadata::Metadata,
            object::{Object, ObjectType},
            resources::Resources,
        },
        threemf_namespaces::CORE_NS,
    };

    use super::{Model, Unit};

    #[test]
    pub fn toxml_simple_model_test() {
        let xml_string = format!(
            r#"<model xmlns="{}" unit="millimeter"><metadata name="Trial Metadata" /><resources><object id="346" type="model" name="test part"></object></resources><build><item objectid="346" /></build></model>"#,
            CORE_NS
        );
        let model = Model {
            xmlns: None,
            unit: Unit::Millimeter,
            requiredextensions: None,
            recommendedextensions: None,
            metadata: vec![Metadata {
                name: "Trial Metadata".to_owned(),
                value: None,
            }],
            resources: Resources {
                basematerials: vec![],
                object: vec![Object {
                    id: 346,
                    objecttype: Some(ObjectType::Model),
                    thumbnail: None,
                    partnumber: None,
                    name: Some("test part".to_owned()),
                    pid: None,
                    pindex: None,
                    mesh: None,
                    components: None,
                }],
            },
            build: Build {
                item: vec![Item {
                    objectid: 346,
                    transform: None,
                    partnumber: None,
                }],
            },
        };
        let model_string = to_string(&model).unwrap();

        assert_eq!(model_string, xml_string);
    }

    #[test]
    pub fn fromxml_simple_model_test() {
        let xml_string = format!(
            r#"<model xmlns="{}" xml:lang="en-us" unit="millimeter"><metadata name="Trial Metadata" /><resources><object id="346" type="model" name="test part"></object></resources><build><item objectid="346" /></build></model>"#,
            CORE_NS
        );
        let model = from_str::<Model>(&xml_string).unwrap();

        assert_eq!(
            model,
            Model {
                xmlns: None,
                unit: Unit::Millimeter,
                requiredextensions: None,
                recommendedextensions: None,
                metadata: vec![Metadata {
                    name: "Trial Metadata".to_owned(),
                    value: None,
                }],
                resources: Resources {
                    basematerials: vec![],
                    object: vec![Object {
                        id: 346,
                        objecttype: Some(ObjectType::Model),
                        thumbnail: None,
                        partnumber: None,
                        name: Some("test part".to_owned()),
                        pid: None,
                        pindex: None,
                        mesh: None,
                        components: None,
                    }],
                },
                build: Build {
                    item: vec![Item {
                        objectid: 346,
                        transform: None,
                        partnumber: None,
                    }],
                },
            }
        );
    }

    #[derive(Debug, ToXml, FromXml, PartialEq, Eq)]
    struct UnitsVector {
        unit: Vec<Unit>,
    }

    #[test]
    pub fn toxml_units_test() {
        let xml_string = "<UnitsVector><unit>micron</unit><unit>millimeter</unit><unit>centimeter</unit><unit>inch</unit><unit>foot</unit><unit>meter</unit></UnitsVector>";
        let unitsvector = UnitsVector {
            unit: vec![
                Unit::Micron,
                Unit::Millimeter,
                Unit::Centimeter,
                Unit::Inch,
                Unit::Foot,
                Unit::Meter,
            ],
        };
        let unitsvector_string = to_string(&unitsvector).unwrap();

        assert_eq!(unitsvector_string, xml_string);
    }

    #[test]
    pub fn fromxml_units_test() {
        let xml_string = r#"<UnitsVector><unit>micron</unit><unit>millimeter</unit><unit>centimeter</unit><unit>inch</unit><unit>foot</unit><unit>meter</unit></UnitsVector>"#;
        let unitsvector = from_str::<UnitsVector>(&xml_string).unwrap();

        assert_eq!(
            unitsvector,
            UnitsVector {
                unit: vec![
                    Unit::Micron,
                    Unit::Millimeter,
                    Unit::Centimeter,
                    Unit::Inch,
                    Unit::Foot,
                    Unit::Meter,
                ],
            }
        );
    }
}
