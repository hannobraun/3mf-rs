use std::io::{self, prelude::*};

use super::error::Error;
use crate::core::model::Model;
use quick_xml::{
    events::{BytesDecl, Event},
    se::Serializer,
    Writer,
};
use serde::Serialize;

use zip::{write::SimpleFileOptions, ZipWriter};

/// Write a triangle mesh to a 3MF writer
pub fn write<W: Write + io::Seek, M: Into<Model>>(writer: W, model: M) -> Result<(), Error> {
    let mut archive = ZipWriter::new(writer);

    archive.start_file("[Content_Types].xml", SimpleFileOptions::default())?;
    archive.write_all(include_bytes!("content-types.xml"))?;

    archive.start_file("_rels/.rels", SimpleFileOptions::default())?;
    archive.write_all(include_bytes!("rels.xml"))?;

    archive.start_file("3D/model.model", SimpleFileOptions::default())?;

    let mut xml = String::new();

    let mut ser = Serializer::with_root(&mut xml, Some("model"))?;
    ser.indent(' ', 2);
    model.into().serialize(ser)?;

    let mut xml_writer = Writer::new_with_indent(&mut archive, b' ', 2);
    xml_writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;
    xml_writer.write_indent()?;
    xml_writer.into_inner().write_all(xml.as_bytes())?;
    // println!("{}", xml);

    archive.finish()?;

    Ok(())
}

pub mod v2 {
    use std::io::{self, Write};

    use instant_xml::to_string;
    use zip::{write::SimpleFileOptions, ZipWriter};

    use crate::io::{
        error::Error, relationship::RelationshipType, threemf_package::ThreemfPackage,
    };

    pub fn write<W: io::Write + io::Seek>(
        threemf_archive: W,
        threemf: ThreemfPackage,
    ) -> Result<(), Error> {
        let mut archive = ZipWriter::new(threemf_archive);

        const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

        let mut content_types = to_string(&threemf.content_types).unwrap();
        content_types.insert_str(0, XML_HEADER);
        archive.start_file("[Content_Types].xml", SimpleFileOptions::default())?;
        archive.write_all(content_types.as_bytes())?;

        for (path, relationships) in threemf.relationships {
            let mut rels = to_string(&relationships).unwrap();
            rels.insert_str(0, XML_HEADER);
            archive.start_file(path.clone(), SimpleFileOptions::default())?;
            archive.write_all(rels.as_bytes())?;

            for relationship in relationships.relationships {
                match relationship.relationship_type {
                    RelationshipType::Model => {
                        let mut model_string = if path == "_rels/.rels".to_owned() {
                            to_string(&threemf.root).unwrap()
                        } else {
                            let model = threemf.sub_models.get(&relationship.target).unwrap();
                            to_string(model).unwrap()
                        };
                        model_string.insert_str(0, XML_HEADER);
                        archive.start_file(relationship.target, SimpleFileOptions::default())?;
                        archive.write_all(model_string.as_bytes())?;
                    }
                    RelationshipType::Thumbnail => {
                        let image = threemf.thumbnails.get(&relationship.target).unwrap();
                        archive.start_file(relationship.target, SimpleFileOptions::default())?;
                        archive.write_all(image.as_bytes())?;
                    }
                }
            }
        }

        Ok(())
    }

    #[cfg(test)]
    pub mod tests {

        use crate::{
            core::{
                build::Build,
                model::{self, Model},
                object::{Object, ObjectType},
                resources::Resources,
            },
            io::{
                content_types::{ContentTypes, DefaultContentTypeEnum, DefaultContentTypes},
                relationship::{Relationship, RelationshipType, Relationships},
                threemf_package::ThreemfPackage,
            },
        };

        use super::write;

        use std::{collections::HashMap, io::Cursor};

        #[test]
        pub fn write_test() {
            let bytes = {
                // let file = File::create_new("trial.3mf").unwrap();
                let bytes = Vec::<u8>::new();
                let mut writer = Cursor::new(bytes);
                let threemf = ThreemfPackage {
                    root: Model {
                        xmlns: None,
                        unit: model::Unit::Centimeter,
                        requiredextensions: None,
                        recommendedextensions: None,
                        metadata: vec![],
                        resources: Resources {
                            object: vec![Object {
                                id: 1,
                                objecttype: Some(ObjectType::Model),
                                thumbnail: None,
                                partnumber: None,
                                name: Some("Some object".to_owned()),
                                pid: None,
                                pindex: None,
                                uuid: Some("uuid".to_owned()),
                                mesh: None,
                                components: None,
                            }],
                            basematerials: vec![],
                        },
                        build: Build {
                            uuid: None,
                            item: vec![],
                        },
                    },
                    sub_models: HashMap::new(),
                    thumbnails: HashMap::new(),
                    relationships: HashMap::from([(
                        "_rels/.rels".to_owned(),
                        Relationships {
                            relationships: vec![Relationship {
                                id: "rel0".to_owned(),
                                target: "/3D/3Dmodel.model".to_owned(),
                                relationship_type: RelationshipType::Model,
                            }],
                        },
                    )]),
                    content_types: ContentTypes {
                        defaults: vec![
                            DefaultContentTypes {
                                extension: "rels".to_owned(),
                                content_type: DefaultContentTypeEnum::Relationship,
                            },
                            DefaultContentTypes {
                                extension: "model".to_owned(),
                                content_type: DefaultContentTypeEnum::Model,
                            },
                        ],
                    },
                };
                write(&mut writer, threemf).unwrap();
                writer
            };

            assert_eq!(bytes.into_inner().len(), 943);
        }
    }
}
