use crate::BasisSet;

pub const DATA_STO3G: &str = include_str!("STO-3G.json");
pub const DATA_6_31G: &str = include_str!("6-31G.json");

lazy_static::lazy_static! {
    pub static ref BASIS_STO_3G: BasisSet = serde_json::from_str(DATA_STO3G).expect("failed to read STO-3G basis set");
    pub static ref BASIS_6_31G: BasisSet = serde_json::from_str(DATA_6_31G).expect("failed to read 6-31G basis set");
}
