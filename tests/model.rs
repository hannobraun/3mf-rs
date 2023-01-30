use threemf::model::ObjectData;

#[test]
fn test_object() {
    let object_str = r#"<components><component objectid="66" transform="0.0393701 0 0 0 0.0393701 0 0 0 0.0393701 0 0 0" /><component objectid="67" transform="0.0393701 0 0 0 0.0393701 0 0 0 0.0393701 0 0 0" /><component objectid="68" transform="0.0393701 0 0 0 0.0393701 0 0 0 0.0393701 0 0 0" /></components>"#;
    let object_de: ObjectData = quick_xml::de::from_str(object_str).unwrap();
    match object_de {
        ObjectData::Mesh(_) => panic!("No mesh in this object"),
        ObjectData::Components { component } => {
            assert_eq!(component.len(), 3);
            let transform = component.first().unwrap().transform.unwrap();
            assert_eq!(transform[0], 0.0393701);
        }
    }
}
