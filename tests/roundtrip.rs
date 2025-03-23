use core::{Triangle, Triangles, Vertex, Vertices};
use pretty_assertions::assert_eq;
use std::{collections::HashMap, io::Cursor};
use threemf::{
    core::{
        self,
        build::{Build, Item},
        model::{Model, Unit},
        object::{Object, ObjectType},
        resources::Resources,
    },
    io::{
        content_types::{ContentTypes, DefaultContentTypeEnum, DefaultContentTypes},
        relationship::{Relationship, RelationshipType, Relationships},
        threemf_package::ThreemfPackage,
    },
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
            p1: None,
            p2: None,
            p3: None,
            pid: None,
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

#[test]
fn roundtrip_threemfpackage_test() {
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
            p1: None,
            p2: None,
            p3: None,
            pid: None,
        }],
    };

    let mesh = Mesh {
        triangles,
        vertices,
    };

    let write_package = ThreemfPackage {
        root: Model {
            xmlns: None,
            unit: Unit::Millimeter,
            requiredextensions: None,
            recommendedextensions: None,
            metadata: vec![],
            resources: Resources {
                object: vec![Object {
                    id: 1,
                    objecttype: Some(ObjectType::Model),
                    thumbnail: None,
                    partnumber: None,
                    name: Some("Mesh".to_owned()),
                    pid: None,
                    pindex: None,
                    uuid: None,
                    mesh: Some(mesh.clone()),
                    components: None,
                }],
                basematerials: vec![],
            },
            build: Build {
                uuid: None,
                item: vec![Item {
                    objectid: 1,
                    transform: None,
                    partnumber: None,
                    path: None,
                    uuid: None,
                }],
            },
        },
        sub_models: HashMap::new(),
        thumbnails: HashMap::new(),
        relationships: HashMap::from([(
            "_rels/.rels".to_owned(),
            Relationships {
                relationships: vec![Relationship {
                    id: "rel0".to_owned(),
                    target: "3D/3Dmodel.model".to_owned(),
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

    let mut buf = Cursor::new(Vec::new());

    write_package
        .write(&mut buf)
        .expect("Error writing package");
    let models = ThreemfPackage::from_reader(&mut buf, false).expect("Error reading package");
    assert_eq!(models, write_package);
}
