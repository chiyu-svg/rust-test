/// 动态解析 toml 配置文件， 将 toml 配置文件整体导入，然后解析成键值对
/// 静态解析 toml 配置文件， 将 toml 导入固定同种结构
use std::env;
use std::fs;
use serde_derive::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Human {
    langle: String,
    wisdom: u8,
    strength: u8,
    civilization: u8
}
#[allow(unused)]
#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
struct Config {
    human: Human,
    plant: Plant,
    animal: Animal
}

fn main() {
    toml_static();
    toml_dynamic();
}

fn toml_static(){
    let config_const_value = {
        let config_path = env::args().nth(1).unwrap();
        let config_context = fs::read_to_string(&config_path).unwrap();
        config_context.parse::<toml::Value>().unwrap()
    };
    println!("{}", config_const_value.get("postgresql").unwrap().get("database").unwrap());
}
fn toml_dynamic(){
    let config_const_value: Config = {
        let config_path = env::args().nth(2).unwrap();
        let config_context = fs::read_to_string(&config_path).unwrap();
        toml::from_str(&config_context).unwrap()
    };
    println!("{:#?}", config_const_value.human)
}