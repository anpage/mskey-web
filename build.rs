use std::{collections::HashMap, error, fs::File, io::Write};

use bink_types::{bink, keys};
use num_bigint::BigInt;
use num_traits::Num;
use serde_json::from_str;

pub fn load_keys() -> serde_json::Result<keys::Keys> {
    let keys = from_str(std::include_str!("keys.json"))?;
    Ok(keys)
}

/// Converts the keys.json file into a binary file of BigInts.
fn main() -> Result<(), Box<dyn error::Error>> {
    let keys = load_keys()?;
    let bink = {
        let products: HashMap<String, bink::Product> = keys
            .products
            .iter()
            .map(|(k, v)| {
                let k = k.clone();
                let v = bink::Product {
                    bink: v
                        .bink
                        .iter()
                        .map(|v| u8::from_str_radix(v, 16).unwrap())
                        .collect(),
                };
                (k, v)
            })
            .collect();

        let bink: HashMap<u8, bink::Bink> = keys
            .bink
            .iter()
            .map(|(k, v)| {
                let k = u8::from_str_radix(k, 16).unwrap();

                let p = BigInt::from_str_radix(&v.p, 10).unwrap();
                let a = BigInt::from_str_radix(&v.a, 10).unwrap();
                let b = BigInt::from_str_radix(&v.b, 10).unwrap();
                let gx = BigInt::from_str_radix(&v.g.x, 10).unwrap();
                let gy = BigInt::from_str_radix(&v.g.y, 10).unwrap();
                let kx = BigInt::from_str_radix(&v.public.x, 10).unwrap();
                let ky = BigInt::from_str_radix(&v.public.y, 10).unwrap();
                let n = BigInt::from_str_radix(&v.n, 10).unwrap();
                let private = BigInt::from_str_radix(&v.private, 10).unwrap();

                let v = bink::Bink {
                    p,
                    a,
                    b,
                    g: bink::Point { x: gx, y: gy },
                    public: bink::Point { x: kx, y: ky },
                    n,
                    private,
                };
                (k, v)
            })
            .collect();

        bink::Keys { products, bink }
    };

    let encoded: Vec<u8> = bincode::serialize(&bink)?;
    let mut f = File::create("bink.bin")?;
    f.write_all(&encoded)?;
    Ok(())
}
