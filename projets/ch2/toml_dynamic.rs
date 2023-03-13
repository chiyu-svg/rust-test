use serde_derive::Deserialize;
use std::env;
use std::fs;

#[allow(unused)]
#[derive(Deserialize,Debug)]
struct Human {
    langle: String,
    wisdom: u8,
    strength: u8,
    civilization: u8
}

#[allow(unused)]
#[derive(Deserialize,Debug)]
struct Plant {
    life_blood: String,
    life: String
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Animal {
    body: String,
    eat: String
}

#[allow(unused)]
#[derive(Deserialize,Debug)]
struct Config {
    human: Human,
    plant: Plant,
    animal: Animal
}

fn main(){
    let config_const_value: Config = {
        let config_path = env::args().nth(1).expect("ENTER THE FILE");
        let config_text = fs::read_to_string(&config_path).unwrap();
        toml::from_str(&config_text).expect("Parse FILE")
    };
    println!("animal {:#?}", config_const_value.animal)
}