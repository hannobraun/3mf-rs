use image::{load_from_memory, DynamicImage};
use instant_xml::{from_str, to_string, ToXml};
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::{
    core::model::Model,
    io::{
        content_types::{ContentTypes, DefaultContentTypeEnum},
        error::Error,
        relationship::{RelationshipType, Relationships},
    },
};

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{self, Read, Seek, Write};
use std::path::PathBuf;

/// Represents a 3mf package, the nested folder structure of the parts
/// in the 3mf package will be flattened into respective dictionaries with
/// the key being the path of the part in the archive package.
#[derive(Debug, PartialEq)]
pub struct ThreemfPackage {
    /// The root model of the 3mf package.
    /// Expected to always exist and be a valid model with a [Build](crate::core::build::Build) object.
    pub root: Model,

    /// The sub models contained in the file. Usually this is to represent the [Object](crate::core::object::Object)
    /// that are to be referenced in the [root](ThreemfPackage::root) model part.
    /// The key is the path of the model in the archive package.
    pub sub_models: HashMap<String, Model>,

    /// The thumbnails contained in the file.
    /// The key is the path of the thumbnail in the archive package.
    /// The thumbnail paths defined in the [Model](crate::core::model::Model) object should match the keys in this dictionary.
    pub thumbnails: HashMap<String, DynamicImage>,

    /// The relationships between the different parts in the 3mf package.
    /// The key is the path of the relationship file in the archive package.
    /// Always expected to have at least 1 relationship file,
    /// the root relationship file placed within "_rels" folder at the root of the package
    pub relationships: HashMap<String, Relationships>,

    /// A summary of all Default Content Types that exists in the current 3mf package.
    /// The reader/writer will fail if an unsupported content type is found in the package.
    /// The extensions defined in the [ContentTypes.xml]
    ///  file should match the extensions of the parts in the package.
    pub content_types: ContentTypes,
}

