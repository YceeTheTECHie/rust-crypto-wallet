use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};
use anyhow::Result;

pub fn create_keypair() -> Result<(SecretKey,PublicKey)> {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(6);
    Ok(secp.generate_keypair(&mut rng))
}
