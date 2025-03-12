use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::threemf_namespaces::{CORE_NS, PROD_NS};

use super::transform::Transform;

#[derive(Serialize, Deserialize, Default, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS, p=PROD_NS), rename = "build")]
pub struct Build {
    #[xml(attribute, ns(PROD_NS), rename = "UUID")]
    pub uuid: Option<String>,

    #[serde(default)]
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS, p=PROD_NS), rename = "item")]
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

    #[xml(attribute, ns(PROD_NS))]
    pub path: Option<String>,

    #[xml(attribute, ns(PROD_NS), rename = "UUID")]
    pub uuid: Option<String>,
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string};
    use pretty_assertions::assert_eq;

    use crate::{
        core::transform::Transform,
        threemf_namespaces::{CORE_NS, PROD_NS, PROD_PREFIX},
    };

    use super::{Build, Item};

    #[test]
    pub fn toxml_item_test() {
        let xml_string = format!(
            r#"<item xmlns="{}" xmlns:{}="{}" objectid="6" partnumber="part_1" />"#,
            CORE_NS, PROD_PREFIX, PROD_NS
        );
        let item = Item {
            objectid: 6,
            partnumber: Some("part_1".to_string()),
            transform: None,
            path: None,
            uuid: None,
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
                path: None,
                uuid: None,
            }
        );
    }

    #[test]
    pub fn toxml_production_item_test() {
        let xml_string = format!(
            r#"<item xmlns="{}" xmlns:{}="{}" objectid="6" partnumber="part_1" {}:path="//somePath//Item" {}:UUID="someUUID" />"#,
            CORE_NS, PROD_PREFIX, PROD_NS, PROD_PREFIX, PROD_PREFIX
        );
        let item = Item {
            objectid: 6,
            partnumber: Some("part_1".to_string()),
            transform: None,
            path: Some("//somePath//Item".to_owned()),
            uuid: Some("someUUID".to_owned()),
        };
        let item_string = to_string(&item).unwrap();

        assert_eq!(item_string, xml_string);
    }

    #[test]
    pub fn fromxml_production_item_test() {
        const CUSTOM_PROD_PREFIX: &str = "custom";
        let xml_string = format!(
            r#"<item xmlns="{}" xmlns:{}="{}" objectid="6" partnumber="part_1" transform="1 0 0 0 1 0 0 0 1 35 35 5.1" {}:path="//somePath//Item" {}:UUID="someUUID"/>"#,
            CORE_NS, CUSTOM_PROD_PREFIX, PROD_NS, CUSTOM_PROD_PREFIX, CUSTOM_PROD_PREFIX
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
                path: Some("//somePath//Item".to_owned()),
                uuid: Some("someUUID".to_owned()),
            }
        );
    }

    #[test]
    pub fn toxml_build_test() {
        let xml_string = format!(
            r#"<build xmlns="{}" xmlns:{}="{}"><item objectid="6" partnumber="part_1" /><item objectid="6" partnumber="part_2" /></build>"#,
            CORE_NS, PROD_PREFIX, PROD_NS
        );
        let build = Build {
            uuid: None,
            item: vec![
                Item {
                    objectid: 6,
                    partnumber: Some("part_1".to_string()),
                    transform: None,
                    path: None,
                    uuid: None,
                },
                Item {
                    objectid: 6,
                    partnumber: Some("part_2".to_string()),
                    transform: None,
                    path: None,
                    uuid: None,
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
                uuid: None,
                item: vec![
                    Item {
                        objectid: 6,
                        partnumber: Some("part_1".to_string()),
                        transform: None,
                        path: None,
                        uuid: None,
                    },
                    Item {
                        objectid: 6,
                        partnumber: Some("part_2".to_string()),
                        transform: None,
                        path: None,
                        uuid: None,
                    },
                ],
            }
        )
    }

    #[test]
    pub fn toxml_production_build_test() {
        let xml_string = format!(
            r#"<build xmlns="{}" xmlns:{}="{}" {}:UUID="someUUID"><item objectid="6" partnumber="part_1" {}:UUID="someItemUUID1" /><item objectid="6" partnumber="part_2" {}:UUID="someItemUUID2" /></build>"#,
            CORE_NS, PROD_PREFIX, PROD_NS, PROD_PREFIX, PROD_PREFIX, PROD_PREFIX
        );
        let build = Build {
            uuid: Some("someUUID".to_owned()),
            item: vec![
                Item {
                    objectid: 6,
                    partnumber: Some("part_1".to_string()),
                    transform: None,
                    path: None,
                    uuid: Some("someItemUUID1".to_owned()),
                },
                Item {
                    objectid: 6,
                    partnumber: Some("part_2".to_string()),
                    transform: None,
                    path: None,
                    uuid: Some("someItemUUID2".to_owned()),
                },
            ],
        };
        let build_string = to_string(&build).unwrap();

        assert_eq!(build_string, xml_string);
    }

    #[test]
    pub fn fromxml_production_build_test() {
        const CUSTOM_PROD_PREFIX: &str = "custom";
        let xml_string = format!(
            r#"<build xmlns="{}" xmlns:{}="{}" {}:UUID="someBuildUUID"><item objectid="6" partnumber="part_1" {}:UUID="someItemUUID1" /><item objectid="6" partnumber="part_2" {}:UUID="someItemUUID2" /></build>"#,
            CORE_NS,
            CUSTOM_PROD_PREFIX,
            PROD_NS,
            CUSTOM_PROD_PREFIX,
            CUSTOM_PROD_PREFIX,
            CUSTOM_PROD_PREFIX
        );
        let build_string = from_str::<Build>(&xml_string).unwrap();

        assert_eq!(
            build_string,
            Build {
                uuid: Some("someBuildUUID".to_owned()),
                item: vec![
                    Item {
                        objectid: 6,
                        partnumber: Some("part_1".to_string()),
                        transform: None,
                        path: None,
                        uuid: Some("someItemUUID1".to_owned()),
                    },
                    Item {
                        objectid: 6,
                        partnumber: Some("part_2".to_string()),
                        transform: None,
                        path: None,
                        uuid: Some("someItemUUID2".to_owned()),
                    },
                ],
            }
        )
    }
}
