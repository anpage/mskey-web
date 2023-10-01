use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    #[serde(rename = "Products")]
    pub products: HashMap<String, Product>,
    #[serde(rename = "BINK")]
    pub bink: HashMap<String, Bink>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "BINK")]
    pub bink: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bink {
    pub p: String,
    pub a: String,
    pub b: String,
    pub g: Point,
    #[serde(rename = "pub")]
    pub public: Point,
    pub n: String,
    #[serde(rename = "priv")]
    pub private: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: String,
    pub y: String,
}
