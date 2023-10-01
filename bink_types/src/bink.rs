use std::collections::HashMap;

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    #[serde(rename = "Products")]
    pub products: HashMap<String, Product>,
    #[serde(rename = "BINK")]
    pub bink: HashMap<u8, Bink>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "BINK")]
    pub bink: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bink {
    pub p: BigInt,
    pub a: BigInt,
    pub b: BigInt,
    pub g: Point,
    #[serde(rename = "pub")]
    pub public: Point,
    pub n: BigInt,
    #[serde(rename = "priv")]
    pub private: BigInt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
}
