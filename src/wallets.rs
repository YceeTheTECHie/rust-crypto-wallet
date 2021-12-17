use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};
use anyhow::Result;
use web3::{self,Web3,transports::Http, types::TransactionParameters};
use web3::types::{U256, H256, H160};

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

pub fn create_tx_object(to: H160, value : usize) ->  Result<TransactionParameters> {
    Ok(TransactionParameters {
        to : Some(to),
        value : U256::exp10(value),
        ..Default::default()
    })
}

pub async fn sign_tx(web3: Web3<Http>, tx_object : TransactionParameters,secKey:SecretKey)-> Result<(H256)>{
    let signed = web3.accounts().sign_transaction(tx_object,&secKey).await?;
    Ok(web3.eth().send_raw_transaction(signed.raw_transaction).await?)
}