use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64
}

fn main() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33
    };
    let as_json = to_json(&calabar).unwrap();
    println!("json:\n{}\n", &as_json);
    let as_cbor = to_cbor(&calabar).unwrap();
    println!("cbor:\n{:?}\n", &as_cbor);
    let as_bincode = to_bincode(&calabar).unwrap();
    println!("bincode: \n{:?}\n", &as_bincode);

    println!("json (as UTF-8): \n{}\n", String::from_utf8_lossy(as_json.as_bytes()));

    println!("cbor: \n{:?}\n", String::from_utf8_lossy(&as_cbor));
    println!("cbor: \n{:?}\n", String::from_utf8_lossy(&as_bincode));
}