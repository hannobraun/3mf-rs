use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::{core::object::Object, threemf_namespaces::CORE_NS};

#[derive(Serialize, Deserialize, FromXml, ToXml, Default, PartialEq, Debug)]
#[xml(ns(CORE_NS), rename = "resources")]
pub struct Resources {
    #[serde(default)]
    pub object: Vec<Object>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basematerials: Vec<BaseMaterials>,
}

#[derive(Serialize, Deserialize, FromXml, ToXml, Default, PartialEq, Eq, Debug)]
#[xml(ns(CORE_NS), rename = "base")]
pub struct Base {
    #[xml(attribute)]
    pub name: String,

    #[xml(attribute)]
    pub displaycolor: String, //ToDo: Make this a specific color struct for flexibility
}

#[derive(Serialize, Deserialize, FromXml, ToXml, Default, Debug, PartialEq, Eq)]
#[xml(ns(CORE_NS), rename = "basematerials")]
pub struct BaseMaterials {
    #[xml(attribute)]
    pub id: usize,

    pub base: Vec<Base>,
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string};
    use pretty_assertions::assert_eq;

    use crate::{core::object::Object, threemf_namespaces::CORE_NS};

    use super::{Base, BaseMaterials, Resources};

    #[test]
    pub fn toxml_resources_with_object_test() {
        let xml_string = format!(
            r#"<resources xmlns="{}"><object id="1"></object></resources>"#,
            CORE_NS
        );
        let resources = Resources {
            object: vec![Object {
                id: 1,
                objecttype: None,
                thumbnail: None,
                partnumber: None,
                name: None,
                pid: None,
                pindex: None,
                mesh: None,
                components: None,
            }],
            basematerials: vec![],
        };
        let resources_string = to_string(&resources).unwrap();

        assert_eq!(resources_string, xml_string);
    }

    #[test]
    pub fn fromxml_resources_with_object_test() {
        let xml_string = format!(
            r#"<resources xmlns="{}"><object id="1"></object></resources>"#,
            CORE_NS
        );
        let resources = from_str::<Resources>(&xml_string).unwrap();

        assert_eq!(
            resources,
            Resources {
                object: vec![Object {
                    id: 1,
                    objecttype: None,
                    thumbnail: None,
                    partnumber: None,
                    name: None,
                    pid: None,
                    pindex: None,
                    mesh: None,
                    components: None,
                }],
                basematerials: vec![],
            }
        );
    }

    #[test]
    pub fn toxml_resources_with_basematerials_test() {
        let xml_string = format!(
            r##"<resources xmlns="{}"><basematerials id="1"><base name="Base" displaycolor="#FEFEFE00" /></basematerials></resources>"##,
            CORE_NS
        );
        let resources = Resources {
            object: vec![],
            basematerials: vec![BaseMaterials {
                id: 1,
                base: vec![Base {
                    name: "Base".to_owned(),
                    displaycolor: "#FEFEFE00".to_owned(),
                }],
            }],
        };
        let resources_string = to_string(&resources).unwrap();

        assert_eq!(resources_string, xml_string);
    }

    #[test]
    pub fn fromxml_resources_with_basematerials_test() {
        let xml_string = format!(
            r##"<resources xmlns="{}"><basematerials id="1"><base name="Base" displaycolor="#FEFEFE00" /></basematerials></resources>"##,
            CORE_NS
        );
        let resources = from_str::<Resources>(&xml_string).unwrap();

        assert_eq!(
            resources,
            Resources {
                object: vec![],
                basematerials: vec![BaseMaterials {
                    id: 1,
                    base: vec![Base {
                        name: "Base".to_owned(),
                        displaycolor: "#FEFEFE00".to_owned(),
                    }],
                }],
            }
        );
    }

    #[test]
    pub fn toxml_base_test() {
        let xml_string = format!(
            r##"<base xmlns="{}" name="Base" displaycolor="#FEF100" />"##,
            CORE_NS
        );
        let base = Base {
            name: "Base".to_string(),
            displaycolor: "#FEF100".to_string(),
        };
        let base_string = to_string(&base).unwrap();

        assert_eq!(base_string, xml_string);
    }

    #[test]
    pub fn fromxml_base_test() {
        let xml_string = format!(
            r##"<base xmlns="{}" name="Base" displaycolor="#FEF100" />"##,
            CORE_NS
        );
        let base = from_str::<Base>(&xml_string).unwrap();

        assert_eq!(
            base,
            Base {
                name: "Base".to_string(),
                displaycolor: "#FEF100".to_string(),
            }
        );
    }

    #[test]
    pub fn toxml_basematerials_test() {
        let xml_string = format!(
            r##"<basematerials xmlns="{}" id="256"><base name="Base 1" displaycolor="#FEF100" /><base name="Base 2" displaycolor="#FEF369" /></basematerials>"##,
            CORE_NS
        );
        let basematerials = BaseMaterials {
            id: 256,
            base: vec![
                Base {
                    name: "Base 1".to_string(),
                    displaycolor: "#FEF100".to_string(),
                },
                Base {
                    name: "Base 2".to_string(),
                    displaycolor: "#FEF369".to_string(),
                },
            ],
        };
        let base_string = to_string(&basematerials).unwrap();

        assert_eq!(base_string, xml_string);
    }

    #[test]
    pub fn fromxml_basematerials_test() {
        let xml_string = format!(
            r##"<basematerials xmlns="{}" id="256"><base name="Base 1" displaycolor="#FEF100" /><base name="Base 2" displaycolor="#FEF369" /></basematerials>"##,
            CORE_NS
        );
        let base = from_str::<BaseMaterials>(&xml_string).unwrap();

        assert_eq!(
            base,
            BaseMaterials {
                id: 256,
                base: vec![
                    Base {
                        name: "Base 1".to_string(),
                        displaycolor: "#FEF100".to_string(),
                    },
                    Base {
                        name: "Base 2".to_string(),
                        displaycolor: "#FEF369".to_string(),
                    },
                ],
            }
        );
    }
}
