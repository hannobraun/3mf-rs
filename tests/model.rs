use threemf::model::{Components, Object};

#[test]
fn test_object() {
    let object_str = r##"<object id="1"><components><component objectid="66" transform="0.0393701 0 0 0 0.0393701 0 0 0 0.0393701 0 0 0" /><component objectid="67" transform="0.0393701 0 0 0 0.0393701 0 0 0 0.0393701 0 0 0" /><component objectid="68" transform="0.0393701 0 0 0 0.0393701 0 0 0 0.0393701 0 0 0" /></components></object>"##;
    let object_de: Object = quick_xml::de::from_str(object_str).unwrap();
    match object_de {
        Object { mesh: Some(_), .. } => panic!("No mesh in this object"),
        Object {
            components: Some(Components { component }),
            ..
        } => {
            assert_eq!(component.len(), 3);
            let transform = component.first().unwrap().transform.unwrap();
            assert_eq!(transform[0], 0.0393701);
        }
        _ => panic!("There should be components"),
    }
}

#[test]
fn test_metadatagroup() {
    let object_str = r##"
                <object id="2" name="Part 2" type="model" p:UUID="5690f40c-430c-479f-b804-29081051c247" pid="1" pindex="0">
                        <metadatagroup>
                                <metadata name="customXMLNS0:PTC_onshape_metadata" type="entity_type">Body</metadata>
                        </metadatagroup>
                        <mesh>
                                <vertices>
                                        <vertex x="0.14470075" y="-0.02387521" z="-0.09085463" />
                                        <vertex x="0.14421695" y="-0.02435900" z="-0.09359834" />
                                        <vertex x="0.14147323" y="-0.02387521" z="-0.09085463" />
                                </vertices>
                                <triangles>
                                        <triangle v1="0" v2="1" v3="2" />
                                </triangles>
                        </mesh>
                </object>
        "##;
    let object_de: Object = quick_xml::de::from_str(object_str).unwrap();
    assert!(object_de.mesh.is_some());
}
