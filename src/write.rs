use std::{fs::File, io::prelude::*, path::Path};

use crate::Error;
use quick_xml::{
    events::{BytesDecl, Event},
    se::Serializer,
    Writer,
};
use serde::Serialize;

use zip::{write::FileOptions, ZipWriter};

use crate::{
    model::{Build, Item, Model, Object, ObjectData, Resources},
    Mesh,
};

/// Write a triangle mesh to a 3MF file
pub fn write(path: &Path, mesh: Mesh) -> Result<(), Error> {
    let file = File::create(path)?;
    let mut archive = ZipWriter::new(file);

    archive.start_file("[Content_Types].xml", FileOptions::default())?;
    archive.write_all(include_bytes!("content-types.xml"))?;

    archive.start_file("_rels/.rels", FileOptions::default())?;
    archive.write_all(include_bytes!("rels.xml"))?;

    archive.start_file("3D/model.model", FileOptions::default())?;
    write_mesh(&mut archive, mesh)?;

    archive.finish()?;

    Ok(())
}

fn write_mesh(sink: impl Write, mesh: Mesh) -> Result<(), Error> {
    let object = Object {
        id: 1,
        partnumber: None,
        name: None,
        pid: None,
        object: ObjectData::Mesh(mesh),
    };
    let resources = Resources {
        object: vec![object],
        basematerials: None,
    };
    let build = Build {
        item: vec![Item {
            objectid: 1,
            transform: None,
            partnumber: None,
        }],
    };
    let model = Model {
        resources,
        build,
        ..Default::default()
    };

    let mut ser = Serializer::with_root(String::new(), Some("model"))?;
    ser.indent(' ', 2);

    let mut xml_writer = Writer::new_with_indent(sink, b' ', 2);
    xml_writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;
    xml_writer.write_indent()?;
    xml_writer
        .inner()
        .write_all(model.serialize(ser).unwrap().as_bytes())?;

    Ok(())
}
