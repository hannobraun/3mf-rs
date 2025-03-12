use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::{
    core::{component::Components, Mesh},
    threemf_namespaces::{CORE_NS, PROD_NS},
};

#[derive(Serialize, Deserialize, FromXml, ToXml, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[xml(ns(CORE_NS, p=PROD_NS), rename="object")]
pub struct Object {
    #[serde(rename = "@id")]
    #[xml(attribute)]
    pub id: usize,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "type", attribute)]
    pub objecttype: Option<ObjectType>,

    #[serde(rename = "@thumbnail", skip_serializing_if = "Option::is_none")]
    #[xml(rename = "thumbnail", attribute)]
    pub thumbnail: Option<String>,

    #[serde(rename = "@partnumber", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub partnumber: Option<String>,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub name: Option<String>,

    #[serde(rename = "@pid", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub pid: Option<usize>,

    #[serde(rename = "@pindex", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub pindex: Option<usize>,

    #[xml(attribute, ns(PROD_NS), rename = "UUID")]
    pub uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Mesh>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

#[derive(Debug, Deserialize, Serialize, ToXml, FromXml, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[xml(scalar, rename_all = "lowercase")]
pub enum ObjectType {
    #[default]
    Model,
    Support,
    SolidSupport,
    Surface,
    Other,
}

#[cfg(test)]
pub mod test {
    use std::vec;

    use instant_xml::{from_str, to_string, FromXml, ToXml};
    use pretty_assertions::assert_eq;

    use crate::{
        core::{
            component::{Component, Components},
            Triangles, Vertices,
        },
        threemf_namespaces::{CORE_NS, PROD_NS, PROD_PREFIX},
        Mesh,
    };

    use super::{Object, ObjectType};

    #[test]
    pub fn toxml_simple_object_test() {
        let xml_string = format!(
            r#"<object xmlns="{}" xmlns:{}="{}" id="4"></object>"#,
            CORE_NS, PROD_PREFIX, PROD_NS
        );
        let object = Object {
            id: 4,
            objecttype: None,
            thumbnail: None,
            partnumber: None,
            name: None,
            pid: None,
            pindex: None,
            uuid: None,
            mesh: None,
            components: None,
        };
        let object_string = to_string(&object).unwrap();

        assert_eq!(object_string, xml_string);
    }

