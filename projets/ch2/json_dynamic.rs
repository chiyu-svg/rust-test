use std::env;
use std::fs;
use serde_json::{Number, Value};


fn main() {
    let input_path = env::args().nth(1).unwrap();
    let output_path = env::args().nth(2).unwrap();
    let mut products_and_sales = {
        let products_and_sales_context = fs::read_to_string(&input_path).unwrap();
        serde_json::from_str::<Value>(&products_and_sales_context).unwrap()
    };
    if let Value::Number(n) = &products_and_sales["sales"][0]["quantity"] {
        let n = n.as_f64().unwrap() + 100.0;
        let n = Number::from(Number::from_f64(n).unwrap());
        products_and_sales["sales"][0]["quantity"] = Value::Number(n);
    }
    fs::write(&output_path, serde_json::to_string_pretty(&products_and_sales).unwrap()).unwrap();
}