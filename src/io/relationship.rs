use instant_xml::{FromXml, ToXml};

const RELATIONSHIP_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

/// Represents a relationship of a single part in the 3mf package along with its [RelationshipType]
/// and target path of the part in the archive.
#[derive(ToXml, FromXml, Debug, Clone, PartialEq, Eq)]
#[xml(ns(RELATIONSHIP_NS))]
pub struct Relationship {
    /// The unique identifier of the relationship.
    #[xml(attribute, rename = "Id")]
    pub id: String,

    /// Target path of the part in the archive.
    #[xml(attribute, rename = "Target")]
    pub target: String,

    /// The actual relationship of the target part
    #[xml(attribute, rename = "Type")]
    pub relationship_type: RelationshipType,
}

/// Represents a collection of [Relationship]s where each collection is an independent
/// relationship part in the 3mf package. A single 3mf package may contain multiple [Relationships].
#[derive(ToXml, FromXml, Debug, Clone, PartialEq, Eq)]
#[xml(ns(RELATIONSHIP_NS))]
pub struct Relationships {
    pub relationships: Vec<Relationship>,
}

/// Represents the type of relationship of a part in the 3mf package.
#[derive(ToXml, FromXml, Debug, Clone, Copy, PartialEq, Eq)]
#[xml(scalar)]
pub enum RelationshipType {
    /// Represents a thumbnail part in the package.
    #[xml(
        rename = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
    )]
    Thumbnail,

    /// Represents a model part in the package.
    #[xml(rename = "http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel")]
    Model,
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string, FromXml, ToXml};
    use pretty_assertions::assert_eq;

    use super::{Relationship, RelationshipType, Relationships, RELATIONSHIP_NS};

    #[test]
    pub fn toxml_relationships_test() {
        let xml_string = format!(
            r#"<{a} xmlns="{RELATIONSHIP_NS}"><{b} Id="someId" Target="//somePath//Of//Resources" Type="{MODEL_NS}" /><{b} Id="someId" Target="//somePath//Of//Resources" Type="{THUMBNAIL_NS}" /></{a}>"#,
            a = "Relationships",
            b = "Relationship",
        );
        let relationships = Relationships {
            relationships: vec![
                Relationship {
                    id: "someId".to_owned(),
                    target: "//somePath//Of//Resources".to_owned(),
                    relationship_type: RelationshipType::Model,
                },
                Relationship {
                    id: "someId".to_owned(),
                    target: "//somePath//Of//Resources".to_owned(),
                    relationship_type: RelationshipType::Thumbnail,
                },
            ],
        };
        let relationships_string = to_string(&relationships).unwrap();

        assert_eq!(relationships_string, xml_string);
    }

    #[test]
    pub fn fromxml_relationships_test() {
        let xml_string = format!(
            r#"<{a} xmlns="{RELATIONSHIP_NS}"><{b} Id="someId" Target="//somePath//Of//Resources" Type="{MODEL_NS}" /><{b} Id="someId" Target="//somePath//Of//Resources" Type="{THUMBNAIL_NS}" /></{a}>"#,
            a = "Relationships",
            b = "Relationship",
        );
        let relationships = from_str::<Relationships>(&xml_string).unwrap();

        assert_eq!(
            relationships,
            Relationships {
                relationships: vec![
                    Relationship {
                        id: "someId".to_owned(),
                        target: "//somePath//Of//Resources".to_owned(),
                        relationship_type: RelationshipType::Model,
                    },
                    Relationship {
                        id: "someId".to_owned(),
                        target: "//somePath//Of//Resources".to_owned(),
                        relationship_type: RelationshipType::Thumbnail,
                    },
                ],
            }
        );
    }

    #[derive(ToXml, FromXml, Debug, PartialEq, Eq)]
    struct RelationshipTypes {
        list: Vec<RelationshipType>,
    }

    const MODEL_NS: &str = "http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel";
    const THUMBNAIL_NS: &str =
        "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail";

    #[test]
    pub fn toxml_relationshiptype_test() {
        let xml_string = format!(
            "<{a}><{b}>{MODEL}</{b}><{b}>{THUMBNAIL}</{b}></{a}>",
            a = "RelationshipTypes",
            b = "list",
            MODEL = MODEL_NS,
            THUMBNAIL = THUMBNAIL_NS
        );
        let content = RelationshipTypes {
            list: vec![RelationshipType::Model, RelationshipType::Thumbnail],
        };
        let content_string = to_string(&content).unwrap();

        assert_eq!(content_string, xml_string);
    }

    #[test]
    pub fn fromxml_relationshiptype_test() {
        let xml_string = format!(
            "<{a}><{b}>{MODEL}</{b}><{b}>{THUMBNAIL}</{b}></{a}>",
            a = "RelationshipTypes",
            b = "list",
            MODEL = MODEL_NS,
            THUMBNAIL = THUMBNAIL_NS
        );
        let content = from_str::<RelationshipTypes>(&xml_string).unwrap();
        assert_eq!(
            content,
            RelationshipTypes {
                list: vec![RelationshipType::Model, RelationshipType::Thumbnail],
            }
        );
    }
}
