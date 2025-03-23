use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{self, Read, Seek, Write};
use std::path::PathBuf;

use image::{load_from_memory, DynamicImage};
use instant_xml::{from_str, to_string, ToXml};
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::core::model::Model;

use super::content_types::ContentTypes;
use super::error::Error;
use super::relationship::{RelationshipType, Relationships};

#[derive(Debug)]
pub struct ThreemfPackage {
    pub root: Model,
    pub sub_models: HashMap<String, Model>,
    pub thumbnails: HashMap<String, DynamicImage>,
    pub relationships: HashMap<String, Relationships>,
    pub content_types: ContentTypes,
}

impl ThreemfPackage {
    pub fn from_reader<R: Read + io::Seek>(
        reader: R,
        process_sub_models: bool,
    ) -> Result<ThreemfPackage, Error> {
        let mut zip = ZipArchive::new(reader)?;

        let content_types: ContentTypes;
        {
            let content_types_file = zip.by_name("[Content_Types].xml");

            //will fail if it found unsupported contents listed in the ContentTypes.xml
            content_types = match content_types_file {
                Ok(mut file) => {
                    let mut xml_string: String = Default::default();
                    let _ = file.read_to_string(&mut xml_string)?;

                    //ToDo extend the error
                    from_str::<ContentTypes>(&xml_string).unwrap()
                }
                Err(err) => {
                    return Err(Error::Zip(err));
                }
            }
        }

        const ROOT_RELS_FILENAME: &str = "_rels/.rels";
        let mut models = HashMap::<String, Model>::new();
        let mut thumbnails = HashMap::<String, DynamicImage>::new();
        let mut relationships = HashMap::<String, Relationships>::new();
        let mut root_model_path: &str = "";

        let root_rels: Relationships =
            relationships_from_zip_by_name(&mut zip, ROOT_RELS_FILENAME)?;

        let root_model_processed = process_rels(&mut zip, &root_rels, &mut models, &mut thumbnails);
        match root_model_processed {
            Ok(_) => {
                let model_rels = root_rels
                    .relationships
                    .iter()
                    .find(|rels| rels.relationship_type == RelationshipType::Model)
                    .unwrap();
                root_model_path = &model_rels.target;
                relationships.insert(ROOT_RELS_FILENAME.to_owned(), root_rels.clone());
            }
            Err(err) => return Err(err),
        }

        if process_sub_models {
            {
                for value in 0..zip.len() {
                    let file = zip.by_index(value)?;

                    if file.is_file() {
                        if let Some(path) = file.enclosed_name() {
                            if Some(OsStr::new("rels")) == path.extension()
                                && path != PathBuf::from(ROOT_RELS_FILENAME)
                            {
                                let rels = relationships_from_zipfile(file)?;
                                relationships.insert("lala".to_owned(), rels);
                            }
                        }
                    }
                }
            }

            {
                relationships.iter_mut().for_each(|rels| {
                    process_rels(&mut zip, rels.1, &mut models, &mut thumbnails).unwrap();
                });
            }
        }

        let root_model = models.remove(root_model_path).unwrap();

        Ok(ThreemfPackage {
            root: root_model,
            sub_models: models,
            thumbnails,
            relationships,
            content_types,
        })
    }

