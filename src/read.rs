use serde::Deserialize;
use std::io::BufReader;
use std::{fs::File, path::Path};

use quick_xml::de::Deserializer;
use zip::ZipArchive;

use crate::model::Model;
use crate::Error;

/// Read all models from a 3MF file
pub fn read(path: &Path) -> Result<Vec<Model>, Error> {
    let file = File::open(path)?;
    let mut zip = ZipArchive::new(file)?;
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
