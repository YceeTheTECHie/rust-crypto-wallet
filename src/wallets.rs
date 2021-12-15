use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};
use anyhow::Result;
use web3::{self,Web3,transports::Http};

pub fn create_keypair() -> Result<(SecretKey,PublicKey)> {
    let secp = secp256k1::Secp256k1::new();
    println!("{:?}", secp);
    let mut rng = rngs::StdRng::seed_from_u64(8);
    Ok(secp.generate_keypair(&mut rng))

}

pub fn establish_connection(url : &str) -> Result<Web3<Http>> {
    let transport = web3::transports::Http::new(url)?;
    // let transport = web3::Transport:Http::new(url)?;
    Ok(web3::Web3::new(transport))
}