    pub fn write<W: io::Write + io::Seek>(&self, threemf_archive: W) -> Result<(), Error> {
        let mut archive = ZipWriter::new(threemf_archive);

        archive_write_xml_with_header(&mut archive, "[Content_Types].xml", &self.content_types)?;

        for (path, relationships) in &self.relationships {
            archive_write_xml_with_header(&mut archive, path, &relationships)?;

            for relationship in &relationships.relationships {
                match relationship.relationship_type {
                    RelationshipType::Model => {
                        let model = if *path == *"_rels/.rels" {
                            &self.root
                        } else {
                            let model = self.sub_models.get(&relationship.target).unwrap();
                            model
                        };
                        archive_write_xml_with_header(&mut archive, &relationship.target, model)?;
                    }
                    RelationshipType::Thumbnail => {
                        let image = self.thumbnails.get(&relationship.target).unwrap();
                        archive.start_file(&relationship.target, SimpleFileOptions::default())?;
                        archive.write_all(image.as_bytes())?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn relationships_from_zip_by_name<R: Read + io::Seek>(
    zip: &mut ZipArchive<R>,
    zip_filename: &str,
) -> Result<Relationships, Error> {
    let rels_file = zip.by_name(zip_filename);
    match rels_file {
        Ok(file) => relationships_from_zipfile(file),
        Err(err) => Err(Error::Zip(err)),
    }
}

fn relationships_from_zipfile(mut file: zip::read::ZipFile<'_>) -> Result<Relationships, Error> {
    let mut xml_string: String = Default::default();
    let _ = file.read_to_string(&mut xml_string)?;

    Ok(from_str::<Relationships>(&xml_string).unwrap())
}

fn process_rels<R: Read + io::Seek>(
    zip: &mut ZipArchive<R>,
    rels: &Relationships,
    models: &mut HashMap<String, Model>,
    thumbnails: &mut HashMap<String, DynamicImage>,
) -> Result<(), Error> {
    let model_rels = rels
        .relationships
        .iter()
        .filter(|r| r.relationship_type == RelationshipType::Model);
    for rels in model_rels {
        let model_file_target = PathBuf::from(&rels.target.clone());
        let model_file_path = model_file_target.strip_prefix("/").unwrap();
        let model_file = zip.by_name(model_file_path.to_str().unwrap());
        match model_file {
            Ok(mut file) => {
                let mut xml_string: String = Default::default();
                let _ = file.read_to_string(&mut xml_string).unwrap();

                let model = from_str::<Model>(&xml_string).unwrap();
                models.insert(rels.target.clone(), model);
            }
            Err(err) => return Err(Error::Zip(err)),
        }
    }

    let thumbnails_rels = rels
        .relationships
        .iter()
        .filter(|r| r.relationship_type == RelationshipType::Thumbnail);

    for rels in thumbnails_rels {
        let thumbnail_file_target = PathBuf::from(&rels.target.clone());
        let thumbnail_file_path = thumbnail_file_target.strip_prefix("/").unwrap();
        let thumbnail_file = zip.by_name(thumbnail_file_path.to_str().unwrap());
        match thumbnail_file {
            Ok(mut file) => {
                let mut bytes: Vec<u8> = vec![];
                let _ = file.read_to_end(&mut bytes).unwrap();

                let image = load_from_memory(&bytes).unwrap();
                thumbnails.insert(rels.target.clone(), image);
            }
            Err(err) => return Err(Error::Zip(err)),
        }
    }

    Ok(())
}

fn archive_write_xml_with_header<W: Write + Seek, T: ToXml + ?Sized>(
    archive: &mut ZipWriter<W>,
    filename: &str,
    content: &T,
) -> Result<(), Error> {
    const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

    let mut content_string = to_string(&content).unwrap();
    content_string.insert_str(0, XML_HEADER);

    archive.start_file(filename, SimpleFileOptions::default())?;
    archive.write_all(content_string.as_bytes())?;
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
        io::{content_types::*, relationship::*},
    };

    use super::ThreemfPackage;
    use std::{collections::HashMap, fs::File, io::Cursor, path::Path};

    #[test]
    pub fn from_reader_test() {
        let path = Path::new("tests\\data\\P_XPX_0702_02.3mf");
        let file = File::open(path).unwrap();

        let result = ThreemfPackage::from_reader(file, true);
        // println!("{:?}", result);

        match result {
            Ok(threemf) => {
                assert_eq!(threemf.sub_models.len(), 1);
                assert_eq!(threemf.thumbnails.len(), 1);
                assert_eq!(threemf.relationships.len(), 2);
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    #[test]
    pub fn write_test() {
        let bytes = {
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
            threemf.write(&mut writer).unwrap();
            writer
        };

        assert_eq!(bytes.into_inner().len(), 943);
    }
}
