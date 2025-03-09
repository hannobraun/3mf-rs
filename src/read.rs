use instant_xml::from_str;
use serde::de::Unexpected;
use serde::Deserialize;
use std::fs::read_to_string;
use std::io::{self, BufReader, Read};

use quick_xml::de::Deserializer;
use zip::ZipArchive;

use crate::core::model::Model;
use crate::Error;

/// Read all models from a 3MF reader
pub fn read<R: Read + io::Seek>(reader: R) -> Result<Vec<Model>, Error> {
    let mut zip = ZipArchive::new(reader)?;
    let mut models = Vec::new();

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        if file.name().ends_with(".model") {
            let mut de = Deserializer::from_reader(BufReader::new(file));
            models.push(Model::deserialize(&mut de)?);
        }
    }

    Ok(models)
}

// pub fn read_root_model<R: Read + io::Seek>(reader: R) -> Result<Model, Error> {
//     let mut zip = ZipArchive::new(reader)?;
//     for i in 0..zip.len() {
//         let file_or_folder = zip.by_index(i)?;
//         let name = file_or_folder.enclosed_name().unwrap();
//         if file_or_folder.is_file() && name.ends_with(".model") {
//             let xml_string = read_to_string(name)?;
//             let model = from_str::<Model>(&xml_string).unwrap();
//             return Ok(model);
//         }
//     }

//     return Err("Missing root model file".to_string());
// }
