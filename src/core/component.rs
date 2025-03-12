use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::{
    core::transform::Transform,
    threemf_namespaces::{CORE_NS, PROD_NS},
};

#[derive(Serialize, Deserialize, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS), rename = "components")]
pub struct Components {
    pub component: Vec<Component>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS, p=PROD_NS), rename = "component")]
pub struct Component {
    #[serde(rename = "@objectid")]
    #[xml(attribute)]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub transform: Option<Transform>,

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

    use super::{Component, Components};

    #[test]
    pub fn fromxml_component_test() {
        let xml_string = format!(
            r#"<component xmlns="{}" objectid="3" transform="1.0000 0.0000 0.0000 0.0000 1.0000 0.0000 0.0000 0.0000 1.0000 35 35 5.1000" />"#,
            CORE_NS
        );
        let component = from_str::<Component>(&xml_string).unwrap();

        assert_eq!(
            component,
            Component {
                objectid: 3,
                transform: Some(Transform([
                    1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.10
                ])),
                path: None,
                uuid: None,
            }
        )
    }

    #[test]
    pub fn toxml_component_test() {
        let xml_string = format!(
            r#"<component xmlns="{}" xmlns:{}="{}" objectid="3" transform="1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 35.000000 35.000000 5.100000" />"#,
            CORE_NS, PROD_PREFIX, PROD_NS
        );
        let component = Component {
            objectid: 3,
            transform: Some(Transform([
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.1,
            ])),
            path: None,
            uuid: None,
        };
        let component_string = to_string(&component).unwrap();

        assert_eq!(component_string, xml_string);
    }

    #[test]
    pub fn fromxml_production_component_test() {
        const CUSTOM_PROD_PREFIX: &str = "custom";
        let xml_string = format!(
            r#"<component xmlns="{}" xmlns:{}="{}" objectid="3" transform="1.0000 0.0000 0.0000 0.0000 1.0000 0.0000 0.0000 0.0000 1.0000 35 35 5.1000" {}:path="//somePath//Component" {}:UUID="someComponentUUID" />"#,
            CORE_NS, CUSTOM_PROD_PREFIX, PROD_NS, CUSTOM_PROD_PREFIX, CUSTOM_PROD_PREFIX
        );
        let component = from_str::<Component>(&xml_string).unwrap();

        assert_eq!(
            component,
            Component {
                objectid: 3,
                transform: Some(Transform([
                    1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.10
                ])),
                path: Some("//somePath//Component".to_owned()),
                uuid: Some("someComponentUUID".to_owned()),
            }
        )
    }

    #[test]
    pub fn toxml_production_component_test() {
        let xml_string = format!(
            r#"<component xmlns="{}" xmlns:{}="{}" objectid="3" transform="1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 35.000000 35.000000 5.100000" {}:path="//somePath//Component" {}:UUID="someComponentUUID" />"#,
            CORE_NS, PROD_PREFIX, PROD_NS, PROD_PREFIX, PROD_PREFIX
        );
        let component = Component {
            objectid: 3,
            transform: Some(Transform([
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.1,
            ])),
            path: Some("//somePath//Component".to_owned()),
            uuid: Some("someComponentUUID".to_owned()),
        };
        let component_string = to_string(&component).unwrap();

        assert_eq!(component_string, xml_string);
    }

    #[test]
    pub fn toxml_components_test() {
        let xml_string = format!(
            r#"<components xmlns="{}"><component xmlns:{}="{}" objectid="4" transform="1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 35.000000 35.000000 5.100000" /><component xmlns:{}="{}" objectid="5" /></components>"#,
            CORE_NS, PROD_PREFIX, PROD_NS, PROD_PREFIX, PROD_NS
        );
        let components = Components {
            component: vec![
                Component {
                    objectid: 4,
                    transform: Some(Transform([
                        1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.1,
                    ])),
                    path: None,
                    uuid: None,
                },
                Component {
                    objectid: 5,
                    transform: None,
                    path: None,
                    uuid: None,
                },
            ],
        };
        let components_string = to_string(&components).unwrap();

        assert_eq!(components_string, xml_string);
    }

    #[test]
    pub fn fromxml_components_test() {
        let xml_string = format!(
            r#"<components xmlns="{}"><component objectid="4" transform="1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 35.000000 35.000000 5.100000" /><component objectid="5" /></components>"#,
            CORE_NS
        );
        let components = from_str::<Components>(&xml_string).unwrap();

        assert_eq!(
            components,
            Components {
                component: vec![
                    Component {
                        objectid: 4,
                        transform: Some(Transform([
                            1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.1,
                        ])),
                        path: None,
                        uuid: None,
                    },
                    Component {
                        objectid: 5,
                        transform: None,
                        path: None,
                        uuid: None,
                    },
                ],
            }
        );
    }
}
