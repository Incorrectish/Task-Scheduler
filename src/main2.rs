use serde_json;
use serde::{Serialize, ser::SerializeStruct};
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;

#[derive(PartialEq, Eq)]

#[derive(Hash)]
pub struct Blob {
    date: [u32; 5],
}

fn main() {
    let x = Blob {
        date: [1, 2, 3, 4, 5],
    };
    let my_map: HashMap<Blob, &str> = HashMap::from([(x, "hi")]);
    let x_string = serde_json::to_string(&my_map).expect("no tostring");
    println!("{x_string}");

    let x_struct: [u32; 5] = serde_json::from_str(&x_string as &str).expect("no deserialize");
    println!("{x_struct:?}")
}


impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Blob", 1)?;
        state.serialize_field("date", &self.date)?;
        state.end()
    }
}

impl Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blob: [{}, {}, {}, {}, {}]", self.date[0], self.date[1], self.date[2], self.date[3], self.date[4])
    }
}

