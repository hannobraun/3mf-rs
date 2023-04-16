use model::{Triangle, Triangles, Vertex, Vertices};
use std::path::Path;
use threemf::{
    model::{self, ObjectData},
    Mesh,
};

#[test]
fn roundtrip() {
    let vertices = Vertices {
        vertex: vec![
            Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vertex {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            Vertex {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
        ],
    };

    let triangles = Triangles {
        triangle: vec![Triangle {
            v1: 0,
            v2: 1,
            v3: 2,
        }],
    };

    let mesh = Mesh {
        triangles,
        vertices,
    };

    let write_mesh = mesh.clone();

    threemf::write(Path::new("triangle.3mf"), mesh).expect("Error writing mesh");
    let models = threemf::read(Path::new("triangle.3mf")).expect("Error reading model");

    if let ObjectData::Mesh(read_mesh) = &models[0].resources.object[0].object {
        assert!(read_mesh == &write_mesh);
    }
}
