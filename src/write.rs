use std::{
    fs::File,
    io::{self, prelude::*},
    path::Path,
};

use crate::Error;

use zip::{write::FileOptions, ZipWriter};

use crate::TriangleMesh;

/// Write a triangle mesh to a 3MF file
pub fn write(path: &Path, mesh: &TriangleMesh) -> Result<(), Error> {
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

fn write_mesh(mut sink: impl Write, mesh: &TriangleMesh) -> io::Result<()> {
    sink.write_all(include_bytes!("model-header.xml"))?;

    writeln!(sink, "\t\t\t\t<vertices>")?;
    for vertex in &mesh.vertices {
        writeln!(
            sink,
            "\t\t\t\t\t<vertex x=\"{}\" y=\"{}\" z=\"{}\" />",
            vertex[0], vertex[1], vertex[2],
        )?;
    }
    writeln!(sink, "\t\t\t\t</vertices>")?;

    writeln!(sink, "\t\t\t\t<triangles>")?;
    for [i1, i2, i3] in &mesh.triangles {
        writeln!(
            sink,
            "\t\t\t\t\t<triangle v1=\"{}\" v2=\"{}\" v3=\"{}\" />",
            i1, i2, i3,
        )?;
    }
    writeln!(sink, "\t\t\t\t</triangles>")?;

    sink.write_all(include_bytes!("model-footer.xml"))?;

    Ok(())
}