impl ThreemfPackage {
    /// Reads a 3mf package from a [reader].
    /// Expected to deal with nested parts of the 3mf package and flatten them into the respective dictionaries.
    /// Only If [process_sub_models] is set to true, it will process the sub models and thumbnails associated with the sub models in the package.
    /// Will return an error if the package is not a valid 3mf package or if the package contains unsupported content types.
    pub fn from_reader<R: Read + io::Seek>(
        reader: R,
        process_sub_models: bool,
    ) -> Result<Self, Error> {
        let mut zip = ZipArchive::new(reader)?;

        let content_types: ContentTypes;
        {
            let content_types_file = zip.by_name("[Content_Types].xml");

            //will fail if it found unsupported contents listed in the ContentTypes.xml
            content_types = match content_types_file {
                Ok(mut file) => {
                    let mut xml_string: String = Default::default();
                    let _ = file.read_to_string(&mut xml_string)?;

                    from_str::<ContentTypes>(&xml_string)?
                }
                Err(err) => {
                    return Err(Error::Zip(err));
                }
            }
        }

        let rels_ext = {
            let rels_content = content_types
                .defaults
                .iter()
                .find(|t| t.content_type == DefaultContentTypeEnum::Relationship);

            match rels_content {
                Some(rels) => &rels.extension,
                None => "rels",
            }
        };

        let root_rels_filename: &str = &format!("_rels/.{rels_ext}");

        let mut models = HashMap::<String, Model>::new();
        let mut thumbnails = HashMap::<String, DynamicImage>::new();
        let mut relationships = HashMap::<String, Relationships>::new();
        let mut root_model_path: &str = "";

        let root_rels: Relationships =
            relationships_from_zip_by_name(&mut zip, root_rels_filename)?;

        let root_model_processed = process_rels(&mut zip, &root_rels, &mut models, &mut thumbnails);
        match root_model_processed {
            Ok(_) => {
                let model_rels = root_rels
                    .relationships
                    .iter()
                    .find(|rels| rels.relationship_type == RelationshipType::Model);

                if let Some(root_model) = model_rels {
                    root_model_path = &root_model.target;
                    relationships.insert(root_rels_filename.to_owned(), root_rels.clone());
                }
            }
            Err(err) => return Err(err),
        }

        if process_sub_models {
            {
                for value in 0..zip.len() {
                    let file = zip.by_index(value)?;

                    if file.is_file() {
                        if let Some(path) = file.enclosed_name() {
                            if Some(OsStr::new(rels_ext)) == path.extension()
                                && path != PathBuf::from(root_rels_filename)
                            {
                                match path.to_str() {
                                    Some(path_str) => {
                                        let rels = relationships_from_zipfile(file)?;
                                        relationships.insert(format!("/{path_str}"), rels);
                                    }
                                    None => {
                                        return Err(Error::ReadError(
                                            "Failed to read the relationship file path".to_owned(),
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for rels in &relationships {
                process_rels(&mut zip, rels.1, &mut models, &mut thumbnails)?;
            }
        }

        if let Some(root_model) = models.remove(root_model_path) {
            Ok(Self {
                root: root_model,
                sub_models: models,
                thumbnails,
                relationships,
                content_types,
            })
        } else {
            Err(Error::ReadError("Root model not found".to_owned()))
        }
    }

    /// Writes the 3mf package to a [writer].
    /// Expects a well formed [ThreemfPackage] object to write the package.
    /// A well formed packaged requires atleast 1 root model and 1 relationship file along with the content types.
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
                        } else if let Some(model) = self.sub_models.get(&relationship.target) {
                            model
                        } else {
                            return Err(Error::WriteError(format!(
                                "No model found for relationship target {}",
                                relationship.target
                            )));
                        };
                        archive_write_xml_with_header(&mut archive, &relationship.target, model)?;
                    }
                    RelationshipType::Thumbnail => {
                        if let Some(image) = self.thumbnails.get(&relationship.target) {
                            archive
                                .start_file(&relationship.target, SimpleFileOptions::default())?;
                            archive.write_all(image.as_bytes())?;
                        } else {
                            return Err(Error::WriteError(format!(
                                "No thumbnail image found for relationshhip target {}",
                                &relationship.target
                            )));
                        }
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
    let rels = from_str::<Relationships>(&xml_string)?;

    Ok(rels)
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
        let name = try_strip_leading_slash(&rels.target);
        let model_file = zip.by_name(name);
        match model_file {
            Ok(mut file) => {
                let mut xml_string: String = Default::default();
                let _ = file.read_to_string(&mut xml_string)?;

                let model = from_str::<Model>(&xml_string)?;
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
        let name = try_strip_leading_slash(&rels.target);
        let thumbnail_file = zip.by_name(name);
        match thumbnail_file {
            Ok(mut file) => {
                let mut bytes: Vec<u8> = vec![];
                let _ = file.read_to_end(&mut bytes)?;

                let image = load_from_memory(&bytes)?;
                thumbnails.insert(rels.target.clone(), image);
            }
            Err(err) => return Err(Error::Zip(err)),
        }
    }

    Ok(())
}

fn try_strip_leading_slash(target: &str) -> &str {
    match target.strip_prefix("/") {
        Some(stripped) => stripped,
        None => target,
    }
}

fn archive_write_xml_with_header<W: Write + Seek, T: ToXml + ?Sized>(
    archive: &mut ZipWriter<W>,
    filename: &str,
    content: &T,
) -> Result<(), Error> {
    const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

    let mut content_string = to_string(&content)?;
    content_string.insert_str(0, XML_HEADER);

    archive.start_file(filename, SimpleFileOptions::default())?;
    archive.write_all(content_string.as_bytes())?;
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::assert_eq;

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

    use std::{collections::HashMap, io::Cursor};

    #[test]
    pub fn from_reader_test() {
        let bytes = include_bytes!("../../tests/data/P_XPX_0702_02.3mf");
        let reader = Cursor::new(bytes);

        let result = ThreemfPackage::from_reader(reader, true);
        // println!("{:?}", result);

        match result {
            Ok(threemf) => {
                assert_eq!(threemf.sub_models.len(), 1);
                assert_eq!(threemf.thumbnails.len(), 1);
                assert_eq!(threemf.relationships.len(), 2);

                assert!(threemf.sub_models.contains_key("/3D/midway.model"));

                assert!(threemf.relationships.contains_key("_rels/.rels"));
                assert!(threemf
                    .relationships
                    .contains_key("/3D/_rels/3dmodel.model.rels"));
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
