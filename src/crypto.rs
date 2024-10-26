use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use bink_types::bink::Keys;
use serde::{Deserialize, Serialize};
use umskt::{
    confid,
    crypto::{EllipticCurve, PrivateKey},
    pidgen3::{bink1998, bink2002},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductKey1998 {
    pub upgrade: bool,
    pub channel_id: String,
    pub sequence: String,
    pub hash: String,
    pub signature: String,
}

impl From<bink1998::ProductKey> for ProductKey1998 {
    fn from(value: bink1998::ProductKey) -> Self {
        ProductKey1998 {
            upgrade: value.upgrade(),
            channel_id: value.channel_id().to_string(),
            sequence: value.sequence().to_string(),
            hash: value.hash().to_string(),
            signature: value.signature().to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductKey2002 {
    pub upgrade: bool,
    pub channel_id: String,
    pub hash: String,
    pub signature: String,
    pub auth_info: String,
}

impl From<bink2002::ProductKey> for ProductKey2002 {
    fn from(value: bink2002::ProductKey) -> Self {
        ProductKey2002 {
            upgrade: value.upgrade(),
            channel_id: value.channel_id().to_string(),
            hash: value.hash().to_string(),
            signature: value.signature().to_string(),
            auth_info: value.auth_info().to_string(),
        }
    }
}

/// Differentiate between the two types of product keys and an invalid key
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProductKey {
    Empty,
    Bink1998 {
        key: ProductKey1998,
        bink_ids: HashMap<String, u8>,
    },
    Bink2002 {
        key: ProductKey2002,
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
pub struct KeyTool {
    keys: Keys,
    products: Vec<Product>,
}

impl KeyTool {
    /// Initialize key crypto with the serialized key data in `bink.bin`
    pub fn new() -> Self {
        let keys: Keys = bincode::deserialize(std::include_bytes!("../bink.bin")).unwrap();
        let mut products: Vec<Product> = keys
            .products
            .iter()
            .map(|(name, product)| Product {
                name: name.clone(),
                bink_ids: product.bink.clone(),
            })
            .collect();
        products.sort();
        Self { keys, products }
    }

    pub fn sorted_products(&self) -> &[Product] {
        &self.products
    }

    pub fn gen_key(
        &self,
        bink_id: u8,
        channel_id: &str,
        sequence: &str,
        upgrade: bool,
    ) -> Result<String> {
        let channel_id = channel_id.parse::<u32>()?;

        if channel_id > 999 {
            bail!("Channel ID must be 3 digits or fewer");
        }

        let sequence = if sequence.is_empty() {
            None
        } else {
            let sequence = sequence.parse::<u32>()?;
            if sequence > 999999 {
                bail!("Sequence must be 6 digits or fewer");
            }
            Some(sequence)
        };

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
            Ok(bink1998::ProductKey::new(
                &curve,
                &private_key,
                channel_id,
                sequence,
                Some(upgrade),
            )?
            .to_string())
        } else {
            Ok(
                bink2002::ProductKey::new(&curve, &private_key, channel_id, None, Some(upgrade))?
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
                key: product_key.into(),
                bink_ids: valid_bink_ids,
            });
        }

        if let Some(product_key) = product_key_bink2002 {
            return Ok(ProductKey::Bink2002 {
                key: product_key.into(),
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
