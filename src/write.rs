use std::io::{self, prelude::*};

use crate::Error;
use quick_xml::{
    events::{BytesDecl, Event},
    se::Serializer,
    Writer,
};
use serde::Serialize;

use zip::{write::FileOptions, ZipWriter};

use crate::model::Model;

/// Write a triangle mesh to a 3MF writer
pub fn write<W: Write + io::Seek, M: Into<Model>>(writer: W, model: M) -> Result<(), Error> {
    let mut archive = ZipWriter::new(writer);

    archive.start_file("[Content_Types].xml", FileOptions::default())?;
    archive.write_all(include_bytes!("content-types.xml"))?;

    archive.start_file("_rels/.rels", FileOptions::default())?;
    archive.write_all(include_bytes!("rels.xml"))?;

    archive.start_file("3D/model.model", FileOptions::default())?;

    let mut xml = String::new();

    let mut ser = Serializer::with_root(&mut xml, Some("model"))?;
    ser.indent(' ', 2);
    model.into().serialize(ser)?;

    let mut xml_writer = Writer::new_with_indent(&mut archive, b' ', 2);
    xml_writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;
    xml_writer.write_indent()?;
    xml_writer.into_inner().write_all(xml.as_bytes())?;

    archive.finish()?;

    Ok(())
}
