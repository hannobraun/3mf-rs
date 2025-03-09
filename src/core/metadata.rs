use instant_xml::{FromXml, ToXml};
use serde::{Deserialize, Serialize};

use crate::threemf_namespaces::CORE_NS;

//ToDo: Add additional optional fields on Metadata
#[derive(Serialize, Deserialize, FromXml, ToXml, Debug, PartialEq, Eq)]
#[xml(ns(CORE_NS), rename = "metadata")]
pub struct Metadata {
    #[serde(rename = "@name")]
    #[xml(attribute)]
    pub name: String,

    #[serde(rename = "$value")]
    #[xml(direct)]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToXml, FromXml, PartialEq, Eq)]
#[xml(ns(CORE_NS), rename = "metadatagroup")]
pub struct MetadataGroup {
    pub metadata: Vec<Metadata>,
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string};
    use pretty_assertions::assert_eq;

    use crate::threemf_namespaces::CORE_NS;

    use super::{Metadata, MetadataGroup};

    #[test]
    pub fn fromxml_metadata_test() {
        let xml_string = format!(
            r#"<metadata xmlns="{}" name="Copyright">Copyright (c) 2018 3MF Consortium. All rights reserved.</metadata>"#,
            CORE_NS
        );
        let metadata = from_str::<Metadata>(&xml_string).unwrap();

        assert_eq!(
            metadata,
            Metadata {
                name: "Copyright".to_string(),
                value: Some("Copyright (c) 2018 3MF Consortium. All rights reserved.".to_string())
            }
        )
    }

    #[test]
    pub fn toxml_metadata_test() {
        let xml_string = format!(
            r#"<metadata xmlns="{}" name="Copyright">Copyright (c) 2018 3MF Consortium. All rights reserved.</metadata>"#,
            CORE_NS
        );
        let metadata = Metadata {
            name: "Copyright".to_string(),
            value: Some("Copyright (c) 2018 3MF Consortium. All rights reserved.".to_string()),
        };
        let metadata_string = to_string(&metadata).unwrap();

        assert_eq!(metadata_string, xml_string);
    }

    #[test]
    pub fn fromxml_simple_metadata_test() {
        let xml_string = format!(r#"<metadata xmlns="{}" name="From Test"/>"#, CORE_NS);
        let metadata = from_str::<Metadata>(&xml_string).unwrap();

        assert_eq!(
            metadata,
            Metadata {
                name: "From Test".to_string(),
                value: None,
            }
        )
    }

    #[test]
    pub fn toxml_simple_metadata_test() {
        let xml_string = format!(r#"<metadata xmlns="{}" name="From Test" />"#, CORE_NS);
        let metadata = Metadata {
            name: "From Test".to_string(),
            value: None,
        };
        let metadata_string = to_string(&metadata).unwrap();

        assert_eq!(metadata_string, xml_string);
    }

    #[test]
    pub fn fromxml_metadatagroup_test() {
        let xml_string = format!(
            r#"<metadatagroup xmlns="{}"><metadata name="From Test"></metadata><metadata name="From Test 2"></metadata></metadatagroup>"#,
            CORE_NS
        );
        let metadatagroup = from_str::<MetadataGroup>(&xml_string).unwrap();

        assert_eq!(
            metadatagroup,
            MetadataGroup {
                metadata: vec![
                    Metadata {
                        name: "From Test".to_string(),
                        value: None,
                    },
                    Metadata {
                        name: "From Test 2".to_string(),
                        value: None,
                    }
                ]
            }
        )
    }

    #[test]
    pub fn toxml_metadatagroup_test() {
        let xml_string = format!(
            r#"<metadatagroup xmlns="{}"><metadata name="From Test"></metadata><metadata name="From Test 2"></metadata></metadatagroup>"#,
            CORE_NS
        );
        let metadatagroup = MetadataGroup {
            metadata: vec![
                Metadata {
                    name: "From Test".to_string(),
                    value: Some("".to_string()),
                },
                Metadata {
                    name: "From Test 2".to_string(),
                    value: Some("".to_string()),
                },
            ],
        };
        let metadatagroup_string = to_string(&metadatagroup).unwrap();

        assert_eq!(metadatagroup_string, xml_string);
    }
}