    #[test]
    pub fn fromxml_simple_object_test() {
        let xml_string = format!(r#"<object xmlns="{}" id="4"></object>"#, CORE_NS);
        let object = from_str::<Object>(&xml_string).unwrap();

        assert_eq!(
            object,
            Object {
                id: 4,
                objecttype: None,
                thumbnail: None,
                partnumber: None,
                name: None,
                pid: None,
                pindex: None,
                uuid: None,
                mesh: None,
                components: None,
            }
        );
    }

    #[test]
    pub fn toxml_production_object_test() {
        let xml_string = format!(
            r#"<object xmlns="{}" xmlns:{}="{}" id="4" {}:UUID="someUUID"></object>"#,
            CORE_NS, PROD_PREFIX, PROD_NS, PROD_PREFIX
        );
        let object = Object {
            id: 4,
            objecttype: None,
            thumbnail: None,
            partnumber: None,
            name: None,
            pid: None,
            pindex: None,
            uuid: Some("someUUID".to_owned()),
            mesh: None,
            components: None,
        };
        let object_string = to_string(&object).unwrap();

        assert_eq!(object_string, xml_string);
    }

    #[test]
    pub fn fromxml_production_object_test() {
        const CUSTOM_PROD_PREFIX: &str = "custom";
        let xml_string = format!(
            r#"<object xmlns="{}" xmlns:{}="{}" id="4" {}:UUID="someUUID"></object>"#,
            CORE_NS, CUSTOM_PROD_PREFIX, PROD_NS, CUSTOM_PROD_PREFIX,
        );
        let object = from_str::<Object>(&xml_string).unwrap();

        assert_eq!(
            object,
            Object {
                id: 4,
                objecttype: None,
                thumbnail: None,
                partnumber: None,
                name: None,
                pid: None,
                pindex: None,
                uuid: Some("someUUID".to_owned()),
                mesh: None,
                components: None,
            }
        );
    }

    #[test]
    pub fn toxml_intermediate_object_test() {
        let xml_string = format!(
            r#"<object xmlns="{}" xmlns:{}="{}" id="4" type="model" thumbnail="\thumbnail\part_thumbnail.png" partnumber="part_1" name="Object Part"></object>"#,
            CORE_NS, PROD_PREFIX, PROD_NS
        );
        let object = Object {
            id: 4,
            objecttype: Some(ObjectType::Model),
            thumbnail: Some("\\thumbnail\\part_thumbnail.png".to_string()),
            partnumber: Some("part_1".to_string()),
            name: Some("Object Part".to_string()),
            pid: None,
            pindex: None,
            uuid: None,
            mesh: None,
            components: None,
        };
        let object_string = to_string(&object).unwrap();
        println!("{}", object_string);

        assert_eq!(object_string, xml_string);
    }

    #[test]
    pub fn fromxml_intermediate_object_test() {
        let xml_string = format!(
            r#"<object xmlns="{}" id="4" type="model" thumbnail="\thumbnail\part_thumbnail.png" partnumber="part_1" name="Object Part" pid="123" pindex="123"></object>"#,
            CORE_NS
        );
        let object = from_str::<Object>(&xml_string).unwrap();

        assert_eq!(
            object,
            Object {
                id: 4,
                objecttype: Some(ObjectType::Model),
                thumbnail: Some("\\thumbnail\\part_thumbnail.png".to_string()),
                partnumber: Some("part_1".to_string()),
                name: Some("Object Part".to_string()),
                pid: Some(123),
                pindex: Some(123),
                uuid: None,
                mesh: None,
                components: None,
            }
        );
    }

    #[test]
    pub fn roundtrip_advanced_mesh_object_test() {
        let xml_string = format!(
            r##"<object xmlns="{}" xmlns:{}="{}" id="4" type="model" thumbnail="\thumbnail\part_thumbnail.png" partnumber="part_1" name="Object Part"><mesh><vertices></vertices><triangles></triangles></mesh></object>"##,
            CORE_NS, PROD_PREFIX, PROD_NS
        );
        let object = Object {
            id: 4,
            objecttype: Some(ObjectType::Model),
            thumbnail: Some("\\thumbnail\\part_thumbnail.png".to_string()),
            partnumber: Some("part_1".to_string()),
            name: Some("Object Part".to_string()),
            pid: None,
            pindex: None,
            uuid: None,
            mesh: Some(Mesh {
                vertices: Vertices { vertex: vec![] },
                triangles: Triangles { triangle: vec![] },
            }),
            components: None,
        };
        let object_string = to_string(&object).unwrap();
        let roundtrip_object = from_str::<Object>(&object_string).unwrap();

        assert_eq!(object_string, xml_string);
        assert_eq!(roundtrip_object, object);
    }

    #[test]
    pub fn roundtrip_advanced_component_object_test() {
        let xml_string = format!(
            r##"<object xmlns="{}" xmlns:{}="{}" id="4" type="model" thumbnail="\thumbnail\part_thumbnail.png" partnumber="part_1" name="Object Part"><components><component objectid="23" /></components></object>"##,
            CORE_NS, PROD_PREFIX, PROD_NS
        );
        let object = Object {
            id: 4,
            objecttype: Some(ObjectType::Model),
            thumbnail: Some("\\thumbnail\\part_thumbnail.png".to_string()),
            partnumber: Some("part_1".to_string()),
            name: Some("Object Part".to_string()),
            pid: None,
            pindex: None,
            uuid: None,
            mesh: None,
            components: Some(Components {
                component: vec![Component {
                    objectid: 23,
                    transform: None,
                    path: None,
                    uuid: None,
                }],
            }),
        };
        let object_string = to_string(&object).unwrap();
        let roundtrip_object = from_str::<Object>(&object_string).unwrap();

        assert_eq!(object_string, xml_string);
        assert_eq!(roundtrip_object, object);
    }

    #[derive(Debug, ToXml, FromXml)]
    pub struct ObjectTypes {
        objecttype: Vec<ObjectType>,
    }

    #[test]
    pub fn toxml_objecttype_test() {
        let xml_string = format!(
            r#"<ObjectTypes><objecttype>model</objecttype><objecttype>support</objecttype><objecttype>solidsupport</objecttype><objecttype>support</objecttype><objecttype>other</objecttype></ObjectTypes>"#,
        );
        let objecttypes = ObjectTypes {
            objecttype: vec![
                ObjectType::Model,
                ObjectType::Support,
                ObjectType::SolidSupport,
                ObjectType::Support,
                ObjectType::Other,
            ],
        };
        let objecttype_string = to_string(&objecttypes).unwrap();

        assert_eq!(objecttype_string, xml_string);
    }
}
