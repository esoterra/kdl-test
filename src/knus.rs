#![allow(unused)]

use knus::{Decode, Error, parse};

#[derive(PartialEq, Eq, Debug, Decode)]
struct Document {
    #[knus(child)]
    foo: Foo,
}

#[derive(PartialEq, Eq, Debug, Decode)]
struct Foo {
    #[knus(child)]
    bar: Bar,
}

#[derive(PartialEq, Eq, Debug, Decode)]
struct Bar {
    #[knus(argument)]
    value: String,
}

fn kdl_parse(input: &str) -> Result<Document, Error> {
    parse::<Document>("test", input)
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
