use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::threemf_namespaces::CORE_NS;

use super::transform::Transform;

#[derive(Serialize, Deserialize, Default, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS), rename = "build")]
pub struct Build {
    #[serde(default)]
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS), rename = "item")]
pub struct Item {
    #[serde(rename = "@objectid")]
    #[xml(attribute)]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub transform: Option<Transform>,

    #[serde(rename = "@partnumber", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub partnumber: Option<String>,
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string};
    use pretty_assertions::assert_eq;

    use crate::{core::transform::Transform, threemf_namespaces::CORE_NS};

    use super::{Build, Item};

    #[test]
    pub fn toxml_item_test() {
        let xml_string = format!(
            r#"<item xmlns="{}" objectid="6" partnumber="part_1" />"#,
            CORE_NS
        );
        let item = Item {
            objectid: 6,
            partnumber: Some("part_1".to_string()),
            transform: None,
        };
        let item_string = to_string(&item).unwrap();

        assert_eq!(item_string, xml_string);
    }

    #[test]
    pub fn fromxml_item_test() {
        let xml_string = format!(
            r#"<item xmlns="{}" objectid="6" partnumber="part_1" transform="1 0 0 0 1 0 0 0 1 35 35 5.1"/>"#,
            CORE_NS
        );
        let item = from_str::<Item>(&xml_string).unwrap();

        assert_eq!(
            item,
            Item {
                objectid: 6,
                partnumber: Some("part_1".to_string()),
                transform: Some(Transform([
                    1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.1
                ])),
            }
        );
    }

    #[test]
    pub fn toxml_build_test() {
        let xml_string = format!(
            r#"<build xmlns="{}"><item objectid="6" partnumber="part_1" /><item objectid="6" partnumber="part_2" /></build>"#,
            CORE_NS
        );
        let build = Build {
            item: vec![
                Item {
                    objectid: 6,
                    partnumber: Some("part_1".to_string()),
                    transform: None,
                },
                Item {
                    objectid: 6,
                    partnumber: Some("part_2".to_string()),
                    transform: None,
                },
            ],
        };
        let build_string = to_string(&build).unwrap();

        assert_eq!(build_string, xml_string);
    }

    #[test]
    pub fn fromxml_build_test() {
        let xml_string = format!(
            r#"<build xmlns="{}"><item objectid="6" partnumber="part_1" /><item objectid="6" partnumber="part_2" /></build>"#,
            CORE_NS
        );
        let build_string = from_str::<Build>(&xml_string).unwrap();

        assert_eq!(
            build_string,
            Build {
                item: vec![
                    Item {
                        objectid: 6,
                        partnumber: Some("part_1".to_string()),
                        transform: None,
                    },
                    Item {
                        objectid: 6,
                        partnumber: Some("part_2".to_string()),
                        transform: None,
                    },
                ],
            }
        )
    }
}
