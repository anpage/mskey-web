use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use bink_types::bink::Keys;
use umskt::{
    confid,
    crypto::{EllipticCurve, PrivateKey},
    pidgen3::{bink1998, bink2002},
};

/// Differentiate between the two types of product keys and an invalid key
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductKey {
    Bink1998 {
        key: bink1998::ProductKey,
        bink_ids: HashMap<String, u8>,
    },
    Bink2002 {
        key: bink2002::ProductKey,
        bink_ids: HashMap<String, u8>,
    },
    Invalid,
}

/// Represents a Product that can be sorted alphabetically by name
pub struct Product {
    pub name: String,
    pub bink_ids: Vec<u8>,
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Product {}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

/// Provides methods for generating and validating product keys
pub struct KeyGen {
    keys: Keys,
    sorted_products: Vec<Product>,
}

impl KeyGen {
    /// Initialize key crypto with the serialized key data in `bink.bin`
    pub fn new() -> Result<Self> {
        let keys = Self::load_keys()?;
        let mut sorted_products: Vec<Product> = keys
            .products
            .iter()
            .map(|(name, product)| Product {
                name: name.clone(),
                bink_ids: product.bink.clone(),
            })
            .collect();
        sorted_products.sort();
        Ok(Self {
            keys,
            sorted_products,
        })
    }

    fn load_keys() -> Result<Keys> {
        let keys = bincode::deserialize(std::include_bytes!("../bink.bin"))?;
        Ok(keys)
    }

    pub fn sorted_products(&self) -> &[Product] {
        &self.sorted_products
    }

    pub fn gen_key(&self, bink_id: u8, channel_id: &str) -> Result<String> {
        let channel_id = channel_id.parse::<u32>()?;

        if channel_id > 999 {
            bail!("Channel ID must be 3 digits or fewer");
        }

        let bink = self
            .keys
            .bink
            .get(&bink_id)
            .ok_or(anyhow!("Bink ID not found"))?;

        let curve = EllipticCurve::new(
            bink.p.clone(),
            bink.a.clone(),
            bink.g.x.clone(),
            bink.g.y.clone(),
            bink.public.x.clone(),
            bink.public.y.clone(),
        );
        let private_key = PrivateKey::new(bink.n.clone(), bink.private.clone());

        if bink_id < 0x40 {
            Ok(
                bink1998::ProductKey::new(&curve, &private_key, channel_id, None, None)?
                    .to_string(),
            )
        } else {
            Ok(
                bink2002::ProductKey::new(&curve, &private_key, channel_id, None, None)?
                    .to_string(),
            )
        }
    }

    pub fn validate_key(&self, key: &str) -> Result<ProductKey> {
        let mut valid_bink_ids = HashMap::new();
        let mut product_key_bink1998: Option<bink1998::ProductKey> = None;
        let mut product_key_bink2002: Option<bink2002::ProductKey> = None;
        for (name, product) in &self.keys.products {
            for bink_id in &product.bink {
                let bink = self
                    .keys
                    .bink
                    .get(bink_id)
                    .ok_or(anyhow!("Bink ID not found"))?;

                let curve = EllipticCurve::new(
                    bink.p.clone(),
                    bink.a.clone(),
                    bink.g.x.clone(),
                    bink.g.y.clone(),
                    bink.public.x.clone(),
                    bink.public.y.clone(),
                );

                if *bink_id < 0x40_u8 {
                    if let Ok(product_key) = bink1998::ProductKey::from_key(&curve, key) {
                        product_key_bink1998 = Some(product_key);
                        valid_bink_ids.insert(name.clone(), *bink_id);
                    }
                } else if let Ok(product_key) = bink2002::ProductKey::from_key(&curve, key) {
                    product_key_bink2002 = Some(product_key);
                    valid_bink_ids.insert(name.clone(), *bink_id);
                }
            }
        }

        if let Some(product_key) = product_key_bink1998 {
            return Ok(ProductKey::Bink1998 {
                key: product_key,
                bink_ids: valid_bink_ids,
            });
        }

        if let Some(product_key) = product_key_bink2002 {
            return Ok(ProductKey::Bink2002 {
                key: product_key,
                bink_ids: valid_bink_ids,
            });
        }

        Ok(ProductKey::Invalid)
    }

    pub fn get_confirmation_id(installation_id: &str) -> Result<String> {
        let installation_id = installation_id.replace('-', "");
        Ok(confid::generate(&installation_id)?)
    }
}
