use instant_xml::*;
use serde::{Deserialize, Serialize};
use std::ops::Index;

const MATRIX_SIZE: usize = 12;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transform(pub [f64; MATRIX_SIZE]);

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

        if values.len() == MATRIX_SIZE {
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

impl Index<usize> for Transform {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index < MATRIX_SIZE {
            &self.0[index]
        } else {
            panic!("Unexpected index for Transform {:?}", index);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string, FromXml, ToXml};
    use pretty_assertions::assert_eq;

    use super::Transform;

    #[test]
    #[rustfmt::skip]
    fn toxml_transform() {
        let xml_string = "3.141592 -2718.281828 1618.033988 707.106781 -1414.213562 2236.067977 1442.249570 -866.025403 0.693147 1732.050807 -523.598775 577.215664";
        let transform = Transform([
            3.141592, -2718.281828, 1618.033988,
            707.106781, -1414.213562, 2236.067977,
            1442.249570, -866.025403, 0.693147,
            1732.050807, -523.598775, 577.215664,
        ]);
        let transform_string = to_string(&transform).unwrap();

        assert_eq!(transform_string, xml_string);
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
        let xml_string = "<TestTransform><transform>3.141592 -2718.281828 1618.033988 707.106781 -1414.213562 2236.067977 1442.249570 -866.025403 0.693147 1732.050807 -523.598775 577.215664</transform></TestTransform>";
       let test_transform = TestTransform{ transform: Transform([
            3.141592, -2718.281828, 1618.033988,
            707.106781, -1414.213562, 2236.067977,
            1442.249570, -866.025403, 0.693147,
            1732.050807, -523.598775, 577.215664,
        ])}; 
        let transform_string = to_string(&test_transform).unwrap();

        assert_eq!(transform_string, xml_string);
    }

    #[test]
    #[rustfmt::skip]
    fn fromxml_test_transform() {
        let xml_string =
            "<TestTransform><transform>3.141592 -2718.281828 1618.033988 707.106781 -1414.213562 2236.067977 1442.249570 -866.025403 0.693147 1732.050807 -523.598775 577.215664</transform></TestTransform>";
        let test_transform = from_str::<TestTransform>(&xml_string).unwrap();

        assert_eq!(
            test_transform.transform,
            Transform([
                3.141592, -2718.281828, 1618.033988,
                707.106781, -1414.213562, 2236.067977,
                1442.249570, -866.025403, 0.693147,
                1732.050807, -523.598775, 577.215664,
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
        let xml_string = "<rename><transform-matrix>3.141592 -2718.281828 1618.033988 707.106781 -1414.213562 2236.067977 1442.249570 -866.025403 0.693147 1732.050807 -523.598775 577.215664</transform-matrix></rename>";
        let test_transform = TestTransformRename {
            transform: Transform([
                3.141592, -2718.281828, 1618.033988,
                707.106781, -1414.213562, 2236.067977,
                1442.249570, -866.025403, 0.693147,
                1732.050807, -523.598775, 577.215664,
            ])
        };
        let transform_string = to_string(&test_transform).unwrap();

        assert_eq!(transform_string, xml_string);
    }

    #[test]
    #[rustfmt::skip]
    fn fromxml_test_transform_rename() {
        let xml_string =
            "<rename><transform-matrix>3.141592 -2718.281828 1618.033988 707.106781 -1414.213562 2236.067977 1442.249570 -866.025403 0.693147 1732.050807 -523.598775 577.215664</transform-matrix></rename>";
        let test_transform = from_str::<TestTransformRename>(&xml_string).unwrap();

        assert_eq!(
            test_transform.transform,
           Transform([
                3.141592, -2718.281828, 1618.033988,
                707.106781, -1414.213562, 2236.067977,
                1442.249570, -866.025403, 0.693147,
                1732.050807, -523.598775, 577.215664,
            ]) 
        );
    }
}
