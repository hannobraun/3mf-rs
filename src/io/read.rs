use quick_xml::de::Deserializer;
use serde::Deserialize;
use zip::ZipArchive;

use crate::core::model::Model;
use crate::io::error::Error;

use std::io::{self, BufReader, Read};

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
