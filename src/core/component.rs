use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::{core::transform::Transform, threemf_namespaces::CORE_NS};

#[derive(Serialize, Deserialize, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS), rename = "components")]
pub struct Components {
    pub component: Vec<Component>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml, PartialEq, Debug)]
#[xml(ns(CORE_NS), rename = "component")]
pub struct Component {
    #[serde(rename = "@objectid")]
    #[xml(attribute)]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    #[xml(attribute)]
    pub transform: Option<Transform>,
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string};
    use pretty_assertions::assert_eq;

    use crate::{core::transform::Transform, threemf_namespaces::CORE_NS};

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
                ]))
            }
        )
    }

    #[test]
    pub fn toxml_component_test() {
        let xml_string = format!(
            r#"<component xmlns="{}" objectid="3" transform="1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 35.000000 35.000000 5.100000" />"#,
            CORE_NS
        );
        let component = Component {
            objectid: 3,
            transform: Some(Transform([
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.1,
            ])),
        };
        let component_string = to_string(&component).unwrap();

        assert_eq!(component_string, xml_string);
    }

    #[test]
    pub fn toxml_components_test() {
        let xml_string = format!(
            r#"<components xmlns="{}"><component objectid="4" transform="1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 35.000000 35.000000 5.100000" /><component objectid="5" /></components>"#,
            CORE_NS
        );
        let components = Components {
            component: vec![
                Component {
                    objectid: 4,
                    transform: Some(Transform([
                        1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 35.0, 35.0, 5.1,
                    ])),
                },
                Component {
                    objectid: 5,
                    transform: None,
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
                    },
                    Component {
                        objectid: 5,
                        transform: None,
                    },
                ],
            }
        );
    }
}
