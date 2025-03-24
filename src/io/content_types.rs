use instant_xml::{FromXml, ToXml};

/// Content types for the Open Packaging Conventions (OPC).
/// Contains a collection of [DefaultContentTypes].
/// [DefaultContentTypes] contains the [DefaultContetnTypeEnum] specifying the content type.
/// [DefaultContentTypes] contains the file extension that is used for the specified content type.
#[derive(ToXml, FromXml, Debug, PartialEq, Eq)]
#[xml(ns(CONTENT_TYPES_NS), rename = "Types")]
pub struct ContentTypes {
    pub defaults: Vec<DefaultContentTypes>,
}

/// Predefined content types supported by [threemf::io] currently.
/// If a content type is not found, it will fail the 3mf file parsing.
#[derive(ToXml, FromXml, Debug, PartialEq, Eq)]
#[xml(scalar)]
pub enum DefaultContentTypeEnum {
    /// Represents a relationship content.
    #[xml(rename = "application/vnd.openxmlformats-package.relationships+xml")]
    Relationship,

    /// Represents a 3D model content.
    #[xml(rename = "application/vnd.ms-package.3dmanufacturing-3dmodel+xml")]
    Model,

    /// Represents a PNG image content.
    #[xml(rename = "image/png")]
    ImagePng,
}

/// Internal structure for serde of [ContentTypes].
#[derive(ToXml, FromXml, Debug, PartialEq, Eq)]
#[xml(ns(CONTENT_TYPES_NS), rename = "Default")]
pub struct DefaultContentTypes {
    #[xml(attribute, rename = "Extension")]
    pub extension: String,

    #[xml(attribute, rename = "ContentType")]
    pub content_type: DefaultContentTypeEnum,
}

const CONTENT_TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string};
    use pretty_assertions::assert_eq;

    use crate::io::content_types::CONTENT_TYPES_NS;

    use super::{ContentTypes, DefaultContentTypeEnum, DefaultContentTypes};

    const RELS_CONTENT_NS: &str = "application/vnd.openxmlformats-package.relationships+xml";
    const MODEL_CONTENT_NS: &str = "application/vnd.ms-package.3dmanufacturing-3dmodel+xml";
    const PNG_CONTENT_NS: &str = "image/png";

    #[test]
    pub fn toxml_content_types_test() {
        let xml_string = format!(
            r#"<{a} xmlns="{b}"><Default Extension="rels" ContentType="{RELS_CONTENT_NS}" /><Default Extension="model" ContentType="{MODEL_CONTENT_NS}" /><Default Extension="png" ContentType="{PNG_CONTENT_NS}" /></{a}>"#,
            a = "Types",
            b = CONTENT_TYPES_NS,
        );
        let content = ContentTypes {
            defaults: vec![
                DefaultContentTypes {
                    extension: "rels".to_owned(),
                    content_type: DefaultContentTypeEnum::Relationship,
                },
                DefaultContentTypes {
                    extension: "model".to_owned(),
                    content_type: DefaultContentTypeEnum::Model,
                },
                DefaultContentTypes {
                    extension: "png".to_owned(),
                    content_type: DefaultContentTypeEnum::ImagePng,
                },
            ],
        };
        let content_string = to_string(&content).unwrap();

        assert_eq!(content_string, xml_string);
    }

    #[test]
    pub fn fromxml_content_types_test() {
        let xml_string = format!(
            r#"<{a} xmlns="{b}"><Default Extension="rels" ContentType="{RELS_CONTENT_NS}"/><Default Extension="model" ContentType="{MODEL_CONTENT_NS}"/><Default Extension="png" ContentType="{PNG_CONTENT_NS}"/></{a}>"#,
            a = "Types",
            b = CONTENT_TYPES_NS,
        );
        let content = from_str::<ContentTypes>(&xml_string).unwrap();

        assert_eq!(
            content,
            ContentTypes {
                defaults: vec![
                    DefaultContentTypes {
                        extension: "rels".to_owned(),
                        content_type: DefaultContentTypeEnum::Relationship,
                    },
                    DefaultContentTypes {
                        extension: "model".to_owned(),
                        content_type: DefaultContentTypeEnum::Model,
                    },
                    DefaultContentTypes {
                        extension: "png".to_owned(),
                        content_type: DefaultContentTypeEnum::ImagePng,
                    },
                ],
            }
        );
    }

    #[test]
    pub fn fromxml_unknown_content_types_test() {
        let xml_string = format!(
            r#"<{a} xmlns="{b}"><Default Extension="rels" ContentType="{RELS_CONTENT_NS}"/><Default Extension="model" ContentType="{MODEL_CONTENT_NS}"/><Default Extension="unknown" ContentType="some/unknown/content"/></{a}>"#,
            a = "Types",
            b = CONTENT_TYPES_NS,
        );
        let content = from_str::<ContentTypes>(&xml_string);

        assert_eq!(
            content,
            Err(instant_xml::Error::UnexpectedValue("enum variant not found for 'some/unknown/content' in field DefaultContentTypes::content_type".to_owned()))
        );
    }
}
