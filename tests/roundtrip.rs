use model::{Triangle, Triangles, Vertex, Vertices};
use std::io::Cursor;
use threemf::{model, Mesh};

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

    let mut buf = Cursor::new(Vec::new());

    threemf::write(&mut buf, mesh).expect("Error writing mesh");
    let models = threemf::read(&mut buf).expect("Error reading model");

    if let Some(read_mesh) = &models[0].resources.object[0].mesh {
        assert!(read_mesh == &write_mesh);
    }
}
