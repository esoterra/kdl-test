#![allow(unused)]

use serde::Deserialize;
type Error = (); // TODO: update to correct error type

#[derive(PartialEq, Eq, Debug, Deserialize)]
struct Document {
    foo: Foo,
}

#[derive(PartialEq, Eq, Debug, Deserialize)]
struct Foo {
    bar: Bar,
}

#[derive(PartialEq, Eq, Debug, Deserialize)]
struct Bar {
    #[serde(rename = "#0")]
    value: String,
}

fn kdl_parse(input: &str) -> Result<Document, kdl::de::Error> {
    kdl::de::from_str(input)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn accepts_child_node() {
        let child_node = "foo {\n  bar \"asdf\"\n}";
        match kdl_parse(child_node) {
            Ok(result) => assert_eq!(
                result,
                Document {
                    foo: Foo {
                        bar: Bar {
                            value: "asdf".into()
                        }
                    }
                }
            ),
            Err(error) => {
                eprintln!("{:?}", miette::Error::new(error));
                panic!("Failed test");
            }
        }
    }

    #[test]
    fn rejects_property() {
        let property = "foo bar=\"asdf\"";
        let Err(error) = kdl_parse(property) else {
            panic!("Test failed");
        };
        eprintln!("{:?}", miette::Error::new(error));
    }
}
