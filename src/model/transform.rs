use instant_xml::*;

#[derive(Debug, PartialEq)]
pub struct Transform([f64; 16]);

impl ToXml for Transform {
    fn serialize<W: std::fmt::Write + ?Sized>(
        &self,
        field: Option<Id<'_>>,
        serializer: &mut Serializer<W>,
    ) -> Result<(), Error> {
        let prefix = match field {
            Some(id) => {
                let prefix = serializer.write_start(id.name, id.ns)?;
                serializer.end_start()?;
                Some((prefix, id.name))
            }
            None => None,
        };

        let transform_str = self
            .0
            .iter()
            .map(|&m| format!("{:.6}", m))
            .collect::<Vec<String>>()
            .join(" ");
        serializer.write_str(&transform_str)?;

        if let Some((prefix, name)) = prefix {
            serializer.write_close(prefix, name)?;
        }

        Ok(())
    }
}

impl<'xml> FromXml<'xml> for Transform {
    fn matches(id: Id<'_>, field: Option<Id<'_>>) -> bool {
        match field {
            Some(field) => id == field,
            None => false,
        }
    }

    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        field: &'static str,
        deserializer: &mut Deserializer<'cx, 'xml>,
    ) -> Result<(), Error> {
        if into.is_some() {
            return Err(Error::DuplicateValue(field));
        }

        let value = match deserializer.take_str()? {
            Some(value) => value,
            None => return Err(Error::MissingValue("No transform string found")),
        };

        let values = value
            .split(" ")
            .map(|v| {
                let parsed = v.parse::<f64>();
                match parsed {
                    Ok(val) => val,
                    Err(_) => f64::MIN_POSITIVE,
                }
            })
            .collect::<Vec<f64>>();

        if values.len() == 16 {
            *into = Some(Transform(values.try_into().unwrap()));
            return Ok(());
        } else {
            return Err(Error::MissingValue(
                "Not enough values to form a Transform Matrix",
            ));
        }
    }

    type Accumulator = Option<Self>;

    const KIND: Kind = Kind::Scalar;
}

#[cfg(test)]
mod tests {
    use super::Transform;
    use instant_xml::{from_str, to_string, FromXml, ToXml};

    #[test]
    #[rustfmt::skip]
    fn toxml_transform() {
        let transform = Transform([
            3.141592, -2718.281828, 1618.033988, -0.577215,
            707.106781, -1414.213562, 2236.067977, -301.029995,
            1442.249570, -866.025403, 0.693147, -1098.612288,
            1732.050807, -523.598775, 577.215664, -1144.729885,
        ]);
        let xml_string = to_string(&transform).unwrap();
        println!("{:?}", transform);
        assert_eq!(
            xml_string,
            "3.141592 -2718.281828 1618.033988 -0.577215 707.106781 -1414.213562 2236.067977 -301.029995 1442.249570 -866.025403 0.693147 -1098.612288 1732.050807 -523.598775 577.215664 -1144.729885"
        );
    }

    // Transform is a transparent tuple struct, it can only be properly write/read to/from XML when
    //placed in a separate struct
    #[derive(FromXml, ToXml, PartialEq, Debug)]
    struct TestTransform {
        transform: Transform,
    }

    #[test]
    #[rustfmt::skip]
    fn toxml_test_transform() {
       let test_transform = TestTransform{ transform: Transform([
            3.141592, -2718.281828, 1618.033988, -0.577215,
            707.106781, -1414.213562, 2236.067977, -301.029995,
            1442.249570, -866.025403, 0.693147, -1098.612288,
            1732.050807, -523.598775, 577.215664, -1144.729885,
        ])}; 
        let xml_string = to_string(&test_transform).unwrap();
        println!("{:?}", test_transform);
        assert_eq!(
            xml_string,
            "<TestTransform><transform>3.141592 -2718.281828 1618.033988 -0.577215 707.106781 -1414.213562 2236.067977 -301.029995 1442.249570 -866.025403 0.693147 -1098.612288 1732.050807 -523.598775 577.215664 -1144.729885</transform></TestTransform>"
        );
    }

    #[test]
    #[rustfmt::skip]
    fn fromxml_test_transform() {
        let xml_string =
            "<TestTransform><transform>3.141592 -2718.281828 1618.033988 -0.577215 707.106781 -1414.213562 2236.067977 -301.029995 1442.249570 -866.025403 0.693147 -1098.612288 1732.050807 -523.598775 577.215664 -1144.729885</transform></TestTransform>";
        let test_transform = from_str::<TestTransform>(&xml_string).unwrap();
        println!("{:?}", test_transform);
        assert_eq!(
            test_transform.transform,
            Transform([
                3.141592, -2718.281828, 1618.033988, -0.577215,
                707.106781, -1414.213562, 2236.067977, -301.029995,
                1442.249570, -866.025403, 0.693147, -1098.612288,
                1732.050807, -523.598775, 577.215664, -1144.729885
            ])
        );
    }

    // Transform rename
    #[derive(FromXml, ToXml, PartialEq, Debug)]
    #[xml(rename = "rename")]
    struct TestTransformRename {
        #[xml(rename = "transform-matrix")]
        transform: Transform,
    }

    #[test]
    #[rustfmt::skip]
    fn toxml_test_transform_rename() {
        let test_transform = TestTransformRename {
            transform: Transform([
                3.141592, -2718.281828, 1618.033988, -0.577215,
                707.106781, -1414.213562, 2236.067977, -301.029995,
                1442.249570, -866.025403, 0.693147, -1098.612288,
                1732.050807, -523.598775, 577.215664, -1144.729885
            ])
        };
        let xml_string = to_string(&test_transform).unwrap();
        println!("{:?}", test_transform);
        assert_eq!(
            xml_string,
            "<rename><transform-matrix>3.141592 -2718.281828 1618.033988 -0.577215 707.106781 -1414.213562 2236.067977 -301.029995 1442.249570 -866.025403 0.693147 -1098.612288 1732.050807 -523.598775 577.215664 -1144.729885</transform-matrix></rename>"
        );
    }

    #[test]
    #[rustfmt::skip]
    fn fromxml_test_transform_rename() {
        let xml_string =
            "<rename><transform-matrix>3.141592 -2718.281828 1618.033988 -0.577215 707.106781 -1414.213562 2236.067977 -301.029995 1442.249570 -866.025403 0.693147 -1098.612288 1732.050807 -523.598775 577.215664 -1144.729885</transform-matrix></rename>";
        let test_transform = from_str::<TestTransformRename>(&xml_string).unwrap();
        println!("{:?}", test_transform);
        assert_eq!(
            test_transform.transform,
           Transform([
                3.141592, -2718.281828, 1618.033988, -0.577215,
                707.106781, -1414.213562, 2236.067977, -301.029995,
                1442.249570, -866.025403, 0.693147, -1098.612288,
                1732.050807, -523.598775, 577.215664, -1144.729885
            ]) 
        );
    }
}
